mod error;

pub use error::{Error, Result};

use lib_models::DisplayParams;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub async fn init_windows(
    _stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    _target_display: &DisplayParams,
) -> Result<()> {
    todo!("windows not implemented yet");
}
