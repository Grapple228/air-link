#![allow(unused)]

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

pub fn set_contents(content: impl Into<String>) -> Result<()> {
    let mut ctx = ClipboardContext::new().map_err(|_| Error::Init)?;

    ctx.set_contents(content.into()).map_err(|_| Error::Set)
}

pub fn get_contents() -> Result<String> {
    let mut ctx = ClipboardContext::new().map_err(|_| Error::Init)?;
    ctx.get_contents().map_err(|_| Error::Get)
}

#[derive(Debug, From)]
pub enum Error {
    Init,
    Get,
    Set,
}

// endregion: --- Error Boilerplate

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
