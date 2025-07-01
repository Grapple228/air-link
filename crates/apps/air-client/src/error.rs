use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Tungstenite(tokio_tungstenite::tungstenite::Error),

    #[from]
    #[cfg(target_os = "linux")]
    X11(crate::x11::Error),

    #[from]
    #[cfg(target_os = "linux")]
    Wayland(crate::wayland::Error),

    #[from]
    #[cfg(target_os = "windows")]
    Windows(crate::windows::Error),

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
