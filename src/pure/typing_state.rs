use crate::pure::edit_distance::{edit_ops, KeyOp};

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
