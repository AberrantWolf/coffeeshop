pub mod layers;
pub mod sources;
pub mod types;

use std::sync::Arc;
use tokio::sync::RwLock;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host};

#[derive(Clone)]
pub struct AudioController {
    inner: Arc<RwLock<AudioController_Inner>>,
}

// TODO: Need a process to listen to the input device and stream that to
// the network host

// #[derive(Debug)]
struct AudioController_Inner {
    host: Host,
    output_device: Device,
    input_device: Device,
}

impl AudioController {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let output_device = host
            .default_output_device()
            .expect("Failed to find a default output device.");
        let input_device = host
            .default_input_device()
            .expect("Failed to find a default input device.");

        AudioController {
            inner: Arc::new(RwLock::new(AudioController_Inner {
                host,
                output_device,
                input_device,
            })),
        }
    }
}

// One of these gets made per-remote-peer, receives and provides bytes
// to a stream generated from the default output device on the controller.
struct AudioNetReceiver {
    // Reference lib here:
// https://github.com/RustAudio/cpal/blob/master/src/lib.rs
// Need a stream to play on creation
}

// Audio controller listens, sends voice packets to the net controller?
//   * What if broadcasts directly to all listeners for local input?
//   * Ignore the net controller entirely for voice packets...!
// Net peers get audio packets from remote and (again) send directly to
// the audio controller -- no need to filter through the net controller.
// ...and anyway, a brodcast channel goes out to ALL listeners...
