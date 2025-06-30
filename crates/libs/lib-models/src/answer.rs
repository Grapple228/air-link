use bincode::{Decode, Encode};
use lib_codec::{decode, encode};
use tokio_tungstenite::tungstenite::{Bytes, Message};

use crate::{MouseButton, MouseScroll};

#[derive(Debug, Clone, Encode, Decode)]
pub enum Answer {
    ClipboardContents(String),
}

impl Into<Message> for &Answer {
    fn into(self) -> Message {
        let data = encode(&self).unwrap();
        Message::Binary(data.into())
    }
}

impl Into<Message> for Answer {
    fn into(self) -> Message {
        let data = encode(&self).unwrap();
        Message::Binary(data.into())
    }
}

impl From<Message> for Answer {
    fn from(value: Message) -> Self {
        decode(&value.into_data()).unwrap()
    }
}

impl From<Bytes> for Answer {
    fn from(value: Bytes) -> Self {
        decode(&value).unwrap()
    }
}
