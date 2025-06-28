use bincode::{Decode, Encode};
use lib_codec::{decode, encode};
use tokio_tungstenite::tungstenite::{Bytes, Message};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum Command {
    SetMouse { x: i32, y: i32 },
    MoveMouse { x: i32, y: i32 },
    KeyCode(u16),
    InputText(String),
}

impl Into<Message> for &Command {
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
