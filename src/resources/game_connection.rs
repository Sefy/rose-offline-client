use rose_game_common::messages::{
    client::{ClientMessage, ConnectionRequest},
    server::ServerMessage,
};

pub struct GameConnection {
    pub client_message_tx: tokio::sync::mpsc::UnboundedSender<ClientMessage>,
    pub server_message_rx: crossbeam_channel::Receiver<ServerMessage>,
}

impl GameConnection {
    pub fn new(
        client_message_tx: tokio::sync::mpsc::UnboundedSender<ClientMessage>,
        server_message_rx: crossbeam_channel::Receiver<ServerMessage>,
        login_token: u32,
        password_md5: String,
    ) -> Self {
        client_message_tx
            .send(ClientMessage::ConnectionRequest(ConnectionRequest {
                login_token,
                password_md5,
            }))
            .ok();

        Self {
            client_message_tx,
            server_message_rx,
        }
    }
}
