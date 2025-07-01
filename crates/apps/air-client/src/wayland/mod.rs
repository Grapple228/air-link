mod error;
mod state;

use std::sync::Arc;

use cli_clipboard::{ClipboardContext, ClipboardProvider};
pub use error::{Error, Result};

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use lib_models::{Answer, DisplayParams};
use state::State;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::warn;
use wayland_client::{Connection, EventQueue};

/// Used for temporary display
const TMP_DISPLAY_WIDTH: u32 = 1280;
/// Used for temporary display
const TMP_DISPLAY_HEIGHT: u32 = 720;

pub fn init_queue() -> Result<EventQueue<State>> {
    let conn = Connection::connect_to_env()?;

    let event_queue: EventQueue<state::State> = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();
    _ = display.get_registry(&qhandle, ());

    Ok(event_queue)
}

pub async fn init_wayland(
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    target_display: &DisplayParams,
) -> Result<()> {
    let (write, read) = stream.split();

    let write = Arc::new(Mutex::new(write));

    // Initialize Wayland
    let mut event_queue = init_queue()?;

    println!("Starting the example window app, press <ESC> to quit.");

    // TODO: Only for testing since actual display will be with size of target
    // Used for modifying tmp display cords into output resolution
    let resolution_rate = TMP_DISPLAY_WIDTH as f64 / target_display.width() as f64;

    // Initialize incoming messages handler
    let incoming_write = write.clone();
    let incoming = tokio::spawn(async move {
        _ = handle_incoming(read, incoming_write).await;
    });

    let mut state = State::new(resolution_rate);
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
                            Answer::ClipboardContents(content) => {
                                let Ok(mut ctx) = ClipboardContext::new() else {
                                    warn!("Failed to get clipboard context");
                                    continue;
                                };

                                match ctx.set_contents(content) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        warn!("Failed to set clipboard content");
                                        continue;
                                    }
                                }
                            }
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
