mod error;
mod state;

use std::{process::Stdio, sync::Arc};

pub use error::{Error, Result};

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use lib_models::{clipboard, Answer, DisplayParams};
use state::State;
use tokio::{
    net::TcpStream,
    process::{Child, Command},
    sync::Mutex,
};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use wayland_client::{Connection, EventQueue};

/// Used for temporary display
const TMP_DISPLAY_WIDTH: u32 = 2560;
/// Used for temporary display
const TMP_DISPLAY_HEIGHT: u32 = 1440;

pub fn init_queue() -> Result<EventQueue<State>> {
    let conn = Connection::connect_to_env()?;

    let event_queue: EventQueue<state::State> = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();

    _ = display.get_registry(&qhandle, ());

    Ok(event_queue)
}

pub async fn start_krfb_virtual(
    target_display: &DisplayParams,
    port: u32,
    password: &str,
) -> Result<Child> {
    println!("🚀 Starting krfb-virtualmonitor...");

    let name = &format!("air-virtual-0",);

    let child = Command::new("krfb-virtualmonitor")
        .args([
            "--resolution",
            &format!(
                "{}x{}",
                target_display.width() + 1,
                target_display.height() + 1
            ),
            "--name",
            &name,
            "--port",
            &port.to_string(),
            "--password",
            password,
            "--session",
            &format!("{name}-session"),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    println!(
        "✅ Virtual monitor created: {} ({}x{})",
        name,
        target_display.width() + 1,
        target_display.height() + 1
    );

    Ok(child)
}

pub async fn init_wayland(
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    target_display: &DisplayParams,
) -> Result<()> {
    // Initialize display
    let _display = start_krfb_virtual(&target_display, 5900, "air-client").await?;

    // Initialize Wayland
    let mut event_queue = init_queue()?;

    let (write, read) = stream.split();

    let write = Arc::new(Mutex::new(write));

    println!("Starting the example window app, press <ESC> to quit.");

    // Initialize incoming messages handler
    let incoming_write = write.clone();
    let incoming = tokio::spawn(async move {
        _ = handle_incoming(read, incoming_write).await;
    });

    let mut state = State::new(target_display.width() + 1, target_display.height() + 1);
    while state.is_running() {
        event_queue.blocking_dispatch(&mut state)?;
        state.handle(write.clone()).await?;
    }

    incoming.await?;

    write.lock().await.close().await?;

    println!("WebSocket connection closed");

    Ok(())
}

async fn handle_incoming(
    mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
) -> Result<()> {
    loop {
        if let Some(message) = read.next().await {
            match message {
                Ok(message) => match message {
                    Message::Binary(bytes) => {
                        let answer: Answer = bytes.into();
                        match answer {
                            Answer::ClipboardContents(content) => clipboard::set_contents(content)?,
                        }
                    }
                    Message::Ping(bytes) => write.lock().await.send(Message::Pong(bytes)).await?,
                    Message::Pong(_) => (),
                    Message::Close(_) => break,
                    _ => (),
                },
                Err(_) => {}
            }
        }
    }

    println!("Read closed");

    Ok(())
}
