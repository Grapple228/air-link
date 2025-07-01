mod error;

pub use error::{Error, Result};

use lib_models::DisplayParams;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub async fn init_x11(
    _stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    _target_display: &DisplayParams,
) -> Result<()> {
    todo!("x11 not implemented yet");
}
