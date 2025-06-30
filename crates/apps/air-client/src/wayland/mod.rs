mod error;
mod state;

pub use error::{Error, Result};

use futures::StreamExt;
use lib_models::DisplayParams;
use state::State;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use wayland_client::{Connection, EventQueue};

/// Used for temporary display
const TMP_DISPLAY_WIDTH: u32 = 1280;
/// Used for temporary display
const TMP_DISPLAY_HEIGHT: u32 = 720;

pub async fn init_wayland(
    stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    target_display: &DisplayParams,
) -> Result<()> {
    let (mut write, mut _read) = stream.split();

    // Initialize Wayland connection
    let conn = Connection::connect_to_env().unwrap();

    let mut event_queue: EventQueue<state::State> = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qhandle, ());

    println!("Starting the example window app, press <ESC> to quit.");

    // TODO: Only for testing since actual display will be with size of target
    // Used for modifying tmp display cords into output resolution
    let resolution_rate = TMP_DISPLAY_WIDTH as f64 / target_display.width() as f64;

    let mut state = State::default();

    while state.is_running() {
        event_queue.blocking_dispatch(&mut state)?;
        state.handle(&mut write, resolution_rate).await?;
    }

    Ok(())
}
