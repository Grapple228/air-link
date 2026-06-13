use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use wayland_client::{Connection, EventQueue};

use super::{Error, Result};
use crate::{
    config,
    dispatcher::{wayland::state::WaylandState, DispatcherTrait},
    HandlerCommand, VirtualDisplay,
};

mod handlers;
mod state;

pub struct WaylandDispatcher {
    command_tx: flume::Sender<HandlerCommand>,
    running: Arc<AtomicBool>,
    display: Option<VirtualDisplay>,
}

impl WaylandDispatcher {
    pub fn new(command_tx: flume::Sender<HandlerCommand>, is_running: Arc<AtomicBool>) -> Self {
        Self {
            command_tx,
            running: is_running,
            display: None,
        }
    }

    pub fn init_virtual_display(&mut self, width: u32, height: u32) -> Result<String> {
        if let Some(display) = self.display.take() {
            display.remove();
        }

        let display =
            VirtualDisplay::create(width, height).map_err(|_| Error::DisplayCreateFail)?;

        println!("✅ Virtual display created: {}", display);

        let output_name = display.output_name().to_string();
        self.display = Some(display);

        Ok(output_name)
    }
}

impl DispatcherTrait for WaylandDispatcher {
    fn run(&mut self) -> Result<()> {
        // Init display
        let output_name = self.init_virtual_display(config().WIDTH, config().HEIGHT)?;

        // Connect to wayland
        let conn = Connection::connect_to_env().map_err(|_| Error::WaylandConnectFail)?;
        let mut event_queue: EventQueue<WaylandState> = conn.new_event_queue();
        let qh = event_queue.handle();

        // Create state
        let mut state = WaylandState::new(self.command_tx.clone(), output_name);

        // Get global registry
        let registry = conn.display().get_registry(&qh, ());
        state.registry = Some(registry);

        println!("🔄 Wayland dispatcher running...");
        self.running.store(true, Ordering::Relaxed);

        while self.running.load(Ordering::Relaxed) {
            event_queue
                .blocking_dispatch(&mut state)
                .map_err(|_| Error::WaylandDispatchFail)?;
        }

        self.stop();

        Ok(())
    }

    fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);

        if let Some(display) = self.display.take() {
            display.remove();
        }
    }
}

impl Drop for WaylandDispatcher {
    fn drop(&mut self) {
        self.stop();
        println!("🛑 Wayland dispatcher stopped");
    }
}
