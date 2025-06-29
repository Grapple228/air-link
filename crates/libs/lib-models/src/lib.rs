mod keyboard;
mod mouse;

use bincode::{Decode, Encode};
use lib_codec::{decode, encode};
use tokio_tungstenite::tungstenite::{Bytes, Message};

pub use keyboard::KeyboardButton;
pub use mouse::{MouseButton, MouseScroll};

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
}

impl Into<Message> for &Command {
    fn into(self) -> Message {
        let data = encode(&self).unwrap();
        Message::Binary(data.into())
    }
}

impl Into<Message> for Command {
    fn into(self) -> Message {
        let data = encode(&self).unwrap();
        Message::Binary(data.into())
    }
}

impl From<Message> for Command {
    fn from(value: Message) -> Self {
        decode(&value.into_data()).unwrap()
    }
}

impl From<Bytes> for Command {
    fn from(value: Bytes) -> Self {
        decode(&value).unwrap()
    }
}
