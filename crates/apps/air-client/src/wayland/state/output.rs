use wayland_client::{protocol::wl_output, Dispatch};

use crate::wayland::state::State;

impl Dispatch<wl_output::WlOutput, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &wl_output::WlOutput,
        event: <wl_output::WlOutput as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wl_output::Event::Mode { width, height, .. } => {
                if state.target_width as i32 == width && state.target_height as i32 == height {
                    state.try_set_fullscreen(&proxy);
                }
            }
            _ => (),
        }
    }
}
