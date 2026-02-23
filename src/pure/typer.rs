use rdev::Key;

pub struct KeyEvent {
    pub key: Key,
    pub shift: bool,
}

pub fn text_to_key_events(text: &str) -> Vec<KeyEvent> {
    text.chars().filter_map(|c| char_to_key(c)).collect()
}

fn char_to_key(c: char) -> Option<KeyEvent> {
    let (key, shift) = match c {
        'a'..='z' => (char_to_key_letter(c), false),
        'A'..='Z' => (char_to_key_letter(c.to_ascii_lowercase()), true),
        '0' => (Key::Num0, false),
        '1' => (Key::Num1, false),
        '2' => (Key::Num2, false),
        '3' => (Key::Num3, false),
        '4' => (Key::Num4, false),
        '5' => (Key::Num5, false),
        '6' => (Key::Num6, false),
        '7' => (Key::Num7, false),
        '8' => (Key::Num8, false),
        '9' => (Key::Num9, false),
        ' ' => (Key::Space, false),
        '.' => (Key::Dot, false),
        ',' => (Key::Comma, false),
        '!' => (Key::Num1, true),
        '?' => (Key::Slash, true),
        '\'' => (Key::Quote, false),
        '"' => (Key::Quote, true),
        '-' => (Key::Minus, false),
        '_' => (Key::Minus, true),
        ':' => (Key::SemiColon, true),
        ';' => (Key::SemiColon, false),
        '(' => (Key::Num9, true),
        ')' => (Key::Num0, true),
        '/' => (Key::Slash, false),
        '@' => (Key::Num2, true),
        '#' => (Key::Num3, true),
        '$' => (Key::Num4, true),
        '%' => (Key::Num5, true),
        '^' => (Key::Num6, true),
        '&' => (Key::Num7, true),
        '*' => (Key::Num8, true),
        '+' => (Key::Equal, true),
        '=' => (Key::Equal, false),
        '[' => (Key::LeftBracket, false),
        ']' => (Key::RightBracket, false),
        '{' => (Key::LeftBracket, true),
        '}' => (Key::RightBracket, true),
        '\\' => (Key::BackSlash, false),
        '|' => (Key::BackSlash, true),
        '`' => (Key::BackQuote, false),
        '~' => (Key::BackQuote, true),
        '<' => (Key::Comma, true),
        '>' => (Key::Dot, true),
        '\n' => (Key::Return, false),
        '\t' => (Key::Tab, false),
        _ => return None,
    };
    Some(KeyEvent { key, shift })
}

fn char_to_key_letter(c: char) -> Key {
    match c {
        'a' => Key::KeyA,
        'b' => Key::KeyB,
        'c' => Key::KeyC,
        'd' => Key::KeyD,
        'e' => Key::KeyE,
        'f' => Key::KeyF,
        'g' => Key::KeyG,
        'h' => Key::KeyH,
        'i' => Key::KeyI,
        'j' => Key::KeyJ,
        'k' => Key::KeyK,
        'l' => Key::KeyL,
        'm' => Key::KeyM,
        'n' => Key::KeyN,
        'o' => Key::KeyO,
        'p' => Key::KeyP,
        'q' => Key::KeyQ,
        'r' => Key::KeyR,
        's' => Key::KeyS,
        't' => Key::KeyT,
        'u' => Key::KeyU,
        'v' => Key::KeyV,
        'w' => Key::KeyW,
        'x' => Key::KeyX,
        'y' => Key::KeyY,
        'z' => Key::KeyZ,
        _ => Key::Unknown(0),
    }
}
