use crate::sources::KeyboardSink;
use libxdo::XDo;

pub struct XdoKeyboardSink {
    xdo: XDo,
}

impl XdoKeyboardSink {
    pub fn new() -> Result<Self, libxdo::CreationError> {
        Ok(Self {
            xdo: XDo::new(None)?,
        })
    }
}

impl KeyboardSink for XdoKeyboardSink {
    fn type_text(&mut self, text: &str) {
        let _ = self.xdo.enter_text(text, 5000);
    }
}

impl Default for XdoKeyboardSink {
    fn default() -> Self {
        Self::new().expect("Failed to create xdo keyboard")
    }
}
