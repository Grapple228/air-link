use bincode::{Decode, Encode};

use crate::{MouseButton, MouseScroll};

#[derive(Debug, Clone, Encode, Decode)]
pub enum Command {
    SetMouse { x: i32, y: i32 },
    MoveMouse { x: i32, y: i32 },
    KeyPressed(u32),
    KeyReleased(u32),
    InputText(String),
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    MouseScroll(MouseScroll),
    SetClipboard(String),
}
