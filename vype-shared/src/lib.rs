use iceoryx2::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ZeroCopySend)]
#[repr(C)]
pub enum PttEventType {
    StartRecording = 0,
    StopRecording = 1,
    PartialTranscribe = 2,
    ToggleRecording = 3,
}

#[derive(Debug, Clone, Copy, ZeroCopySend)]
#[repr(C)]
pub struct PttEvent {
    pub event_type: PttEventType,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyOp {
    Backspace(usize),
    Delete(usize),
    Type(char),
    Left(usize),
    Right(usize),
}

pub struct TypingState {
    typed: String,
}

impl TypingState {
    pub fn new() -> Self {
        Self {
            typed: String::new(),
        }
    }

    pub fn typed(&self) -> &str {
        &self.typed
    }

    pub fn transition(&mut self, new_text: &str) -> Vec<KeyOp> {
        let ops = edit_ops(&self.typed, new_text);
        self.typed = new_text.to_string();
        ops
    }

    pub fn clear(&mut self) {
        self.typed.clear();
    }
}

impl Default for TypingState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn edit_ops(previous: &str, current: &str) -> Vec<KeyOp> {
    if previous == current {
        return vec![];
    }

    if previous.is_empty() {
        return current.chars().map(KeyOp::Type).collect();
    }

    if current.is_empty() {
        return vec![KeyOp::Backspace(previous.chars().count())];
    }

    let prev_chars: Vec<char> = previous.chars().collect();
    let curr_chars: Vec<char> = current.chars().collect();

    let common_prefix_len = prev_chars
        .iter()
        .zip(curr_chars.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let mut ops = Vec::new();

    let backspace_count = prev_chars.len() - common_prefix_len;
    if backspace_count > 0 {
        ops.push(KeyOp::Backspace(backspace_count));
    }

    for c in curr_chars[common_prefix_len..].iter() {
        ops.push(KeyOp::Type(*c));
    }

    ops
}

#[derive(Debug, Clone)]
pub struct PttConfig {
    pub key: String,
    pub max_duration: u64,
    pub partial_interval: f64,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub model: Option<String>,
    pub model_size: String,
    pub language: String,
    pub ptt: PttConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model: None,
            model_size: "medium".to_string(),
            language: "en".to_string(),
            ptt: PttConfig {
                key: "F9".to_string(),
                max_duration: 30,
                partial_interval: 2.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typing_state() {
        let mut state = TypingState::new();

        let ops = state.transition("hello");
        assert_eq!(state.typed(), "hello");
        assert!(!ops.is_empty());

        let ops = state.transition("hello world");
        assert_eq!(state.typed(), "hello world");
        assert!(!ops.is_empty());

        state.clear();
        assert_eq!(state.typed(), "");
    }

    #[test]
    fn test_edit_ops() {
        let ops = edit_ops("", "hello");
        assert_eq!(ops.len(), 5);

        let ops = edit_ops("hello", "hello world");
        assert_eq!(ops.len(), 6); // " world"

        let ops = edit_ops("hello world", "");
        assert_eq!(ops, vec![KeyOp::Backspace(11)]);
    }
}
