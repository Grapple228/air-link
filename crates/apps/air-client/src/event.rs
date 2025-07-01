use lib_models::MouseButton;

#[derive(Debug, Default)]
#[allow(unused)]
pub enum AppEvent {
    #[default]
    None,
    MouseMove {
        x: i32,
        y: i32,
    },
    MouseEnter {
        x: i32,
        y: i32,
    },
    MouseLeave,
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    ScrollHorizontal(i32),
    ScrollVertical(i32),

    KeyPressed(u32),
    KeyReleased(u32),
}
