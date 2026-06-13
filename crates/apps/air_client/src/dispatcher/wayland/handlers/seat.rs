use crate::dispatcher::wayland::state::WaylandState;
use wayland_client::{
    protocol::wl_seat::{Capability, WlSeat},
    Connection, Dispatch, QueueHandle, WEnum,
};

impl Dispatch<WlSeat, ()> for WaylandState {
    fn event(
        state: &mut Self,
        seat: &WlSeat,
        event: wayland_client::protocol::wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wayland_client::protocol::wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(capabilities),
        } = event
        {
            if capabilities.contains(Capability::Keyboard) {
                let keyboard = seat.get_keyboard(qh, ());
                println!("⌨️ Keyboard registered");
                state.keyboard = Some(keyboard);
            }

            if capabilities.contains(Capability::Pointer) {
                let pointer = seat.get_pointer(qh, ());
                println!("🖱️ Pointer registered");
                state.pointer = Some(pointer);
            }
        }
    }
}
