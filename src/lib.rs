#[cfg(all(feature = "cpu", any(feature = "cuda", feature = "vulkan")))]
compile_error!("Features `cpu` and GPU backends are mutually exclusive. Enable exactly one of: cpu, cuda, vulkan.");

#[cfg(all(feature = "cuda", feature = "vulkan"))]
compile_error!("Features `cuda` and `vulkan` are mutually exclusive. Enable exactly one of: cpu, cuda, vulkan.");

pub mod actors;
pub mod audio;
pub mod config;
pub mod keyboard;
pub mod model;
pub mod pure;
pub mod sources;
pub mod transcriber;
