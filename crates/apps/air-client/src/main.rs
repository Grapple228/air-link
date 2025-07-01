use air_client::Result;
use lib_models::DisplayParams;
use tokio_tungstenite::connect_async_with_config;

#[tokio::main]
#[cfg(target_os = "linux")]
async fn main() -> Result<()> {
    // Подключение к серверу

    let (ws_stream, _) = connect_async_with_config("ws://192.168.0.150:5555", None, true).await?;

    println!("Connected to the WebSocket server");

    // Получаем параметры дисплея
    // todo Get from stream
    let display = DisplayParams::new(2560, 1440);

    // Инициализируем дисплей
    air_client::init_wayland(ws_stream, &display).await?;

    Ok(())
}

#[tokio::main]
#[cfg(target_os = "windows")]
async fn main() -> Result<()> {
    // Подключение к серверу
    let (ws_stream, _) = connect_async_with_config("ws://192.168.0.150:5555", None, true).await?;

    println!("Connected to the WebSocket server");

    // Получаем параметры дисплея
    // todo Get from stream
    let display = DisplayParams::new(2560, 1440);

    // Инициализируем дисплей
    air_client::init_windows(ws_stream, &display).await?;

    Ok(())
}
