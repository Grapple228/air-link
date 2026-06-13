// region:    --- Modules

use tracing::{debug, info, Level};
use tracing_subscriber::EnvFilter;

// -- Modules
mod config;
mod error;
mod input;

// -- Flatten
pub use config::config;
pub use error::{Error, Result};
pub use input::{InputSimulator, Simulator};

// endregion: --- Modules

pub fn init() -> Result<()> {
    // LOGGING INITIALIZATION
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(Level::DEBUG)
        .init();

    info!("Initializing");

    // CONFIG INITIALIZATION
    info!("Loading config...");
    let config = config();
    debug!("{:?}", config);

    Ok(())
}
