use crate::{dispatcher::wayland::state::WaylandState, HandlerCommand};
use lib_models::Command;
use tracing::warn;
use wayland_client::{
    protocol::wl_keyboard::{KeyState, WlKeyboard},
    Connection, Dispatch, QueueHandle,
};

impl Dispatch<WlKeyboard, ()> for WaylandState {
    fn event(
        state: &mut Self,
        _: &WlKeyboard,
        event: wayland_client::protocol::wl_keyboard::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            wayland_client::protocol::wl_keyboard::Event::Key {
                key,
                state: key_state,
                ..
            } => {
                let command = match key_state {
                    wayland_client::WEnum::Value(KeyState::Pressed) => {
                        HandlerCommand::Command(Command::KeyPressed(key))
                    }
                    wayland_client::WEnum::Value(KeyState::Released) => {
                        HandlerCommand::Command(Command::KeyReleased(key))
                    }
                    wayland_client::WEnum::Value(KeyState::Repeated) => {
                        HandlerCommand::Command(Command::KeyPressed(key))
                    }

                    _ => return,
                };

                let _ = state.command_tx.send(command);
            }
            _ => {}
        }
    }
}
