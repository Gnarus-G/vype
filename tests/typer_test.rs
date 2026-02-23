use vype::pure::typer::text_to_key_events;

#[test]
fn lowercase_char_maps_to_key() {
    let events = text_to_key_events("a");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].key, rdev::Key::KeyA);
    assert!(!events[0].shift);
}

#[test]
fn different_lowercase_chars_map_correctly() {
    let tests = [
        ('b', rdev::Key::KeyB),
        ('z', rdev::Key::KeyZ),
        ('0', rdev::Key::Num0),
        ('9', rdev::Key::Num9),
    ];

    for (char, expected_key) in tests {
        let events = text_to_key_events(&char.to_string());
        assert_eq!(events.len(), 1, "char '{}'", char);
        assert_eq!(events[0].key, expected_key, "char '{}'", char);
        assert!(!events[0].shift, "char '{}'", char);
    }
}

#[test]
fn uppercase_char_requires_shift() {
    let events = text_to_key_events("A");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].key, rdev::Key::KeyA);
    assert!(events[0].shift);
}

#[test]
fn uppercase_letters_require_shift() {
    let tests = [
        ('H', rdev::Key::KeyH),
        ('E', rdev::Key::KeyE),
        ('L', rdev::Key::KeyL),
        ('O', rdev::Key::KeyO),
    ];

    for (char, expected_key) in tests {
        let events = text_to_key_events(&char.to_string());
        assert_eq!(events.len(), 1, "char '{}'", char);
        assert_eq!(events[0].key, expected_key, "char '{}'", char);
        assert!(events[0].shift, "char '{}' should require shift", char);
    }
}

#[test]
fn space_maps_to_space_key() {
    let events = text_to_key_events(" ");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].key, rdev::Key::Space);
    assert!(!events[0].shift);
}

#[test]
fn punctuation_maps_correctly() {
    let tests: [(char, rdev::Key, bool); 8] = [
        ('.', rdev::Key::Dot, false),
        (',', rdev::Key::Comma, false),
        ('!', rdev::Key::Num1, true),
        ('?', rdev::Key::Slash, true),
        ('-', rdev::Key::Minus, false),
        ('_', rdev::Key::Minus, true),
        ('(', rdev::Key::Num9, true),
        (')', rdev::Key::Num0, true),
    ];

    for (char, expected_key, expected_shift) in tests {
        let events = text_to_key_events(&char.to_string());
        assert_eq!(events.len(), 1, "char '{}'", char);
        assert_eq!(events[0].key, expected_key, "char '{}'", char);
        assert_eq!(events[0].shift, expected_shift, "char '{}' shift", char);
    }
}

#[test]
fn full_sentence_hello_world() {
    let events = text_to_key_events("Hello, world!");

    assert!(events.len() > 0);

    assert_eq!(events[0].key, rdev::Key::KeyH);
    assert!(events[0].shift);

    assert_eq!(events[1].key, rdev::Key::KeyE);
    assert!(!events[1].shift);

    assert_eq!(events[2].key, rdev::Key::KeyL);
    assert!(!events[2].shift);

    let comma_idx = 5;
    assert_eq!(events[comma_idx].key, rdev::Key::Comma);
    assert!(!events[comma_idx].shift);

    let space_idx = 6;
    assert_eq!(events[space_idx].key, rdev::Key::Space);

    let exclamation_idx = events.len() - 1;
    assert_eq!(events[exclamation_idx].key, rdev::Key::Num1);
    assert!(events[exclamation_idx].shift);
}
