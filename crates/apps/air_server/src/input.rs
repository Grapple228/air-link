use super::Result;
use lib_models::{MouseButton, MouseScroll};

pub trait InputSimulator {
    fn set_mouse(&mut self, x: i32, y: i32) -> Result<()>;
    fn move_mouse(&mut self, x: i32, y: i32) -> Result<()>;
    fn mouse_press(&mut self, button: MouseButton) -> Result<()>;
    fn mouse_release(&mut self, button: MouseButton) -> Result<()>;
    fn key_press(&mut self, keycode: u32) -> Result<()>;
    fn key_release(&mut self, keycode: u32) -> Result<()>;
    fn scroll(&mut self, scroll: MouseScroll) -> Result<()>;
    fn text(&mut self, text: &str) -> Result<()>;
}

pub struct Simulator {
    #[cfg(target_os = "windows")]
    inner: enigo::Enigo,

    #[cfg(target_os = "linux")]
    mouse: mouce::Mouse,
}

impl Simulator {
    #[cfg(target_os = "windows")]
    pub fn new() -> Result<Self> {
        use enigo::Enigo;

        let enigo_settings = enigo::Settings {
            x11_display: std::env::var("DISPLAY").ok(),
            wayland_display: std::env::var("WAYLAND_DISPLAY").ok(),

            linux_delay: 1,
            windows_subject_to_mouse_speed_and_acceleration_level: false,
            release_keys_when_dropped: true,
            independent_of_keyboard_state: true,
            ..Default::default()
        };

        let enigo_settings = enigo::Settings::default();

        let inner = Enigo::new(&enigo_settings).unwrap();
        Ok(Self { inner })
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> Result<Self> {
        Ok(Self {
            mouse: mouce::Mouse::new(),
        })
    }

    #[cfg(target_os = "windows")]
    const fn map_mouse_button(mouse_button: MouseButton) -> enigo::Button {
        match mouse_button {
            MouseButton::LEFT => enigo::Button::Left,
            MouseButton::RIGHT => enigo::Button::Right,
            MouseButton::MIDDLE => enigo::Button::Middle,
            MouseButton::MOUSE4 => enigo::Button::Back,
            MouseButton::MOUSE5 => enigo::Button::Forward,
        }
    }
}

#[cfg(target_os = "linux")]
impl InputSimulator for Simulator {
    fn set_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        use mouce::MouseActions;

        self.mouse.move_to(x, y).unwrap();

        Ok(())
    }

    fn move_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        use mouce::MouseActions;

        self.mouse.move_relative(x, y).unwrap();

        Ok(())
    }

    fn mouse_press(&mut self, _button: MouseButton) -> Result<()> {
        todo!()
    }

    fn mouse_release(&mut self, _button: MouseButton) -> Result<()> {
        todo!()
    }

    fn scroll(&mut self, _scroll: MouseScroll) -> Result<()> {
        todo!()
    }

    fn key_press(&mut self, _keycode: u32) -> Result<()> {
        todo!()
    }

    fn key_release(&mut self, _keycode: u32) -> Result<()> {
        todo!()
    }

    fn text(&mut self, _text: &str) -> Result<()> {
        todo!()
    }
}

#[cfg(target_os = "windows")]
impl InputSimulator for Simulator {
    fn set_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        use enigo::{Coordinate, Mouse};
        self.inner.move_mouse(x, y, Coordinate::Abs).unwrap();
        Ok(())
    }

    fn move_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        use enigo::{Coordinate, Mouse};
        self.inner.move_mouse(x, y, Coordinate::Rel).unwrap();
        Ok(())
    }

    fn mouse_press(&mut self, button: MouseButton) -> Result<()> {
        use enigo::Mouse;
        let button = Self::map_mouse_button(button);
        self.inner.button(button, enigo::Direction::Press).unwrap();
        Ok(())
    }

    fn mouse_release(&mut self, button: MouseButton) -> Result<()> {
        use enigo::Mouse;
        let button = Self::map_mouse_button(button);
        self.inner
            .button(button, enigo::Direction::Release)
            .unwrap();
        Ok(())
    }

    fn key_press(&mut self, keycode: u32) -> Result<()> {
        use enigo::{Key, Keyboard};
        let direction = enigo::Direction::Press;

        match keycode {
            0x69 => self.inner.key(Key::LeftArrow, direction).unwrap(),
            0x6A => self.inner.key(Key::RightArrow, direction).unwrap(),
            0x6C => self.inner.key(Key::DownArrow, direction).unwrap(),
            0x67 => self.inner.key(Key::UpArrow, direction).unwrap(),
            keycode => self.inner.raw(keycode as u16, direction).unwrap(),
        }

        Ok(())
    }

    fn key_release(&mut self, keycode: u32) -> Result<()> {
        use enigo::{Key, Keyboard};
        let direction = enigo::Direction::Release;

        match keycode {
            0x69 => self.inner.key(Key::LeftArrow, direction).unwrap(),
            0x6A => self.inner.key(Key::RightArrow, direction).unwrap(),
            0x6C => self.inner.key(Key::DownArrow, direction).unwrap(),
            0x67 => self.inner.key(Key::UpArrow, direction).unwrap(),
            keycode => self.inner.raw(keycode as u16, direction).unwrap(),
        }

        Ok(())
    }

    fn scroll(&mut self, scroll: MouseScroll) -> Result<()> {
        use enigo::Mouse;

        match scroll {
            MouseScroll::Vertical(value) => {
                self.inner
                    .scroll(value.signum(), enigo::Axis::Vertical)
                    .unwrap();
            }
            MouseScroll::Horizontal(value) => {
                self.inner
                    .scroll(value.signum(), enigo::Axis::Horizontal)
                    .unwrap();
            }
        }

        Ok(())
    }

    fn text(&mut self, text: &str) -> Result<()> {
        use enigo::Keyboard;
        self.inner.text(text).unwrap();
        Ok(())
    }
}
