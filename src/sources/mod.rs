use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SourceError {
    #[error("Audio source error: {0}")]
    Audio(String),

    #[error("Transcription error: {0}")]
    Transcription(String),
}

pub trait AudioSource {
    fn start(&mut self) -> Result<(), SourceError>;
    fn stop(&mut self) -> Vec<f32>;
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u16;
}

pub trait KeyboardSink {
    fn type_text(&mut self, text: &str);
}

pub trait Transcriber {
    fn transcribe(&self, audio: &[f32]) -> Result<String, SourceError>;
}

pub mod fakes;

#[cfg(not(test))]
pub mod cpal_audio;

#[cfg(not(test))]
pub mod xdo_keyboard;

#[cfg(all(feature = "transcription", not(test)))]
pub mod whisper_transcriber;

#[cfg(not(test))]
pub use cpal_audio::CpalAudioSource;

#[cfg(not(test))]
pub use xdo_keyboard::XdoKeyboardSink;

#[cfg(all(feature = "transcription", not(test)))]
pub use whisper_transcriber::WhisperTranscriber;
