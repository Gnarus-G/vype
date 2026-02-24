use crate::sources::{AudioSource, KeyboardSink, SourceError, Transcriber};
use std::sync::{Arc, Mutex};

pub struct FakeAudioSource {
    samples: Vec<f32>,
    sample_rate: u32,
    channels: u16,
    recording: bool,
}

impl FakeAudioSource {
    pub fn new(samples: Vec<f32>, sample_rate: u32, channels: u16) -> Self {
        Self {
            samples,
            sample_rate,
            channels,
            recording: false,
        }
    }

    pub fn is_recording(&self) -> bool {
        self.recording
    }
}

impl AudioSource for FakeAudioSource {
    fn start(&mut self) -> Result<(), SourceError> {
        self.recording = true;
        Ok(())
    }

    fn stop(&mut self) -> Vec<f32> {
        self.recording = false;
        std::mem::take(&mut self.samples)
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn get_current_samples(&self) -> Vec<f32> {
        self.samples.clone()
    }
}

#[derive(Clone)]
pub struct CaptureKeyboardSink {
    captured: Arc<Mutex<Vec<String>>>,
}

impl CaptureKeyboardSink {
    pub fn new() -> Self {
        Self {
            captured: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn captured(&self) -> Vec<String> {
        self.captured.lock().unwrap().clone()
    }
}

impl Default for CaptureKeyboardSink {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardSink for CaptureKeyboardSink {
    fn type_text(&mut self, text: &str) {
        self.captured.lock().unwrap().push(text.to_string());
    }
}

pub struct MockTranscriber {
    result: Result<String, String>,
}

impl MockTranscriber {
    pub fn new(text: String) -> Self {
        Self { result: Ok(text) }
    }

    pub fn with_error(error: &str) -> Self {
        Self {
            result: Err(error.to_string()),
        }
    }
}

impl Transcriber for MockTranscriber {
    fn transcribe(&self, _audio: &[f32]) -> Result<String, SourceError> {
        self.result.clone().map_err(SourceError::Transcription)
    }
}
