use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use rdev::{listen, Event, EventType, Key};

use vype::config::Config;
#[cfg(feature = "transcription")]
use vype::sources::AudioSource;
#[cfg(feature = "transcription")]
use vype::sources::CpalAudioSource;

#[cfg(feature = "transcription")]
use vype::sources::{KeyboardSink, Transcriber, WhisperTranscriber, XdoKeyboardSink};

#[cfg(feature = "transcription")]
use vype::model::get_model_path;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::parse();

    log::info!(
        "Vype started. Press and hold {} to record, release to transcribe.",
        config.key
    );
    log::info!("Press Ctrl+C to exit.");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let recording = Arc::new(AtomicBool::new(false));
    let ptt_key = parse_ptt_key(&config.key);

    let (tx, rx) = std::sync::mpsc::channel();
    let tx_clone = tx.clone();
    let recording_clone = recording.clone();
    let _model_opt = config.model.clone();
    let _language_opt = config.language.clone();

    #[cfg(feature = "transcription")]
    let mut sink = XdoKeyboardSink::new().expect("Failed to create xdo keyboard");

    std::thread::spawn(move || {
        #[cfg(feature = "transcription")]
        let model_path = get_model_path(model_opt.as_deref()).expect("Failed to get model");

        #[cfg(feature = "transcription")]
        let mut audio_source = CpalAudioSource::new().expect("Failed to create audio source");

        #[cfg(feature = "transcription")]
        let transcriber = WhisperTranscriber::new(model_path.to_str().unwrap(), &language_opt)
            .expect("Failed to create transcriber");

        while running.load(Ordering::SeqCst) {
            if let Ok(msg) = rx.recv_timeout(std::time::Duration::from_millis(50)) {
                match msg {
                    AppMsg::StartRecording => {
                        #[cfg(feature = "transcription")]
                        {
                            if let Err(e) = audio_source.start() {
                                log::error!("Error starting audio: {}", e);
                            } else {
                                log::info!("Recording started...");
                            }
                        }
                    }
                    AppMsg::StopRecording => {
                        #[cfg(feature = "transcription")]
                        {
                            let samples = audio_source.stop();
                            log::info!("Recording stopped. Samples: {}", samples.len());
                            if !samples.is_empty() {
                                match transcriber.transcribe(&samples) {
                                    Ok(text) => {
                                        log::info!("Transcribed: {}", text);
                                        sink.type_text(&text);
                                    }
                                    Err(e) => log::error!("Transcription error: {}", e),
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    log::info!("Starting keyboard listener...");

    let callback = move |event: Event| {
        let is_key_press = matches!(event.event_type, EventType::KeyPress(_));
        let is_key_release = matches!(event.event_type, EventType::KeyRelease(_));

        if is_key_press || is_key_release {
            if let EventType::KeyPress(key) | EventType::KeyRelease(key) = event.event_type {
                if key == ptt_key {
                    if is_key_press && !recording_clone.load(Ordering::SeqCst) {
                        recording_clone.store(true, Ordering::SeqCst);
                        let _ = tx_clone.send(AppMsg::StartRecording);
                        log::info!("PTT pressed");
                    } else if is_key_release && recording_clone.load(Ordering::SeqCst) {
                        recording_clone.store(false, Ordering::SeqCst);
                        let _ = tx_clone.send(AppMsg::StopRecording);
                        log::info!("PTT released");
                    }
                }
            }
        }
    };

    if let Err(error) = listen(callback) {
        log::error!("Listen error: {:?}", error);
    }

    log::info!("Keyboard listener exited");

    Ok(())
}

enum AppMsg {
    StartRecording,
    StopRecording,
}

fn parse_ptt_key(key: &str) -> Key {
    match key.to_uppercase().as_str() {
        "F1" => Key::F1,
        "F2" => Key::F2,
        "F3" => Key::F3,
        "F4" => Key::F4,
        "F5" => Key::F5,
        "F6" => Key::F6,
        "F7" => Key::F7,
        "F8" => Key::F8,
        "F9" => Key::F9,
        "F10" => Key::F10,
        "F11" => Key::F11,
        "F12" => Key::F12,
        _ => Key::F12,
    }
}
