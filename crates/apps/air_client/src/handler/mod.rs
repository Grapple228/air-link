use lib_protocol::handler::Handler;

mod error;

pub use error::{Error, Result};
use lib_quic::{datagram::Datagram, quinn, Ssrc};

pub enum HandlerCommand {
    Command(lib_models::Command),
}

pub struct EventHandler {
    #[allow(unused)]
    connection: quinn::Connection,
    datagram: Datagram,
    encode_buf: [u8; 1024],
    command_rx: flume::Receiver<HandlerCommand>,
    command_tx: flume::Sender<HandlerCommand>,
}

impl EventHandler {
    pub fn new(connection: quinn::Connection) -> Self {
        let (command_tx, command_rx) = flume::bounded(1000);

        let datagram = Datagram::new(connection.clone());

        Self {
            connection,
            datagram,
            encode_buf: [0; 1024],
            command_tx,
            command_rx,
        }
    }

    pub fn sender(&self) -> flume::Sender<HandlerCommand> {
        self.command_tx.clone()
    }
}

impl Handler for EventHandler {
    type Error = Error;

    type Message = HandlerCommand;

    async fn handle(&mut self, message: Self::Message) -> Result<bool> {
        let _len = match message {
            HandlerCommand::Command(command) => {
                let encoded = lib_codec::encode_to_stack(&command, &mut self.encode_buf).unwrap();

                if let Err(e) = self.datagram.send(
                    &self.encode_buf[0..encoded],
                    0,
                    lib_quic::datagram::DatagramType::Command,
                    Ssrc(1),
                ) {
                    tracing::error!("Error occured in send: {}", e)
                }
            }
        };

        Ok(false)
    }

    async fn receive(&mut self) -> Result<Option<Self::Message>> {
        match self.command_rx.recv_async().await {
            Ok(command) => Ok(Some(command)),
            Err(e) => {
                tracing::error!(
                    "Error occured while receiving in {}: {e}",
                    Self::display_name()
                );
                Ok(None)
            }
        }
    }
}
