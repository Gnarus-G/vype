use crate::sources::{AudioSource, SourceError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream};
use std::sync::{Arc, Mutex};

pub struct CpalAudioSource {
    device: Device,
    sample_rate: u32,
    channels: u16,
    stream: Option<Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
}

impl CpalAudioSource {
    pub fn new() -> Result<Self, SourceError> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| SourceError::Audio("No default input device found".to_string()))?;

        let supported_config = device
            .default_input_config()
            .map_err(|e| SourceError::Audio(format!("Failed to get default config: {}", e)))?;

        let sample_rate = supported_config.sample_rate().0;
        let channels = supported_config.channels();

        Ok(Self {
            device,
            sample_rate,
            channels,
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn with_device(device: Device) -> Result<Self, SourceError> {
        let supported_config = device
            .default_input_config()
            .map_err(|e| SourceError::Audio(format!("Failed to get default config: {}", e)))?;

        let sample_rate = supported_config.sample_rate().0;
        let channels = supported_config.channels();

        Ok(Self {
            device,
            sample_rate,
            channels,
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }
}

impl AudioSource for CpalAudioSource {
    fn start(&mut self) -> Result<(), SourceError> {
        self.buffer.lock().unwrap().clear();

        let config = cpal::StreamConfig {
            channels: self.channels,
            sample_rate: cpal::SampleRate(self.sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        let buffer = self.buffer.clone();
        let channels = self.channels as usize;

        let stream = self
            .device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut buf = buffer.lock().unwrap();
                    if channels == 1 {
                        buf.extend_from_slice(data);
                    } else {
                        for chunk in data.chunks(channels) {
                            let mono: f32 = chunk.iter().sum::<f32>() / channels as f32;
                            buf.push(mono);
                        }
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )
            .map_err(|e| SourceError::Audio(format!("Failed to build input stream: {}", e)))?;

        stream
            .play()
            .map_err(|e| SourceError::Audio(format!("Failed to start stream: {}", e)))?;

        self.stream = Some(stream);
        Ok(())
    }

    fn stop(&mut self) -> Vec<f32> {
        self.stream = None;
        std::mem::take(&mut self.buffer.lock().unwrap())
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn get_current_samples(&self) -> Vec<f32> {
        self.buffer.lock().unwrap().clone()
    }
}
