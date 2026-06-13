use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub enum Answer {
    ClipboardContents(String),
}
