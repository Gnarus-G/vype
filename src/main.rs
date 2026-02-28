use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use env_logger::Env;

use vype::config::Config;

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
use vype::pure::typing_state::TypingState;

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
use vype::sources::AudioSource;
#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
use vype::sources::CpalAudioSource;

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
use vype::sources::{KeyboardSink, Transcriber, WhisperTranscriber, XdoKeyboardSink};

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
use vype::model::get_model_path;

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
use vype::pure::resample::resample_to_16khz_mono;

#[cfg(any(
    feature = "cpu",
    feature = "vulkan",
    feature = "cuda",
    feature = "dbus"
))]
use vype::sources::dbus_service::{DbusMsg, Dbusservice};

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::parse();

    log::info!("Vype started. Use D-Bus to control recording.");
    log::info!(
        "Partial transcription interval: {}s",
        config.partial_interval_secs
    );
    log::info!("Press Ctrl+C to exit.");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    })?;

    let recording = Arc::new(AtomicBool::new(false));
    let partial_interval = Duration::from_secs_f64(config.partial_interval_secs);

    let (tx, rx) = std::sync::mpsc::channel();
    let recording_for_timer = recording.clone();
    let tx_for_timer = tx.clone();
    let running_for_audio = running.clone();
    let running_for_timer = running.clone();

    #[cfg(any(
        feature = "cpu",
        feature = "vulkan",
        feature = "cuda",
        feature = "dbus"
    ))]
    let (model_opt, model_size_opt, language_opt) = (
        config.model.clone(),
        config.model_size.clone(),
        config.language.clone(),
    );

    #[cfg(any(
        feature = "cpu",
        feature = "vulkan",
        feature = "cuda",
        feature = "dbus"
    ))]
    let (dbus_tx, dbus_rx) = std::sync::mpsc::channel();

    #[cfg(any(
        feature = "cpu",
        feature = "vulkan",
        feature = "cuda",
        feature = "dbus"
    ))]
    let dbus_running = running.clone();
    #[cfg(any(
        feature = "cpu",
        feature = "vulkan",
        feature = "cuda",
        feature = "dbus"
    ))]
    let dbus_recording = recording.clone();
    #[cfg(any(
        feature = "cpu",
        feature = "vulkan",
        feature = "cuda",
        feature = "dbus"
    ))]
    let dbus_tx_clone = tx.clone();

    #[cfg(any(
        feature = "cpu",
        feature = "vulkan",
        feature = "cuda",
        feature = "dbus"
    ))]
    {
        let dbus_service = Dbusservice::new(dbus_tx, dbus_recording.clone());
        if let Err(e) = dbus_service.run() {
            log::error!("Failed to start D-Bus service: {}", e);
        } else {
            log::info!("D-Bus service started. Access via: busctl --user call tech.bytin.vype /tech/bytin/vype tech.bytin.vype.Recorder ToggleRecording");
        }

        std::thread::spawn(move || {
            while dbus_running.load(Ordering::SeqCst) {
                if let Ok(msg) = dbus_rx.recv_timeout(Duration::from_millis(50)) {
                    match msg {
                        DbusMsg::StartRecording => {
                            if !dbus_recording.load(Ordering::SeqCst) {
                                dbus_recording.store(true, Ordering::SeqCst);
                                let _ = dbus_tx_clone.send(AppMsg::StartRecording);
                                log::info!("D-Bus: Recording started");
                                send_notification("🎤 Listening...");
                            }
                        }
                        DbusMsg::StopRecording => {
                            if dbus_recording.load(Ordering::SeqCst) {
                                dbus_recording.store(false, Ordering::SeqCst);
                                let _ = dbus_tx_clone.send(AppMsg::StopRecording);
                                log::info!("D-Bus: Recording stopped");
                                send_notification("⏹️ Transcribing...");
                            }
                        }
                        DbusMsg::ToggleRecording => {
                            let currently_recording = dbus_recording.load(Ordering::SeqCst);
                            dbus_recording.store(!currently_recording, Ordering::SeqCst);
                            let _ = dbus_tx_clone.send(if currently_recording {
                                AppMsg::StopRecording
                            } else {
                                AppMsg::StartRecording
                            });
                            log::info!("D-Bus: Recording toggled to {}", !currently_recording);
                            if !currently_recording {
                                send_notification("🎤 Listening...");
                            } else {
                                send_notification("⏹️ Transcribing...");
                            }
                        }
                    }
                }
            }
        });
    }

    std::thread::spawn(move || {
        #[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
        {
            let mut sink = XdoKeyboardSink::new().expect("Failed to create xdo keyboard");

            let model_path = get_model_path(model_opt.as_deref(), model_size_opt.as_deref())
                .expect("Failed to get model");

            let mut audio_source = CpalAudioSource::new().expect("Failed to create audio source");

            let transcriber = WhisperTranscriber::new(model_path.to_str().unwrap(), &language_opt)
                .expect("Failed to create transcriber");

            let mut typing_state = TypingState::new();

            while running_for_audio.load(Ordering::SeqCst) {
                if let Ok(msg) = rx.recv_timeout(Duration::from_millis(50)) {
                    match msg {
                        AppMsg::StartRecording => {
                            typing_state.clear();
                            if let Err(e) = audio_source.start() {
                                log::error!("Error starting audio: {}", e);
                            } else {
                                log::info!("Recording started...");
                            }
                        }
                        AppMsg::StopRecording => {
                            let samples = audio_source.stop();
                            log::info!("Recording stopped. Samples: {}", samples.len());
                            if !samples.is_empty() {
                                let resampled = resample_to_16khz_mono(
                                    &samples,
                                    audio_source.sample_rate(),
                                    audio_source.channels(),
                                );
                                log::info!(
                                    "Resampled from {}Hz to 16kHz: {} -> {} samples",
                                    audio_source.sample_rate(),
                                    samples.len(),
                                    resampled.len()
                                );
                                match transcriber.transcribe(&resampled) {
                                    Ok(text) => {
                                        log::info!("Transcribed {} chars", text.len());
                                        if !text.is_empty() {
                                            let ops = typing_state.transition(&text);
                                            sink.execute_ops(&ops);
                                            send_notification("Typed text");
                                        }
                                    }
                                    Err(e) => log::error!("Transcription error: {}", e),
                                }
                            } else {
                                typing_state.clear();
                            }
                        }
                        AppMsg::PartialTranscribe => {
                            let samples = audio_source.get_current_samples();
                            if !samples.is_empty() {
                                let resampled = resample_to_16khz_mono(
                                    &samples,
                                    audio_source.sample_rate(),
                                    audio_source.channels(),
                                );
                                match transcriber.transcribe(&resampled) {
                                    Ok(text) => {
                                        if !text.is_empty() {
                                            log::debug!("Partial: {}", text);
                                            let ops = typing_state.transition(&text);
                                            sink.execute_ops(&ops);
                                        }
                                    }
                                    Err(e) => log::error!("Partial transcription error: {}", e),
                                }
                            }
                        }
                    }
                }
            }
        }

        #[cfg(feature = "dbus")]
        {
            while running_for_audio.load(Ordering::SeqCst) {
                if let Ok(msg) = rx.recv_timeout(Duration::from_millis(50)) {
                    match msg {
                        AppMsg::StartRecording => {
                            log::info!("Recording started (D-Bus mode)...");
                        }
                        AppMsg::StopRecording => {
                            log::info!("Recording stopped (D-Bus mode)...");
                        }
                        AppMsg::PartialTranscribe => {}
                    }
                }
            }
        }

        #[cfg(not(any(
            feature = "cpu",
            feature = "vulkan",
            feature = "cuda",
            feature = "dbus"
        )))]
        {
            while running_for_audio.load(Ordering::SeqCst) {
                let _ = rx.recv_timeout(Duration::from_millis(50));
            }
        }
    });

    std::thread::spawn(move || {
        while running_for_timer.load(Ordering::SeqCst) {
            std::thread::sleep(partial_interval);
            if recording_for_timer.load(Ordering::SeqCst) {
                let _ = tx_for_timer.send(AppMsg::PartialTranscribe);
            }
        }
    });

    // Keep the main thread alive to handle D-Bus messages
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

enum AppMsg {
    StartRecording,
    StopRecording,
    PartialTranscribe,
}

fn send_notification(body: &str) {
    use std::process::Command;
    log::info!("Sending notification: {}", body);
    let _ = Command::new("notify-send")
        .args([
            "-u",
            "low",
            "-t",
            "1000",
            "-a",
            "tech.bytin.vype",
            "vype",
            body,
        ])
        .spawn();
}
