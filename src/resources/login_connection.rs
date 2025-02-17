use bevy::prelude::Resource;
use rose_game_common::messages::{client::ClientMessage, server::ServerMessage};

#[derive(Resource)]
pub struct LoginConnection {
    pub client_message_tx: tokio::sync::mpsc::UnboundedSender<ClientMessage>,
    pub server_message_rx: crossbeam_channel::Receiver<ServerMessage>,
}

impl LoginConnection {
    pub fn new(
        client_message_tx: tokio::sync::mpsc::UnboundedSender<ClientMessage>,
        server_message_rx: crossbeam_channel::Receiver<ServerMessage>,
    ) -> Self {
        client_message_tx
            .send(ClientMessage::ConnectionRequest(Default::default()))
            .ok();

        Self {
            client_message_tx,
            server_message_rx,
        }
    }
}
