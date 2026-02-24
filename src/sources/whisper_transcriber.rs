use crate::sources::{SourceError, Transcriber};

#[cfg(any(feature = "vulkan", feature = "cuda"))]
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperTranscriber {
    #[cfg(any(feature = "vulkan", feature = "cuda"))]
    ctx: WhisperContext,
    language: String,
}

impl WhisperTranscriber {
    #[cfg(any(feature = "vulkan", feature = "cuda"))]
    pub fn new(model_path: &str, language: &str) -> Result<Self, SourceError> {
        let ctx_params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(model_path, ctx_params)
            .map_err(|e| SourceError::Transcription(format!("Failed to load model: {}", e)))?;
        Ok(Self {
            ctx,
            language: language.to_string(),
        })
    }

    #[cfg(not(any(feature = "vulkan", feature = "cuda")))]
    pub fn new(_model_path: &str, _language: &str) -> Result<Self, SourceError> {
        Err(SourceError::Transcription(
            "GPU feature not enabled. Recompile with --features vulkan or --features cuda"
                .to_string(),
        ))
    }
}

impl Transcriber for WhisperTranscriber {
    #[cfg(any(feature = "vulkan", feature = "cuda"))]
    fn transcribe(&self, audio: &[f32]) -> Result<String, SourceError> {
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some(&self.language));
        params.set_translate(false);
        params.set_no_context(true);
        params.set_single_segment(true);

        let mut state = self
            .ctx
            .create_state()
            .map_err(|e| SourceError::Transcription(format!("Failed to create state: {}", e)))?;

        state
            .full(params, audio)
            .map_err(|e| SourceError::Transcription(format!("Transcription failed: {}", e)))?;

        let mut result = String::new();
        for segment in state.as_iter() {
            if let Ok(text) = segment.to_str() {
                result.push_str(text);
            }
        }

        Ok(result.trim().to_string())
    }

    #[cfg(not(any(feature = "vulkan", feature = "cuda")))]
    fn transcribe(&self, _audio: &[f32]) -> Result<String, SourceError> {
        Err(SourceError::Transcription(
            "GPU feature not enabled".to_string(),
        ))
    }
}
