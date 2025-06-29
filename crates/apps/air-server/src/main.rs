use air_server::Result;
use chrono::Utc;
use enigo::{Coordinate, Enigo, Keyboard, Mouse, Settings};
use futures::{stream::StreamExt, SinkExt};
use lib_models::Command;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    // Инициализация вашего сервера
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

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;

    // Принятие WebSocket-соединения
    let ws_stream = accept_async(stream).await?;

    println!("New WebSocket connection established");

    let (mut write, mut read) = ws_stream.split();

    let mut bytes_count = 0;
    let mut msg_count = 0;

    while let Some(message) = read.next().await {
        let time = Utc::now();
        match message {
            Ok(msg) => {
                let bytes_len = msg.len();
                bytes_count += bytes_len;
                msg_count += 1;

                match msg {
                    Message::Binary(bytes) => {
                        let command: Command = bytes.into();
                        println!("[{time}] {command:?}: Received bytes: {bytes_len}, Total: {bytes_count}, Mesages: {msg_count}");
                        process_command(&mut enigo, command)?
                    }
                    Message::Ping(bytes) => write.send(Message::Pong(bytes)).await?,
                    Message::Pong(_) => (),
                    Message::Close(_) => break,
                    _ => (),
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
                break;
            }
        }
    }

    println!("WebSocket connection closed");

    Ok(())
}

fn process_command(enigo: &mut Enigo, command: impl Into<Command>) -> Result<()> {
    let command: Command = command.into();
    debug!("Processing command {:?}", command);

    match command {
        Command::MoveMouse { x, y } => {
            move_mouse(enigo, x, y, MoveType::Smooth)?;
        }
        Command::SetMouse { x, y } => {
            move_mouse(enigo, x, y, MoveType::Immediate)?;
        }

        Command::KeyCode(keycode) => enigo.raw(keycode, enigo::Direction::Press)?,
        Command::InputText(text) => enigo.text(&text)?,
    }

    Ok(())
}

#[derive(Debug)]
enum MoveType {
    Immediate = 0,
    Smooth = 1,
    Faster = 2,
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
