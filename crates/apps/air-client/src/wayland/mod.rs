use super::Result;
use std::{
    fs::File,
    os::{fd::OwnedFd, unix::io::AsFd},
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use futures::{
    channel::mpsc::{self, channel, Receiver, Sender},
    future::{poll_fn, select, Either},
    SinkExt, StreamExt, TryFutureExt,
};
use lib_models::{Command, MouseButton, MouseScroll};
use tokio::{net::TcpStream, runtime::Runtime, sync::Mutex, time::sleep};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use wayland_client::{
    backend::{self, Backend},
    delegate_noop,
    protocol::{
        wl_buffer, wl_callback, wl_compositor, wl_display, wl_keyboard, wl_pointer, wl_registry,
        wl_seat::{self, Capability},
        wl_shm, wl_shm_pool, wl_surface, wl_touch,
    },
    Connection, Dispatch, EventQueue, Proxy, QueueHandle, WEnum,
};

use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

// Ignore events from these object types in this example.
delegate_noop!(State: ignore wl_compositor::WlCompositor);
delegate_noop!(State: ignore wl_surface::WlSurface);
delegate_noop!(State: ignore wl_shm::WlShm);
delegate_noop!(State: ignore wl_shm_pool::WlShmPool);
delegate_noop!(State: ignore wl_buffer::WlBuffer);

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            match &interface[..] {
                "wl_compositor" => {
                    let compositor =
                        registry.bind::<wl_compositor::WlCompositor, _, _>(name, 1, qh, ());
                    let surface = compositor.create_surface(qh, ());
                    state.base_surface = Some(surface);

                    if state.wm_base.is_some() && state.xdg_surface.is_none() {
                        state.init_xdg_surface(qh);
                    }
                }
                "wl_shm" => {
                    let shm = registry.bind::<wl_shm::WlShm, _, _>(name, 1, qh, ());

                    let (init_w, init_h) = (1280, 720);

                    let mut file = tempfile::tempfile().unwrap();
                    draw(&mut file, (init_w, init_h));
                    let pool = shm.create_pool(file.as_fd(), (init_w * init_h * 4) as i32, qh, ());
                    let buffer = pool.create_buffer(
                        0,
                        init_w as i32,
                        init_h as i32,
                        (init_w * 4) as i32,
                        wl_shm::Format::Argb8888,
                        qh,
                        (),
                    );
                    state.buffer = Some(buffer.clone());

                    if state.configured {
                        let surface = state.base_surface.as_ref().unwrap();
                        surface.attach(Some(&buffer), 0, 0);
                        surface.commit();
                    }
                }
                "wl_seat" => {
                    let seat = registry.bind::<wl_seat::WlSeat, _, _>(name, 1, qh, ());
                }
                "xdg_wm_base" => {
                    let wm_base = registry.bind::<xdg_wm_base::XdgWmBase, _, _>(name, 1, qh, ());
                    state.wm_base = Some(wm_base);

                    if state.base_surface.is_some() && state.xdg_surface.is_none() {
                        state.init_xdg_surface(qh);
                    }
                }
                _ => {}
            }
        }
    }
}

fn draw(tmp: &mut File, (buf_x, buf_y): (u32, u32)) {
    use std::{cmp::min, io::Write};
    let mut buf = std::io::BufWriter::new(tmp);
    for y in 0..buf_y {
        for x in 0..buf_x {
            let a = 0xFF;
            let r = min(((buf_x - x) * 0xFF) / buf_x, ((buf_y - y) * 0xFF) / buf_y);
            let g = min((x * 0xFF) / buf_x, ((buf_y - y) * 0xFF) / buf_y);
            let b = min(((buf_x - x) * 0xFF) / buf_x, (y * 0xFF) / buf_y);
            buf.write_all(&[b as u8, g as u8, r as u8, a as u8])
                .unwrap();
        }
    }
    buf.flush().unwrap();
}

impl State {
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

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for State {
    fn event(
        _: &mut Self,
        wm_base: &xdg_wm_base::XdgWmBase,
        event: xdg_wm_base::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm_base.pong(serial);
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for State {
    fn event(
        state: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial, .. } = event {
            xdg_surface.ack_configure(serial);
            state.configured = true;
            let surface = state.base_surface.as_ref().unwrap();
            if let Some(ref buffer) = state.buffer {
                surface.attach(Some(buffer), 0, 0);
                surface.commit();
            }
        }
    }
}

impl Dispatch<xdg_toplevel::XdgToplevel, ()> for State {
    fn event(
        state: &mut Self,
        _: &xdg_toplevel::XdgToplevel,
        event: xdg_toplevel::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_toplevel::Event::Close = event {
            state.running = false;
        }
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        _: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(capabilities),
        } = event
        {
            if capabilities.contains(Capability::Keyboard) {
                seat.get_keyboard(qh, ());
            }

            if capabilities.contains(Capability::Pointer) {
                seat.get_pointer(qh, ());
            }
        }
    }
}

impl Dispatch<wl_pointer::WlPointer, ()> for State {
    fn event(
        state: &mut Self,
        _: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            wl_pointer::Event::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
            } => {
                state.is_focus = true;
                state.event = AppEvent::MouseEnter {
                    x: surface_x as i32,
                    y: surface_y as i32,
                };
            }
            wl_pointer::Event::Leave { serial, surface } => {
                state.is_focus = false;
                state.event = AppEvent::MouseLeave;
            }
            wl_pointer::Event::Motion {
                time,
                surface_x,
                surface_y,
            } => {
                // Skip mouse move right after mouse enter
                match &state.event {
                    AppEvent::MouseEnter { .. } => return,
                    _ => {}
                }

                state.event = AppEvent::MouseMove {
                    x: surface_x as i32,
                    y: surface_y as i32,
                };
            }
            wl_pointer::Event::Button {
                serial,
                time,
                button,
                state: button_state,
            } => {
                let WEnum::Value(button_state) = button_state else {
                    return;
                };

                let button = MouseButton::from_linux_code(button);

                match button_state {
                    wl_pointer::ButtonState::Released => {
                        state.event = AppEvent::MouseButtonReleased(button);
                    }
                    wl_pointer::ButtonState::Pressed => {
                        state.event = AppEvent::MouseButtonPressed(button)
                    }
                    _ => {}
                }
            }
            wl_pointer::Event::Axis { time, axis, value } => {
                let WEnum::Value(axis) = axis else {
                    return;
                };

                match axis {
                    wl_pointer::Axis::VerticalScroll => {
                        state.event = AppEvent::ScrollVertical(value);
                    }
                    wl_pointer::Axis::HorizontalScroll => {
                        state.event = AppEvent::ScrollHorizontal(value);
                    }
                    _ => {}
                }
            }
            _ => {
                println!("other");
            }
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for State {
    fn event(
        state: &mut Self,
        _: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let wl_keyboard::Event::Key { key, .. } = event {
            if key == 1 {
                // ESC key
                state.running = false;
            }
        }
    }
}

#[derive(Debug, Default)]
struct State {
    running: bool,
    base_surface: Option<wl_surface::WlSurface>,
    buffer: Option<wl_buffer::WlBuffer>,
    wm_base: Option<xdg_wm_base::XdgWmBase>,
    xdg_surface: Option<(xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel)>,
    configured: bool,

    is_focus: bool,
    event: AppEvent,
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
enum AppEvent {
    #[default]
    None,
    MouseMove {
        x: i32,
        y: i32,
    },
    MouseEnter {
        x: i32,
        y: i32,
    },
    MouseLeave,
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    ScrollHorizontal(f64),
    ScrollVertical(f64),
}

impl State {
    pub async fn handle(
        &mut self,
        stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> Result<()> {
        let event = &self.event;
        // println!("handle {event:?}");

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
                self.x = *x;
                self.y = *y;

                Command::SetMouse { x: *x, y: *y }
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
                println!("Horizontal scroll: {}", value);

                Command::MouseScroll(MouseScroll::Horizontal(*value))
            }
            AppEvent::ScrollVertical(value) => {
                println!("Vertival scroll: {}", value);

                Command::MouseScroll(MouseScroll::Vertical(*value))
            }
        };

        stream.send(command.into()).await?;

        self.event = AppEvent::None;

        Ok(())
    }
}

pub async fn init_wayland(mut stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<()> {
    // Initialize Wayland connection
    let conn = Connection::connect_to_env().unwrap();

    let mut event_queue: EventQueue<State> = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qhandle, ());

    println!("Starting the example window app, press <ESC> to quit.");

    let mut state = State {
        running: true,
        ..Default::default()
    };

    while state.running {
        event_queue.blocking_dispatch(&mut state)?;
        state.handle(&mut stream).await?;
    }

    Ok(())
}
