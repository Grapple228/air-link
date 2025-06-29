use bincode::{Decode, Encode};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Encode, Decode)]
pub enum MouseButton {
    LEFT,
    RIGHT,
    MIDDLE,
}

impl From<u32> for MouseButton {
    fn from(value: u32) -> Self {
        match value {
            272 => Self::LEFT,
            273 => Self::RIGHT,
            274 => Self::MIDDLE,
            _ => panic!("Unsupported mouse keycode"),
        }
    }
}
