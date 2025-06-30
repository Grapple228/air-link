use wayland_client::{
    protocol::wl_seat::{Capability, Event, WlSeat},
    Connection, Dispatch, QueueHandle, WEnum,
};

use super::State;

impl Dispatch<WlSeat, ()> for State {
    fn event(
        _: &mut Self,
        seat: &WlSeat,
        event: Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let Event::Capabilities {
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
