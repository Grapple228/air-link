use std::path::Path;

use air_server::{config, InputSimulator, Result, Simulator};
use lib_models::Command;
use lib_quic::{
    datagram::{Datagram, ReceivedDatagram},
    quinn,
    server::QuicServer,
    tls::TlsLoader,
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    air_server::init()?;

    TlsLoader::init_provider();
    TlsLoader::debug_cipher_info();

    let address = config().ADDRESS;

    info!("🔊 Server starting on {}", address);

    let server = QuicServer::new(
        address,
        Path::new("./certs/cert.pem"),
        Path::new("./certs/key.pem"),
    )
    .await
    .unwrap();

    let server_addr = server.local_addr().unwrap();
    info!("✅ Server listening on {}", server_addr);

    tokio::select! {
         server = server.run(handler, ()) => {
             if let Err(e) = server {
                 error!("Server error: {e}");
             }
         },
         _ctrlc = tokio::signal::ctrl_c() => {
             info!("Received ctrl-c")
         }
    }

    Ok(())
}

struct Handler {
    datagram: Datagram,
}

impl Handler {
    pub fn new(connection: quinn::Connection) -> Self {
        let datagram = Datagram::new(connection);

        Self { datagram }
    }

    async fn receive(&self) -> Option<ReceivedDatagram> {
        self.datagram.receive().await
    }

    fn process(&mut self, input: &mut Simulator, command: Command) -> Result<()> {
        // info!("Reveived command: {:?}", command);

        match command {
            Command::SetMouse { x, y } => input.set_mouse(x, y).unwrap(),
            Command::MoveMouse { x, y } => input.move_mouse(x, y).unwrap(),
            Command::MouseButtonPressed(button) => input.mouse_press(button).unwrap(),
            Command::MouseButtonReleased(button) => input.mouse_release(button).unwrap(),
            Command::MouseScroll(scroll) => input.scroll(scroll).unwrap(),
            Command::InputText(text) => input.text(&text).unwrap(),
            Command::KeyPressed(keycode) => input.key_press(keycode).unwrap(),
            Command::KeyReleased(keycode) => input.key_release(keycode).unwrap(),
            Command::SetClipboard(_) => todo!(),
        }

        Ok(())
    }
}

async fn handler(connection: quinn::Connection, _state: ()) -> lib_quic::Result<()> {
    let address = connection.remote_address();
    info!("New connection: {}", address);

    println!("DISPLAY: {:?}", std::env::var("DISPLAY"));
    println!("WAYLAND_DISPLAY: {:?}", std::env::var("WAYLAND_DISPLAY"));

    let mut input = Simulator::new().unwrap();
    let mut handler = Handler::new(connection);

    while let Some(data) = handler.receive().await {
        let Ok(command) = lib_codec::decode::<Command>(&data.data) else {
            error!("Decode command failed");
            continue;
        };

        if let Err(e) = handler.process(&mut input, command) {
            error!("Error occured: {}", e);
            break;
        }
    }

    info!("👋 Client disconnected: {}", address);

    Ok(())
}
