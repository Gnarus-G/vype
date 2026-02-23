use anyhow::Result;
use hf_hub::api::sync::Api;
use std::path::PathBuf;

const DEFAULT_MODEL: &str = "ggml-base.en.bin";

pub fn get_model_path(custom_path: Option<&str>) -> Result<PathBuf> {
    if let Some(path) = custom_path {
        return Ok(PathBuf::from(path));
    }

    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
        .join("vype");

    std::fs::create_dir_all(&config_dir)?;

    let model_path = config_dir.join(DEFAULT_MODEL);

    if !model_path.exists() {
        println!("Downloading whisper model to {:?}...", model_path);
        let api = Api::new()?;
        let repo = api.model("ggerganov/whisper.cpp".to_string());
        let downloaded = repo.get(DEFAULT_MODEL)?;
        std::fs::copy(downloaded, &model_path)?;
        println!("Model downloaded successfully.");
    }

    Ok(model_path)
}
