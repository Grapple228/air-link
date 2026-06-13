// air_client2/src/dispatcher/wayland/handlers/xdg.rs
use crate::dispatcher::wayland::state::WaylandState;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::xdg::shell::client::{
    xdg_surface::XdgSurface,
    xdg_toplevel::XdgToplevel,
    xdg_wm_base::{self, XdgWmBase},
};

impl Dispatch<XdgWmBase, ()> for WaylandState {
    fn event(
        _state: &mut Self,
        wm_base: &XdgWmBase,
        event: xdg_wm_base::Event,
        _: &(),
        _: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            xdg_wm_base::Event::Ping { serial } => {
                wm_base.pong(serial);
            }
            _ => {}
        }
    }
}

// Добавь реализацию для XdgSurface
impl Dispatch<XdgSurface, ()> for WaylandState {
    fn event(
        _state: &mut Self,
        xdg_surface: &XdgSurface,
        event: wayland_protocols::xdg::shell::client::xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            wayland_protocols::xdg::shell::client::xdg_surface::Event::Configure {
                serial, ..
            } => {
                xdg_surface.ack_configure(serial);
            }
            _ => {}
        }
    }
}

// Добавь реализацию для XdgToplevel
impl Dispatch<XdgToplevel, ()> for WaylandState {
    fn event(
        _state: &mut Self,
        _: &XdgToplevel,
        event: wayland_protocols::xdg::shell::client::xdg_toplevel::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            wayland_protocols::xdg::shell::client::xdg_toplevel::Event::Close => {
                println!("Window close requested");
            }
            wayland_protocols::xdg::shell::client::xdg_toplevel::Event::Configure { .. } => {}
            _ => {}
        }
    }
}
