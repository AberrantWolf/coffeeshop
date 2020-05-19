use crate::coffee_audio::AudioController;
use crate::coffee_chat::ChatController;
use crate::coffee_network::NetworkController;

#[derive(Clone)]
pub struct CoffeeAppContext {
    net_controller: Option<NetworkController>,
    chat_controller: Option<ChatController>,
    audio_controller: Option<AudioController>,
}

impl CoffeeAppContext {
    pub fn new() -> Self {
        CoffeeAppContext {
            net_controller: Option::None,
            chat_controller: Option::None,
            audio_controller: Option::None,
        }
    }

    pub fn construct(port_num: u16, username: String) -> Self {
        let net_controller = NetworkController::new_with_port_and_username(port_num, username);
        CoffeeAppContext {
            net_controller: Some(net_controller),
            chat_controller: Some(ChatController::new()),
            audio_controller: Some(AudioController::new()),
        }
    }

    pub fn get_net_controller(&self) -> Result<&NetworkController, &str> {
        if let Some(con) = &self.net_controller {
            Ok(&con)
        } else {
            Err("Network controller has not been initialized")
        }
    }

    pub fn get_audio_controller(&self) -> Result<&AudioController, &str> {
        if let Some(con) = &self.audio_controller {
            Ok(&con)
        } else {
            Err("Audio controller has not been initialized")
        }
    }
}
