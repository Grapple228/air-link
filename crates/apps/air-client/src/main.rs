mod wayland;

use air_client::Result;
use futures::sink::SinkExt;
use lib_models::Command;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;
use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_keyboard::WlKeyboard;
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::protocol::wl_registry::WlRegistry;
use wayland_client::protocol::wl_seat::{self, WlSeat};
use wayland_client::protocol::wl_touch::WlTouch;
use wayland_client::protocol::{wl_compositor, wl_pointer, wl_registry, wl_touch};
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};

use std::fs::{File, OpenOptions};
use std::os::unix::thread;
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    wayland::tst();
    return Ok(());

    // Подключение к серверу
    let (mut ws_stream, _) = connect_async("ws://192.168.0.150:5555").await?;

    println!("Connected to the WebSocket server");

    let mut commands: Vec<Command> = (1..500)
        .map(|_| Command::MoveMouse { x: 1, y: 0 })
        .collect();

    commands.insert(0, Command::SetMouse { x: 1000, y: 700 });

    for command in &commands {
        // sleep(Duration::from_micros(10)).await;

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
