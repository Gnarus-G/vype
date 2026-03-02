use anyhow::Result;
use clap::Parser;
use iceoryx2::prelude::*;
use libxdo::XDo;
use log::{error, info};
use rdev::{listen, EventType, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use vype_shared::{AppConfig, PttConfig, PttEvent, PttEventType};

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
use vype_shared::{KeyOp, TypingState};

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
use vype::{
    model::get_model_path,
    pure::resample::resample_to_16khz_mono,
    sources::{AudioSource, CpalAudioSource, Transcriber, WhisperTranscriber},
};

#[derive(Parser, Debug)]
#[command(name = "vyped")]
#[command(about = "Vype daemon - Audio capture, transcription, and typing")]
struct Args {
    #[arg(short = 'm', long = "model", value_name = "PATH")]
    model: Option<String>,

    #[arg(short = 's', long = "model-size", default_value = "medium", value_name = "SIZE", value_parser = ["tiny", "base", "small", "medium", "large"])]
    model_size: String,

    #[arg(
        short = 'l',
        long = "language",
        default_value = "en",
        value_name = "LANG"
    )]
    language: String,

    #[arg(short = 'k', long = "key", default_value = "F9", value_name = "KEY")]
    key: String,

    #[arg(
        short = 'd',
        long = "max-duration",
        default_value = "30",
        value_name = "SEC"
    )]
    max_duration: u64,

    #[arg(
        short = 'p',
        long = "partial-interval",
        default_value = "2.0",
        value_name = "SECS"
    )]
    partial_interval: f64,

    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Debug, Clone, Copy)]
enum ControlMsg {
    Start,
    Stop,
    Partial,
    Toggle,
}

impl From<PttEventType> for ControlMsg {
    fn from(value: PttEventType) -> Self {
        match value {
            PttEventType::StartRecording => ControlMsg::Start,
            PttEventType::StopRecording => ControlMsg::Stop,
            PttEventType::PartialTranscribe => ControlMsg::Partial,
            PttEventType::ToggleRecording => ControlMsg::Toggle,
        }
    }
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
        _ => Key::F9,
    }
}

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
fn execute_key_op(xdo: &XDo, op: &KeyOp) -> Result<()> {
    match op {
        KeyOp::Backspace(n) => {
            for _ in 0..*n {
                xdo.send_keysequence("BackSpace", 0)?;
            }
        }
        KeyOp::Delete(n) => {
            for _ in 0..*n {
                xdo.send_keysequence("Delete", 0)?;
            }
        }
        KeyOp::Type(c) => {
            xdo.enter_text(&c.to_string(), 0)?;
        }
        KeyOp::Left(n) => {
            for _ in 0..*n {
                xdo.send_keysequence("Left", 0)?;
            }
        }
        KeyOp::Right(n) => {
            for _ in 0..*n {
                xdo.send_keysequence("Right", 0)?;
            }
        }
    }
    Ok(())
}

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
fn execute_ops(xdo: &XDo, ops: &[KeyOp]) -> Result<()> {
    for op in ops {
        execute_key_op(xdo, op)?;
    }
    Ok(())
}

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
fn process_stop(
    typing_state: &mut TypingState,
    audio_source: &mut CpalAudioSource,
    transcriber: &WhisperTranscriber,
    xdo: &XDo,
) -> Result<()> {
    let samples = audio_source.stop();
    info!("Recording stopped. Samples: {}", samples.len());

    if samples.is_empty() {
        typing_state.clear();
        return Ok(());
    }

    let resampled = resample_to_16khz_mono(
        &samples,
        audio_source.sample_rate(),
        audio_source.channels(),
    );
    info!(
        "Resampled from {}Hz to 16kHz: {} -> {} samples",
        audio_source.sample_rate(),
        samples.len(),
        resampled.len()
    );

    match transcriber.transcribe(&resampled) {
        Ok(text) => {
            info!("Transcribed: {}", text);
            if !text.is_empty() {
                let ops = typing_state.transition(&text);
                execute_ops(xdo, &ops)?;
            }
        }
        Err(e) => {
            error!("Transcription error: {}", e);
            typing_state.clear();
        }
    }

    Ok(())
}

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
fn process_partial(
    typing_state: &mut TypingState,
    audio_source: &mut CpalAudioSource,
    transcriber: &WhisperTranscriber,
    xdo: &XDo,
) -> Result<()> {
    let samples = audio_source.get_current_samples();
    if samples.is_empty() {
        return Ok(());
    }

    let resampled = resample_to_16khz_mono(
        &samples,
        audio_source.sample_rate(),
        audio_source.channels(),
    );

    match transcriber.transcribe(&resampled) {
        Ok(text) => {
            if !text.is_empty() {
                info!("Partial transcription: {}", text);
                let ops = typing_state.transition(&text);
                execute_ops(xdo, &ops)?;
            }
        }
        Err(e) => error!("Partial transcription error: {}", e),
    }

    Ok(())
}

#[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
fn handle_control_msg(
    msg: ControlMsg,
    is_recording: &Arc<AtomicBool>,
    typing_state: &mut TypingState,
    audio_source: &mut CpalAudioSource,
    transcriber: &WhisperTranscriber,
    xdo: &XDo,
) -> Result<()> {
    match msg {
        ControlMsg::Start => {
            if !is_recording.load(Ordering::SeqCst) {
                is_recording.store(true, Ordering::SeqCst);
                typing_state.clear();
                audio_source.start()?;
                info!("Recording started");
            }
        }
        ControlMsg::Stop => {
            if is_recording.load(Ordering::SeqCst) {
                is_recording.store(false, Ordering::SeqCst);
                process_stop(typing_state, audio_source, transcriber, xdo)?;
            }
        }
        ControlMsg::Partial => {
            if is_recording.load(Ordering::SeqCst) {
                process_partial(typing_state, audio_source, transcriber, xdo)?;
            }
        }
        ControlMsg::Toggle => {
            if is_recording.load(Ordering::SeqCst) {
                is_recording.store(false, Ordering::SeqCst);
                process_stop(typing_state, audio_source, transcriber, xdo)?;
            } else {
                is_recording.store(true, Ordering::SeqCst);
                typing_state.clear();
                audio_source.start()?;
                info!("Recording started (toggle)");
            }
        }
    }

    Ok(())
}

#[cfg(not(any(feature = "cpu", feature = "vulkan", feature = "cuda")))]
fn handle_control_msg(msg: ControlMsg, is_recording: &Arc<AtomicBool>) {
    match msg {
        ControlMsg::Start => {
            is_recording.store(true, Ordering::SeqCst);
            info!("Start requested (no transcription support)");
        }
        ControlMsg::Stop => {
            is_recording.store(false, Ordering::SeqCst);
            info!("Stop requested (no transcription support)");
        }
        ControlMsg::Partial => {
            info!("Partial requested (no transcription support)");
        }
        ControlMsg::Toggle => {
            let next = !is_recording.load(Ordering::SeqCst);
            is_recording.store(next, Ordering::SeqCst);
            info!("Toggle requested (no transcription support): {}", next);
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::Builder::from_default_env()
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    let config = AppConfig {
        model: args.model,
        model_size: args.model_size,
        language: args.language,
        ptt: PttConfig {
            key: args.key,
            max_duration: args.max_duration,
            partial_interval: args.partial_interval,
        },
    };

    info!(
        "Starting vyped daemon with model size {} and PTT key {}",
        config.model_size, config.ptt.key
    );

    let running = Arc::new(AtomicBool::new(true));
    let is_recording = Arc::new(AtomicBool::new(false));

    let running_for_ctrlc = running.clone();
    ctrlc::set_handler(move || {
        running_for_ctrlc.store(false, Ordering::SeqCst);
    })?;

    #[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
    let (mut audio_source, transcriber, xdo) = {
        let model_path = get_model_path(config.model.as_deref(), Some(&config.model_size))?;
        let model_path_str = model_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("model path contains invalid UTF-8"))?;
        let audio_source = CpalAudioSource::new()?;
        let transcriber = WhisperTranscriber::new(model_path_str, &config.language)?;
        let xdo = XDo::new(None)?;
        (audio_source, transcriber, xdo)
    };

    #[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
    let mut typing_state = TypingState::new();

    let node = NodeBuilder::new()
        .name(&"vyped".try_into()?)
        .create::<ipc::Service>()?;

    let ptt_service = node
        .service_builder(&"vype/ptt_events".try_into()?)
        .publish_subscribe::<PttEvent>()
        .max_publishers(8)
        .max_subscribers(4)
        .open_or_create()?;

    let ptt_subscriber = ptt_service.subscriber_builder().create()?;

    let (control_tx, control_rx) = std::sync::mpsc::channel::<ControlMsg>();

    let key = parse_ptt_key(&config.ptt.key);
    let key_tx = control_tx.clone();
    std::thread::spawn(move || {
        let key_down = Arc::new(AtomicBool::new(false));
        let key_down_cb = key_down.clone();
        if let Err(e) = listen(move |event| match event.event_type {
            EventType::KeyPress(k) if k == key => {
                if !key_down_cb.swap(true, Ordering::SeqCst) {
                    let _ = key_tx.send(ControlMsg::Start);
                }
            }
            EventType::KeyRelease(k) if k == key => {
                if key_down_cb.swap(false, Ordering::SeqCst) {
                    let _ = key_tx.send(ControlMsg::Stop);
                }
            }
            _ => {}
        }) {
            error!("Keyboard listener failed: {:?}", e);
        }
    });

    let timer_tx = control_tx.clone();
    let recording_for_timer = is_recording.clone();
    let running_for_timer = running.clone();
    let partial_interval = Duration::from_secs_f64(config.ptt.partial_interval);
    std::thread::spawn(move || {
        while running_for_timer.load(Ordering::SeqCst) {
            std::thread::sleep(partial_interval);
            if recording_for_timer.load(Ordering::SeqCst) {
                let _ = timer_tx.send(ControlMsg::Partial);
            }
        }
    });

    info!("Listening for PTT key and IPC control events...");

    while running.load(Ordering::SeqCst) {
        while let Some(sample) = ptt_subscriber.receive()? {
            let msg: ControlMsg = sample.event_type.into();
            let _ = control_tx.send(msg);
        }

        while let Ok(msg) = control_rx.try_recv() {
            #[cfg(any(feature = "cpu", feature = "vulkan", feature = "cuda"))]
            {
                if let Err(e) = handle_control_msg(
                    msg,
                    &is_recording,
                    &mut typing_state,
                    &mut audio_source,
                    &transcriber,
                    &xdo,
                ) {
                    error!("Failed to process control message: {}", e);
                }
            }

            #[cfg(not(any(feature = "cpu", feature = "vulkan", feature = "cuda")))]
            {
                handle_control_msg(msg, &is_recording);
            }
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    info!("Shutting down vyped daemon");
    Ok(())
}
