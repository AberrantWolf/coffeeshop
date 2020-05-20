use crate::coffee_audio::AudioController;
use crate::coffee_chat::ChatController;
use crate::coffee_network::NetworkController;

use uuid::Uuid;

// #[derive(Clone)]
pub struct CoffeeAppContext {
    local_id: Uuid,
    net_controller: Option<NetworkController>,
    chat_controller: Option<ChatController>,
    audio_controller: Option<AudioController>,
}

impl CoffeeAppContext {
    pub fn new() -> Self {
        CoffeeAppContext {
            local_id: Uuid::new_v4(),
            net_controller: Option::None,
            chat_controller: Option::None,
            audio_controller: Option::None,
        }
    }

    pub fn start_chat(&mut self, username: String) {
        self.chat_controller = Some(ChatController::new(self.local_id, username));
    }

    pub fn _construct(port_num: u16, username: String) -> Self {
        let net_controller =
            NetworkController::new_with_port_and_username(port_num, username.clone());
        let local_id = Uuid::new_v4();
        CoffeeAppContext {
            local_id,
            net_controller: Some(net_controller),
            chat_controller: Some(ChatController::new(local_id, username)),
            audio_controller: Some(AudioController::new()),
        }
    }

    pub fn _get_net_controller(&self) -> Result<&NetworkController, &str> {
        if let Some(con) = &self.net_controller {
            Ok(&con)
        } else {
            Err("Network controller has not been initialized")
        }
    }

    pub fn _get_audio_controller(&self) -> Result<&AudioController, &str> {
        if let Some(con) = &self.audio_controller {
            Ok(&con)
        } else {
            Err("Audio controller has not been initialized")
        }
    }
}
