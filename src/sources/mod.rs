use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
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
    fn get_current_samples(&self) -> Vec<f32>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyOp {
    Backspace(usize),
    Delete(usize),
    Type(char),
    Left(usize),
    Right(usize),
}

pub trait KeyboardSink {
    fn type_text(&mut self, text: &str);
    fn execute_ops(&mut self, ops: &[KeyOp]) {
        for op in ops {
            match op {
                KeyOp::Backspace(n) => {
                    for _ in 0..*n {
                        self.backspace();
                    }
                }
                KeyOp::Delete(n) => {
                    for _ in 0..*n {
                        self.delete();
                    }
                }
                KeyOp::Type(c) => {
                    self.type_text(&c.to_string());
                }
                KeyOp::Left(n) => {
                    for _ in 0..*n {
                        self.left();
                    }
                }
                KeyOp::Right(n) => {
                    for _ in 0..*n {
                        self.right();
                    }
                }
            }
        }
    }

    fn backspace(&mut self) {
        self.type_text("\x08");
    }

    fn delete(&mut self) {
        self.type_text("\x7F");
    }

    fn left(&mut self) {
        self.type_text("\x1B[D");
    }

    fn right(&mut self) {
        self.type_text("\x1B[C");
    }
}

pub trait Transcriber {
    fn transcribe(&self, audio: &[f32]) -> Result<String, SourceError>;
}

pub mod fakes;

#[cfg(not(test))]
pub mod cpal_audio;

#[cfg(not(test))]
pub mod xdo_keyboard;

#[cfg(all(any(feature = "cpu", feature = "vulkan", feature = "cuda"), not(test)))]
pub mod whisper_transcriber;

#[cfg(not(test))]
pub use cpal_audio::CpalAudioSource;

#[cfg(not(test))]
pub use xdo_keyboard::XdoKeyboardSink;

#[cfg(all(any(feature = "cpu", feature = "vulkan", feature = "cuda"), not(test)))]
pub use whisper_transcriber::WhisperTranscriber;
