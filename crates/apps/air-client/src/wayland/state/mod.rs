use std::sync::Arc;

use super::Result;
use crate::event::AppEvent;
use futures::{stream::SplitSink, SinkExt};
use lib_models::{Command, MouseScroll};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use wayland_client::{
    delegate_noop,
    protocol::{
        wl_buffer::WlBuffer, wl_compositor::WlCompositor, wl_shm::WlShm, wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    },
    QueueHandle,
};
use wayland_protocols::xdg::shell::client::{
    xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase,
};

mod keyboard;
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
    resolution_rate: f64,
    event: AppEvent,
    x: i32,
    y: i32,
}

impl State {
    pub fn new(resolution_rate: f64) -> Self {
        Self {
            running: true,
            resolution_rate,
            base_surface: None,
            buffer: None,
            wm_base: None,
            xdg_surface: None,
            configured: false,
            is_focus: false,
            event: AppEvent::None,
            x: 0,
            y: 0,
        }
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

                let x = map_cord(*x, self.resolution_rate);
                let y = map_cord(*y, self.resolution_rate);

                self.x = x;
                self.y = y;

                Command::MoveMouse { x, y }
            }
            AppEvent::MouseEnter { x, y } => {
                let x = map_cord(*x, self.resolution_rate);
                let y = map_cord(*y, self.resolution_rate);

                self.x = x;
                self.y = y;

                Command::SetMouse { x, y }
            }
            AppEvent::MouseLeave => {
                return Ok(());
            }
            AppEvent::MouseButtonPressed(mouse_button) => {
                println!("Mouse {:?} pressed", mouse_button);

                Command::MouseButtonPressed(mouse_button.clone())
            }
            AppEvent::MouseButtonReleased(mouse_button) => {
                println!("Mouse {:?} released", mouse_button);

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

        base_surface.commit();

        self.xdg_surface = Some((xdg_surface, toplevel));
    }
}

#[inline]
/// TODO: TMP FUNCTION, ONLY FOR DEV
fn map_cord(cord: i32, resolution_rate: f64) -> i32 {
    (cord as f64 / resolution_rate) as i32
}
