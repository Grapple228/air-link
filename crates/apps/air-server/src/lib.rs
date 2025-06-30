// region:    --- Modules

use tracing::info;
use tracing_subscriber::EnvFilter;

// -- Modules
mod error;
mod modifier;

// -- Flatten
pub use error::{Error, Result};
pub use modifier::Modifier;

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
