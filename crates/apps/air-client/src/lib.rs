// region:    --- Modules

use tracing::info;
use tracing_subscriber::EnvFilter;

// -- Modules
mod error;
mod wayland;

// -- Flatten
pub use error::{Error, Result};
pub use wayland::init_wayland;

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
