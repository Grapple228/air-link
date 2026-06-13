//! Main Crate Error

use derive_more::derive::From;

use crate::handler;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Config
    ConfigAlreadyInitialized,

    // -- Modules
    #[from]
    Handler(handler::Error),

    // -- Externals
    #[from]
    Codec(lib_codec::Error),
    #[from]
    Quic(lib_quic::Error),
    #[from]
    Envs(grapple_utils::envs::Error),

    #[from]
    Io(std::io::Error),
    NotFound, // as example
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
