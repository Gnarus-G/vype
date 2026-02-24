use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "vype", about = "Speech-to-text with keyboard output")]
pub struct Config {
    #[arg(short = 'm', long, value_name = "PATH")]
    pub model: Option<String>,

    #[arg(short = 's', long, value_name = "SIZE", value_parser = ["tiny", "base", "small", "medium", "large"])]
    pub model_size: Option<String>,

    #[arg(short = 'k', long, default_value = "F12", value_name = "KEY")]
    pub key: String,

    #[arg(short = 'l', long, default_value = "en", value_name = "LANG")]
    pub language: String,

    #[arg(
        short = 'd',
        long = "max-duration",
        default_value = "30",
        value_name = "SEC"
    )]
    pub max_duration_secs: u64,
}

impl Config {
    pub fn parse_from<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        <Self as Parser>::parse_from(args)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: None,
            model_size: None,
            key: "F12".to_string(),
            language: "en".to_string(),
            max_duration_secs: 30,
        }
    }
}
