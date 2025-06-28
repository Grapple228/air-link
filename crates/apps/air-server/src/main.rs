use air_server::Result;
use enigo::{Coordinate, Enigo, Keyboard, Mouse, Settings};
use futures::{stream::StreamExt, SinkExt};
use lib_models::Command;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings {
        linux_delay: 30,
        wayland_display: Some("wayland-0".to_string()),
        ..Default::default()
    };
    let mut enigo = Enigo::new(&settings)?;
    enigo.move_mouse(100, 100, Coordinate::Abs)?;
    enigo.move_mouse(0, -100, Coordinate::Rel)?;
    enigo.move_mouse(0, -100, Coordinate::Rel)?;
    enigo.move_mouse(0, -100, Coordinate::Rel)?;
    enigo.move_mouse(0, -100, Coordinate::Rel)?;
    enigo.move_mouse(0, -100, Coordinate::Rel)?;
    enigo.button(enigo::Button::Left, enigo::Direction::Click);

    return Ok(());

    // Инициализация вашего сервера
    air_server::init()?;

    // Инициализация TCP слушателя
    let listener = TcpListener::bind("127.0.0.1:5555").await?;
    println!("WebSocket server is running on ws://127.0.0.1:5555");

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

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                let bytes_len = msg.len();
                bytes_count += bytes_len;
                println!("Received bytes: {bytes_len}, Total: {bytes_count}");

                match msg {
                    Message::Binary(bytes) => process_command(&mut enigo, bytes)?,
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
            println!("Move mouse");
            enigo.move_mouse(x, y, enigo::Coordinate::Abs)?;
        }
        Command::KeyCode(keycode) => enigo.raw(keycode, enigo::Direction::Press)?,
        Command::InputText(text) => enigo.text(&text)?,
    }

    Ok(())
}
