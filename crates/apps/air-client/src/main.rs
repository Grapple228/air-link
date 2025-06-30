use air_client::{init_wayland, Result};
use lib_models::DisplayParams;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() -> Result<()> {
    // Подключение к серверу
    let (mut ws_stream, _) = connect_async("ws://192.168.0.150:5555").await?;

    println!("Connected to the WebSocket server");

    // Получаем параметры дисплея
    // todo Get from stream
    let display = DisplayParams::new(2560, 1440);

    // Инициализируем дисплей
    init_wayland(&mut ws_stream, &display).await?;

    ws_stream.close(None).await?;
    println!("WebSocket connection closed");

    Ok(())
}
