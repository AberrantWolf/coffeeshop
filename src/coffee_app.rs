use crate::coffee_audio::AudioController;
use crate::coffee_network::NetworkController;

#[derive(Clone)]
pub struct CoffeeAppContext {
    net_controller: NetworkController,
    audio_controller: AudioController,
}

impl CoffeeAppContext {
    pub fn construct(port_num: u16, username: String) -> Self {
        let net_controller = NetworkController::new_with_port_and_username(port_num, username);
        CoffeeAppContext {
            net_controller,
            audio_controller: AudioController::new(),
        }
    }

    pub fn get_net_controller(&self) -> &NetworkController {
        &self.net_controller
    }

    pub fn get_audio_controller(&self) -> &AudioController {
        &self.audio_controller
    }
}
