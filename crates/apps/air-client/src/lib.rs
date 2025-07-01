// region:    --- Modules

use tracing::info;
use tracing_subscriber::EnvFilter;

// -- Modules
mod error;
mod event;
#[cfg(target_os = "linux")]
mod wayland;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod x11;

// -- Flatten
pub use error::{Error, Result};

#[cfg(target_os = "linux")]
pub use wayland::init_wayland;
#[cfg(target_os = "windows")]
pub use windows::init_windows;
#[cfg(target_os = "linux")]
pub use x11::init_x11;

// endregion: --- Modules

pub fn init() -> Result<()> {
    // LOGGING INITIALIZATION
    tracing_subscriber::fmt()
        .without_time() // For early development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Initializing");

    Ok(())
}
