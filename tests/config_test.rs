use vype::config::Config;

#[test]
fn config_has_default_values() {
    let config = Config::parse_from::<_, &str>([]);

    assert_eq!(config.language, "en");
    assert_eq!(config.max_duration_secs, 30);
    assert_eq!(config.partial_interval_secs, 2.0);
}

#[test]
fn config_parses_custom_values() {
    let config = Config::parse_from(["vype", "--language", "es", "--max-duration", "60"]);

    assert_eq!(config.language, "es");
    assert_eq!(config.max_duration_secs, 60);
}

#[test]
fn config_parses_short_flags() {
    let config = Config::parse_from(["vype", "-l", "de", "-d", "45"]);

    assert_eq!(config.language, "de");
    assert_eq!(config.max_duration_secs, 45);
}

#[test]
fn config_model_path_default() {
    let config = Config::parse_from::<_, &str>([]);

    assert!(config.model.is_none());
}

#[test]
fn config_parses_partial_interval() {
    let config = Config::parse_from(["vype", "-p", "1.5"]);
    assert_eq!(config.partial_interval_secs, 1.5);

    let config = Config::parse_from(["vype", "--partial-interval", "3.0"]);
    assert_eq!(config.partial_interval_secs, 3.0);
}
