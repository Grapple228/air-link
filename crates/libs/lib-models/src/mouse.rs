use bincode::{Decode, Encode};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Encode, Decode)]
pub enum MouseButton {
    LEFT,
    RIGHT,
    MIDDLE,
    MOUSE4,
    MOUSE5,
}

impl MouseButton {
    pub fn from_linux_code(value: u32) -> Self {
        match value {
            272 => Self::LEFT,   // BTN_LEFT
            273 => Self::RIGHT,  // BTN_RIGHT
            274 => Self::MIDDLE, // BTN_MIDDLE
            275 => Self::MOUSE4, // BTN_BACK
            276 => Self::MOUSE5, // BTN_FORWARD
            _ => panic!("Unsupported mouse button code: {}", value),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Encode, Decode)]
pub enum MouseScroll {
    Vertical(i32),
    Horizontal(i32),
}
