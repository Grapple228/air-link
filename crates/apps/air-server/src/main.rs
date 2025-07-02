use air_server::{Modifier, Result};
use enigo::{Coordinate, Enigo, Key, Keyboard, Mouse, Settings};
use futures::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use lib_models::{clipboard, Answer, Command, MouseButton};
use std::{sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::sleep,
};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

type Stream = WebSocketStream<TcpStream>;

#[tokio::main]
async fn main() -> Result<()> {
    // Инициализация сервера
    air_server::init()?;

    // Инициализация TCP слушателя
    let listener = TcpListener::bind("192.168.0.150:5555").await?;
    println!("WebSocket server is running on ws://192.168.0.150:5555");

    while let Ok((stream, _)) = listener.accept().await {
        // Обработка каждого соединения асинхронно
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

enum AnswerType {
    None,
    Clipboard,
}

struct State {
    modifiers: Modifier,
}

async fn send_clipboard(wait_ms: u64, write: Arc<Mutex<SplitSink<Stream, Message>>>) -> Result<()> {
    sleep(Duration::from_millis(wait_ms)).await;

    let content = clipboard::get_contents()?;

    write
        .lock()
        .await
        .send(Answer::ClipboardContents(content).into())
        .await?;

    Ok(())
}

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let settings = Settings {
        windows_subject_to_mouse_speed_and_acceleration_level: true,
        ..Default::default()
    };
    let mut enigo = Enigo::new(&settings)?;
    let mut state = State {
        modifiers: Modifier::None,
    };

    // Принятие WebSocket-соединения
    let ws_stream = accept_async(stream).await?;

    println!("New WebSocket connection established");

    let (write, mut read) = ws_stream.split();

    let write = Arc::new(Mutex::new(write));

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => match msg {
                Message::Binary(bytes) => {
                    let command: Command = bytes.into();
                    let answer_type = process_command(&mut state, &mut enigo, command)?;

                    match answer_type {
                        AnswerType::None => {}
                        AnswerType::Clipboard => {
                            let write = write.clone();
                            tokio::spawn(async move { send_clipboard(20, write).await });
                        }
                    }
                }
                Message::Ping(bytes) => write.lock().await.send(Message::Pong(bytes)).await?,
                Message::Pong(_) => (),
                Message::Close(_) => break,
                _ => (),
            },
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
                break;
            }
        }
    }

    println!("WebSocket connection closed");

    Ok(())
}

fn process_command(
    state: &mut State,
    enigo: &mut Enigo,
    command: impl Into<Command>,
) -> Result<AnswerType> {
    let command: Command = command.into();
    // println!("Processing command {:?}", command);

    match command {
        Command::MoveMouse { x, y } => move_mouse(enigo, x, y, MoveType::Faster)?,
        Command::SetMouse { x, y } => move_mouse(enigo, x, y, MoveType::Immediate)?,
        Command::InputText(text) => enigo.text(&text)?,
        Command::MouseButtonPressed(mouse_button) => {
            let (button, direction) = map_mouse_button(mouse_button, true);
            enigo.button(button, direction)?
        }
        Command::MouseButtonReleased(mouse_button) => {
            let (button, direction) = map_mouse_button(mouse_button, false);
            enigo.button(button, direction)?
        }
        Command::MouseScroll(mouse_scroll) => match mouse_scroll {
            lib_models::MouseScroll::Vertical(value) => {
                enigo.scroll(value.signum(), enigo::Axis::Vertical)?
            }

            lib_models::MouseScroll::Horizontal(value) => {
                enigo.scroll(value.signum(), enigo::Axis::Horizontal)?
            }
        },
        Command::KeyPressed(keycode) => {
            let direction = enigo::Direction::Press;

            match keycode {
                0x69 => enigo.key(Key::LeftArrow, direction)?,
                0x6A => enigo.key(Key::RightArrow, direction)?,
                0x6C => enigo.key(Key::DownArrow, direction)?,
                0x67 => enigo.key(Key::UpArrow, direction)?,

                0x2E => {
                    enigo.raw(keycode as u16, direction)?;

                    if state.modifiers.is_control() {
                        return Ok(AnswerType::Clipboard);
                    }
                }

                0x1D => {
                    state.modifiers.set(Modifier::LControl);
                    enigo.key(Key::LControl, direction)?;
                }

                keycode => {
                    println!("Key pressed: {:x}", keycode);

                    enigo.raw(keycode as u16, direction)?
                }
            }
        }
        Command::KeyReleased(keycode) => {
            let direction = enigo::Direction::Release;

            match keycode {
                0x69 => enigo.key(Key::LeftArrow, direction)?,
                0x6A => enigo.key(Key::RightArrow, direction)?,
                0x6C => enigo.key(Key::DownArrow, direction)?,
                0x67 => enigo.key(Key::UpArrow, direction)?,

                0x1D => {
                    state.modifiers.remove(Modifier::LControl);
                    enigo.key(Key::LControl, direction)?;
                }

                keycode => {
                    println!("Key released: {:x}", keycode);

                    enigo.raw(keycode as u16, direction)?
                }
            }
        }
        Command::SetClipboard(content) => {
            clipboard::set_contents(content)?;
        }
    }

    Ok(AnswerType::None)
}

fn map_mouse_button(
    mouse_button: MouseButton,
    is_press: bool,
) -> (enigo::Button, enigo::Direction) {
    let direction = if is_press {
        enigo::Direction::Press
    } else {
        enigo::Direction::Release
    };
    let button = match mouse_button {
        MouseButton::LEFT => enigo::Button::Left,
        MouseButton::RIGHT => enigo::Button::Right,
        MouseButton::MIDDLE => enigo::Button::Middle,
        MouseButton::MOUSE4 => enigo::Button::Back,
        MouseButton::MOUSE5 => enigo::Button::Right,
    };

    (button, direction)
}

#[derive(Debug)]
#[allow(unused)]
enum MoveType {
    /// Just sets cursor into position
    Immediate = 0,
    /// Moves cursor by 1 pixel
    Smooth = 1,
    /// Moves cursor by 2 pixels
    Faster = 2,
    /// Moves cursor by 3 pixels
    VeryFast = 3,
}

/// Smoothly moves to coordinates
fn move_mouse(enigo: &mut Enigo, target_x: i32, target_y: i32, move_type: MoveType) -> Result<()> {
    let step_size = move_type as i32;

    // Если step_size меньше или равен 0, перемещаем мышь на целевые координаты
    if step_size <= 0 {
        enigo.move_mouse(target_x, target_y, Coordinate::Abs)?;
        return Ok(());
    }

    let (mut current_x, mut current_y) = enigo.location()?;

    let mut left_x = target_x - current_x;
    let mut left_y = target_y - current_y;

    // Если оба значения меньше или равны step_size, перемещаем сразу
    if left_x.abs() <= step_size && left_y.abs() <= step_size {
        enigo.move_mouse(target_x, target_y, Coordinate::Abs)?;
        return Ok(());
    }

    // Цикл, пока оба значения не равны 0
    while left_x != 0 || left_y != 0 {
        // Вычисляем шаги по x и y
        let move_x = if left_x.abs() < step_size {
            left_x // Перемещаем на оставшееся значение
        } else {
            step_size * left_x.signum() // Используем знак для определения направления
        };

        let move_y = if left_y.abs() < step_size {
            left_y // Перемещаем на оставшееся значение
        } else {
            step_size * left_y.signum() // Используем знак для определения направления
        };

        // Обновляем текущие координаты
        let new_x = current_x + move_x;
        let new_y = current_y + move_y;

        // Перемещаем мышь к новым координатам
        enigo.move_mouse(new_x, new_y, Coordinate::Abs)?; // Перемещение к абсолютным координатам

        // Обновляем текущие координаты
        current_x = new_x;
        current_y = new_y;

        // Обновляем оставшиеся значения
        left_x = target_x - current_x;
        left_y = target_y - current_y;
    }

    Ok(())
}
