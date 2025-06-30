use lib_models::MouseButton;
use wayland_client::{
    protocol::wl_pointer::{Axis, ButtonState, Event, WlPointer},
    Connection, Dispatch, QueueHandle, WEnum,
};

use crate::event::AppEvent;

use super::State;

impl Dispatch<WlPointer, ()> for State {
    fn event(
        state: &mut Self,
        _: &WlPointer,
        event: Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            // TODO: surface can be used for every display, so I easily understand on which is my cursor
            Event::Enter {
                surface_x,
                surface_y,
                ..
            } => {
                state.is_focus = true;
                state.event = AppEvent::MouseEnter {
                    x: surface_x as i32,
                    y: surface_y as i32,
                };
            }
            // TODO: surface can be used for every display, so I easily understand on which is my cursor
            Event::Leave { .. } => {
                state.is_focus = false;
                state.event = AppEvent::MouseLeave;
            }
            Event::Motion {
                surface_x,
                surface_y,
                ..
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
            Event::Button {
                button,
                state: button_state,
                ..
            } => {
                let WEnum::Value(button_state) = button_state else {
                    return;
                };

                let button = MouseButton::from_linux_code(button);

                match button_state {
                    ButtonState::Released => {
                        state.event = AppEvent::MouseButtonReleased(button);
                    }
                    ButtonState::Pressed => state.event = AppEvent::MouseButtonPressed(button),
                    _ => {}
                }
            }
            Event::Axis { axis, value, .. } => {
                let WEnum::Value(axis) = axis else {
                    return;
                };

                let value = value as i32;

                if value == 0 {
                    return;
                }

                match axis {
                    Axis::VerticalScroll => {
                        state.event = AppEvent::ScrollVertical(value as i32);
                    }
                    Axis::HorizontalScroll => {
                        state.event = AppEvent::ScrollHorizontal(value as i32);
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
