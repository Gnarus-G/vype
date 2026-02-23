use vype::config::Config;

#[test]
fn config_has_default_values() {
    let config = Config::parse_from::<_, &str>([]);

    assert_eq!(config.key, "F13");
    assert_eq!(config.language, "en");
    assert_eq!(config.max_duration_secs, 30);
}

#[test]
fn config_parses_custom_values() {
    let config = Config::parse_from([
        "vype",
        "--key",
        "F14",
        "--language",
        "es",
        "--max-duration",
        "60",
    ]);

    assert_eq!(config.key, "F14");
    assert_eq!(config.language, "es");
    assert_eq!(config.max_duration_secs, 60);
}

#[test]
fn config_parses_short_flags() {
    let config = Config::parse_from(["vype", "-k", "F15", "-l", "de", "-d", "45"]);

    assert_eq!(config.key, "F15");
    assert_eq!(config.language, "de");
    assert_eq!(config.max_duration_secs, 45);
}

#[test]
fn config_model_path_default() {
    let config = Config::parse_from::<_, &str>([]);

    assert!(config.model.is_none());
}
