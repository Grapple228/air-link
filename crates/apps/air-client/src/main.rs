use air_client::Result;
use futures::sink::SinkExt;
use lib_models::Command;
use tokio_tungstenite::connect_async;

use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Подключение к серверу
    let (mut ws_stream, _) = connect_async("ws://127.0.0.1:5555").await?;

    println!("Connected to the WebSocket server");

    // Пример отправки команд
    let commands: Vec<Command> = vec![
        Command::MoveMouse { x: 100, y: 0 },
        Command::MoveMouse { x: 0, y: 100 },
        Command::MoveMouse { x: 100, y: 0 },
        Command::MoveMouse { x: 0, y: 100 },
        Command::KeyCode(1),
    ];

    for command in &commands {
        // Отправка команды на сервер
        if let Err(e) = ws_stream.send(command.into()).await {
            eprintln!("Error sending message: {:?}", e);
            break;
        }
        println!("Sent command: {:?}", command);
    }

    // Закрытие соединения
    ws_stream.close(None).await?;
    println!("WebSocket connection closed");

    Ok(())
}
