mod error;
mod wayland;

use std::sync::{atomic::AtomicBool, Arc};

pub use error::{Error, Result};

use crate::HandlerCommand;

#[enum_dispatch::enum_dispatch(DispatcherTrait)]
pub enum Dispatcher {
    #[cfg(unix)]
    Wayland(wayland::WaylandDispatcher),
    #[cfg(windows)]
    Windows,
}

impl Dispatcher {
    pub fn init(tx: flume::Sender<HandlerCommand>, is_running: Arc<AtomicBool>) -> Result<Self> {
        #[cfg(unix)]
        let dispatcher = {
            use tracing::info;

            info!("Creating unix dispatcher...");
            wayland::WaylandDispatcher::new(tx, is_running)
        };
        #[cfg(windows)]
        let dispatcher = {
            use tracing::info;

            info!("Creating windows dispatcher...");
            panic!("windows currently not supported")
        };

        Ok(dispatcher.into())
    }
}

#[enum_dispatch::enum_dispatch]
pub trait DispatcherTrait {
    fn run(&mut self) -> Result<()>;
    fn stop(&mut self);
}
