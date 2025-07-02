use derive_more::From;
use lib_models::clipboard;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Clipboard(clipboard::Error),

    #[from]
    Enigo(enigo::InputError),
    #[from]
    EnigoConfig(enigo::NewConError),
    #[from]
    Tungstenite(tokio_tungstenite::tungstenite::Error),

    #[from]
    Io(std::io::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
