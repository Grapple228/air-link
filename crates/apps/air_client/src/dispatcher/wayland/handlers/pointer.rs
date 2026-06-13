// air_client2/src/dispatcher/wayland/handlers/pointer.rs

use crate::{dispatcher::wayland::state::WaylandState, HandlerCommand};
use lib_models::{Command, MouseButton, MouseScroll};
use wayland_client::{
    protocol::wl_pointer::{Axis, ButtonState, Event, WlPointer},
    Connection, Dispatch, Proxy, QueueHandle,
};

impl Dispatch<WlPointer, ()> for WaylandState {
    fn event(
        state: &mut Self,
        _: &WlPointer,
        event: Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            Event::Enter { surface, .. } => {
                // Получаем ID поверхности
                let surface_id = surface.id();

                if let Some(our_surface) = &state.surface {
                    if surface_id == our_surface.id() {
                        state.is_on_virtual = true;
                    } else {
                        state.is_on_virtual = false;
                    }
                } else {
                    state.is_on_virtual = false;
                }
            }

            Event::Motion {
                surface_x,
                surface_y,
                ..
            } => {
                if state.is_on_virtual {
                    let command = HandlerCommand::Command(Command::SetMouse {
                        x: surface_x as i32,
                        y: surface_y as i32,
                    });
                    let _ = state.command_tx.send(command);
                }
            }

            Event::Leave { surface, .. } => {
                // Получаем ID поверхности
                let surface_id = surface.id();

                if let Some(our_surface) = &state.surface {
                    if surface_id == our_surface.id() {
                        state.is_on_virtual = false;
                    }
                } else {
                    state.is_on_virtual = false;
                }
            }

            Event::Button {
                button,
                state: btn_state,
                ..
            } => {
                if !state.is_on_virtual {
                    return;
                }

                let mouse_button = match button {
                    272 => MouseButton::LEFT,
                    273 => MouseButton::RIGHT,
                    274 => MouseButton::MIDDLE,
                    _ => return,
                };

                match btn_state {
                    wayland_client::WEnum::Value(ButtonState::Pressed) => {
                        let _ = state.command_tx.send(HandlerCommand::Command(
                            Command::MouseButtonPressed(mouse_button),
                        ));
                    }
                    wayland_client::WEnum::Value(ButtonState::Released) => {
                        let _ = state.command_tx.send(HandlerCommand::Command(
                            Command::MouseButtonReleased(mouse_button),
                        ));
                    }
                    _ => {}
                }
            }

            Event::Axis { axis, value, .. } => {
                if !state.is_on_virtual {
                    return;
                }

                let value = value as i32;
                if value == 0 {
                    return;
                }

                let scroll = match axis {
                    wayland_client::WEnum::Value(Axis::VerticalScroll) => {
                        MouseScroll::Vertical(value)
                    }
                    wayland_client::WEnum::Value(Axis::HorizontalScroll) => {
                        MouseScroll::Horizontal(value)
                    }
                    _ => return,
                };

                let _ = state
                    .command_tx
                    .send(HandlerCommand::Command(Command::MouseScroll(scroll)));
            }

            _ => {}
        }
    }
}
