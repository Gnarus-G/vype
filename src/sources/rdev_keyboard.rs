use crate::pure::typer::text_to_key_events;
use crate::sources::KeyboardSink;

pub struct RdevKeyboardSink;

impl RdevKeyboardSink {
    pub fn new() -> Self {
        Self
    }
}

impl KeyboardSink for RdevKeyboardSink {
    fn type_text(&self, text: &str) {
        let events = text_to_key_events(text);
        for key_event in events {
            if key_event.shift {
                let _ = rdev::simulate(&rdev::EventType::KeyPress(rdev::Key::ShiftLeft));
            }
            let _ = rdev::simulate(&rdev::EventType::KeyPress(key_event.key));
            let _ = rdev::simulate(&rdev::EventType::KeyRelease(key_event.key));
            if key_event.shift {
                let _ = rdev::simulate(&rdev::EventType::KeyRelease(rdev::Key::ShiftLeft));
            }
        }
    }
}

impl Default for RdevKeyboardSink {
    fn default() -> Self {
        Self::new()
    }
}
