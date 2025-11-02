use super::State;
use crate::event::AppEvent;
use wayland_client::{
    protocol::wl_keyboard::{Event, KeyState, WlKeyboard},
    Connection, Dispatch, QueueHandle, WEnum,
};

impl Dispatch<WlKeyboard, ()> for State {
    fn event(
        state: &mut Self,
        _: &WlKeyboard,
        event: Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            #[allow(unused)]
            Event::Keymap { format, fd, size } => {
                // println!("keymap {:?} {:?} {}", format, fd, size);
            }
            // TODO: surface can be used for every display, so I easily understand on which is my cursor
            Event::Enter { .. } => {}
            // TODO: surface can be used for every display, so I easily understand on which is my cursor
            // could be useful
            Event::Leave { .. } => {}
            Event::Key {
                key,
                state: key_state,
                ..
            } => {
                let WEnum::Value(key_state) = key_state else {
                    return;
                };

                match key_state {
                    KeyState::Released => state.event = AppEvent::KeyReleased(key),
                    KeyState::Pressed => state.event = AppEvent::KeyPressed(key),
                    _ => {}
                }
            }
            Event::Modifiers { .. } => {
                // println!(
                //     "modifiers {} {} {} ",
                //     mods_depressed, mods_latched, mods_locked
                // );
            }
            #[allow(unused)]
            Event::RepeatInfo { rate, delay } => {
                // println!("repeat {rate} {delay}");
            }
            _ => {
                // println!("other");
            }
        }
    }
}
