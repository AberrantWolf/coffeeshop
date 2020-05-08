pub mod layers;
pub mod sources;
pub mod types;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct AudioController {
    inner: Arc<RwLock<AudioController_Inner>>,
}

#[derive(Debug)]
struct AudioController_Inner {}

impl AudioController {
    pub fn new() -> Self {
        AudioController {
            inner: Arc::new(RwLock::new(AudioController_Inner {})),
        }
    }
}

struct AudioNetReceiver {}
