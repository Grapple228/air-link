use lib_models::Command;

use super::Result;

pub trait Deleg<K, P> {
    fn execute_keyboard(keyboard_event: K) -> Result<()>;
    fn execute_pointer(pointer_event: P) -> Result<()>;
}
