use air_client::{config, Dispatcher, DispatcherTrait, EventHandler, Result};
use lib_protocol::handler::Handler;
use lib_quic::{client::QuicClient, tls::TlsLoader};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};
use tracing::info;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    air_client::init()?;

    TlsLoader::init_provider();
    TlsLoader::debug_cipher_info();

    let address = config().ADDRESS;

    let client = QuicClient::new(Path::new("./certs/cert.pem"))
        .await
        .unwrap();
    let conn = client.connect(address, "localhost").await.unwrap();

    info!("✅ Client connected to server");

    let is_running = Arc::new(AtomicBool::new(false));
    let event_handler = EventHandler::new(conn);
    let mut dispatcher = Dispatcher::init(event_handler.sender(), is_running.clone()).unwrap();

    let _command_tx = event_handler.sender();

    let event_handler = tokio::spawn(event_handler.run_loop());
    let dispatcher_handle = thread::spawn(move || dispatcher.run().unwrap());

    _ = tokio::signal::ctrl_c().await;
    is_running.store(false, Ordering::Relaxed);

    _ = dispatcher_handle.join();
    event_handler.abort();

    info!("✅ Client disconnected from server");

    Ok(())
}
