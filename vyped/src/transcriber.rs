use anyhow::Result;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperTranscriber {
    ctx: WhisperContext,
    language: String,
}

impl WhisperTranscriber {
    pub fn new(model_path: &str, language: &str) -> Result<Self> {
        let ctx_params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(model_path, ctx_params)
            .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;
        Ok(Self {
            ctx,
            language: language.to_string(),
        })
    }

    pub fn transcribe(&self, audio: &[f32]) -> Result<String> {
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some(&self.language));
        params.set_translate(false);
        params.set_no_context(true);
        params.set_single_segment(true);

        let mut state = self
            .ctx
            .create_state()
            .map_err(|e| anyhow::anyhow!("Failed to create state: {}", e))?;

        state
            .full(params, audio)
            .map_err(|e| anyhow::anyhow!("Transcription failed: {}", e))?;

        let mut result = String::new();
        for segment in state.as_iter() {
            if let Ok(text) = segment.to_str() {
                result.push_str(text);
            }
        }

        Ok(result.trim().to_string())
    }
}
