use anyhow::Result;
use hf_hub::api::sync::Api;
use std::path::PathBuf;

const DEFAULT_MODEL_SIZE: &str = "small";

pub fn get_model_path(custom_path: Option<&str>, model_size: Option<&str>) -> Result<PathBuf> {
    if let Some(path) = custom_path {
        return Ok(PathBuf::from(path));
    }

    let size = model_size.unwrap_or(DEFAULT_MODEL_SIZE);
    let model_filename = model_filename_for_size(size);

    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
        .join("vype");

    std::fs::create_dir_all(&config_dir)?;

    let model_path = config_dir.join(&model_filename);

    if !model_path.exists() {
        log::info!(
            "Downloading whisper model {} to {:?}...",
            model_filename,
            model_path
        );
        let api = Api::new()?;
        let repo = api.model("ggerganov/whisper.cpp".to_string());
        let downloaded = repo.get(&model_filename)?;
        std::fs::copy(downloaded, &model_path)?;
        log::info!("Model downloaded successfully.");
    }

    Ok(model_path)
}

fn model_filename_for_size(size: &str) -> String {
    match size {
        "tiny" => "ggml-tiny.en.bin".to_string(),
        "base" => "ggml-base.en.bin".to_string(),
        "small" => "ggml-small.en.bin".to_string(),
        "medium" => "ggml-medium.en.bin".to_string(),
        "large" => "ggml-large-v3.bin".to_string(),
        _ => "ggml-small.en.bin".to_string(),
    }
}
