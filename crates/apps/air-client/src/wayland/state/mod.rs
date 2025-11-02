use std::sync::Arc;

use super::Result;
use crate::event::AppEvent;
use futures::{stream::SplitSink, SinkExt};
use lib_models::{clipboard, Command, MouseScroll};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::warn;
use wayland_client::{
    delegate_noop,
    protocol::{
        wl_buffer::WlBuffer, wl_compositor::WlCompositor, wl_output::WlOutput, wl_shm::WlShm,
        wl_shm_pool::WlShmPool, wl_surface::WlSurface,
    },
    QueueHandle,
};
use wayland_protocols::xdg::shell::client::{
    xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase,
};

mod keyboard;
mod output;
mod pointer;
mod registry;
mod seat;
mod xdg;

// Ignore events from these object types in this example.
delegate_noop!(State: ignore WlCompositor);
delegate_noop!(State: ignore WlSurface);
delegate_noop!(State: ignore WlShm);
delegate_noop!(State: ignore WlShmPool);
delegate_noop!(State: ignore WlBuffer);

#[derive(Debug, Default)]
pub struct State {
    running: bool,
    base_surface: Option<WlSurface>,
    buffer: Option<WlBuffer>,
    wm_base: Option<XdgWmBase>,
    xdg_surface: Option<(XdgSurface, XdgToplevel)>,
    configured: bool,

    is_focus: bool,

    event: AppEvent,
    x: i32,
    y: i32,

    pending_fullscreen: bool,

    target_width: u32,
    target_height: u32,
}

impl State {
    pub fn new(target_width: u32, target_height: u32) -> Self {
        Self {
            running: true,
            base_surface: None,
            buffer: None,
            wm_base: None,
            xdg_surface: None,
            configured: false,
            is_focus: false,
            event: AppEvent::None,
            x: 0,
            y: 0,
            pending_fullscreen: true,

            target_width,
            target_height,
        }
    }

    pub fn try_set_fullscreen(&mut self, wl_output: &WlOutput) {
        // if self.pending_fullscreen {

        if let Some((_, top_level)) = &self.xdg_surface {
            top_level.set_fullscreen(Some(&wl_output));
            self.pending_fullscreen = false;
        }
        // }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub async fn handle(
        &mut self,
        write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    ) -> Result<()> {
        let event = &self.event;

        let command: Command = match event {
            AppEvent::None => {
                return Ok(());
            }
            AppEvent::MouseMove { x, y } => {
                if !self.is_focus {
                    return Ok(());
                }

                self.x = *x;
                self.y = *y;

                Command::MoveMouse { x: *x, y: *y }
            }
            AppEvent::MouseEnter { x, y } => {
                // Send clipboard
                let clipboard_write = write.clone();
                tokio::spawn(async move {
                    let Ok(content) = clipboard::get_contents() else {
                        warn!("Failed to get clipboard contents");
                        return;
                    };

                    _ = clipboard_write
                        .lock()
                        .await
                        .send(Command::SetClipboard(content).into())
                        .await;
                });

                self.x = *x;
                self.y = *y;

                Command::SetMouse { x: *x, y: *y }
            }
            AppEvent::MouseLeave => {
                return Ok(());
            }
            AppEvent::MouseButtonPressed(mouse_button) => {
                Command::MouseButtonPressed(mouse_button.clone())
            }
            AppEvent::MouseButtonReleased(mouse_button) => {
                Command::MouseButtonReleased(mouse_button.clone())
            }
            AppEvent::ScrollHorizontal(value) => {
                Command::MouseScroll(MouseScroll::Horizontal(*value))
            }
            AppEvent::ScrollVertical(value) => Command::MouseScroll(MouseScroll::Vertical(*value)),
            AppEvent::KeyPressed(key) => Command::KeyPressed(*key),
            AppEvent::KeyReleased(key) => Command::KeyReleased(*key),
        };

        write.lock().await.send(command.into()).await?;

        self.event = AppEvent::None;

        Ok(())
    }

    fn init_xdg_surface(&mut self, qh: &QueueHandle<State>) {
        let wm_base = self.wm_base.as_ref().unwrap();
        let base_surface = self.base_surface.as_ref().unwrap();

        let xdg_surface = wm_base.get_xdg_surface(base_surface, qh, ());
        let toplevel = xdg_surface.get_toplevel(qh, ());
        toplevel.set_title("A fantastic window!".into());

        toplevel.set_min_size(2560, 1440);
        toplevel.set_max_size(2560, 1440);

        base_surface.commit();

        self.xdg_surface = Some((xdg_surface, toplevel));
    }
}
