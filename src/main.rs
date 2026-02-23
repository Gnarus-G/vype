use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use rdev::{listen, Event, EventType, Key};

use vype::config::Config;
use vype::sources::fakes::{FakeAudioSource, MockTranscriber};
use vype::sources::{AudioSource, Transcriber};

#[cfg(feature = "transcription")]
use vype::sources::WhisperTranscriber;

#[cfg(feature = "transcription")]
use vype::sources::RdevKeyboardSink;

#[cfg(feature = "transcription")]
use vype::sources::CpalAudioSource;

#[cfg(feature = "transcription")]
use vype::model::get_model_path;

fn main() -> Result<()> {
    let config = Config::parse();

    #[cfg(feature = "transcription")]
    let model_path = get_model_path(config.model.as_deref())?;

    #[cfg(feature = "transcription")]
    let audio_source = CpalAudioSource::new()?;

    #[cfg(not(feature = "transcription"))]
    let audio_source = FakeAudioSource::new(vec![0.0; 16000], 16000, 1);

    #[cfg(feature = "transcription")]
    let transcriber = WhisperTranscriber::new(model_path.to_str().unwrap(), &config.language)?;

    #[cfg(not(feature = "transcription"))]
    let transcriber = MockTranscriber::new("[transcription disabled]".to_string());

    println!(
        "Vype started. Press and hold {} to record, release to transcribe.",
        config.key
    );
    println!("Press Ctrl+C to exit.");

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

    std::thread::spawn(move || {
        let mut audio_source = audio_source;
        let transcriber = transcriber;

        while running.load(Ordering::SeqCst) {
            if let Ok(msg) = rx.recv_timeout(Duration::from_millis(50)) {
                match msg {
                    AppMsg::StartRecording => {
                        let _ = audio_source.start();
                    }
                    AppMsg::StopRecording => {
                        let samples = audio_source.stop();
                        if !samples.is_empty() {
                            match transcriber.transcribe(&samples) {
                                Ok(text) => {
                                    #[cfg(not(feature = "transcription"))]
                                    {
                                        println!("Transcribed: {}", text);
                                    }
                                    #[cfg(feature = "transcription")]
                                    {
                                        println!("Transcribed: {}", text);
                                        let sink = RdevKeyboardSink::new();
                                        sink.type_text(&text);
                                    }
                                }
                                Err(e) => eprintln!("Transcription error: {}", e),
                            }
                        }
                    }
                }
            }
        }
    });

    let callback = move |event: Event| match event.event_type {
        EventType::KeyPress(key) if key == ptt_key => {
            if !recording_clone.load(Ordering::SeqCst) {
                recording_clone.store(true, Ordering::SeqCst);
                let _ = tx_clone.send(AppMsg::StartRecording);
            }
        }
        EventType::KeyRelease(key) if key == ptt_key => {
            if recording_clone.load(Ordering::SeqCst) {
                recording_clone.store(false, Ordering::SeqCst);
                let _ = tx_clone.send(AppMsg::StopRecording);
            }
        }
        _ => {}
    };

    if let Err(e) = listen(callback) {
        eprintln!("Error listening for keyboard events: {:?}", e);
    }

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
