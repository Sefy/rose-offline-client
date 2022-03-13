use bevy::prelude::{Commands, Res, ResMut, State};
use rose_game_common::messages::{client::ClientMessage, server::ServerMessage};
use rose_network_common::ConnectionError;

use crate::resources::{AppState, CharacterList, WorldConnection};

pub fn world_connection_system(
    mut commands: Commands,
    world_connection: Option<Res<WorldConnection>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if world_connection.is_none() {
        return;
    }

    let world_connection = world_connection.unwrap();
    let result: Result<(), anyhow::Error> = loop {
        match world_connection.server_message_rx.try_recv() {
            Ok(ServerMessage::ConnectionResponse(response)) => match response {
                Ok(_) => {
                    world_connection
                        .client_message_tx
                        .send(ClientMessage::GetCharacterList)
                        .ok();
                }
                Err(_) => {
                    break Err(ConnectionError::ConnectionLost.into());
                }
            },
            Ok(ServerMessage::CharacterList(characters)) => {
                if !matches!(app_state.current(), AppState::GameCharacterSelect) {
                    app_state.set(AppState::GameCharacterSelect).ok();
                }

                commands.insert_resource(CharacterList { characters });
            }
            // TODO:
            // ServerMessage::CreateCharacter
            // ServerMessage::DeleteCharacter
            // ServerMessage::SelectCharacter
            // ServerMessage::ReturnToCharacterSelect
            Ok(message) => {
                log::warn!("Received unexpected world server message: {:#?}", message);
            }
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                break Err(ConnectionError::ConnectionLost.into());
            }
            Err(crossbeam_channel::TryRecvError::Empty) => break Ok(()),
        }
    };

    if let Err(error) = result {
        // TODO: Store error somewhere to display to user
        log::warn!("World server connection error: {}", error);
        commands.remove_resource::<WorldConnection>();
    }
}
