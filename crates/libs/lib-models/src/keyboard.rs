#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum KeyboardButton {}

impl From<u32> for KeyboardButton {
    fn from(value: u32) -> Self {
        match value {
            _ => panic!("Unsupported keyboard keycode"),
        }
    }
}
