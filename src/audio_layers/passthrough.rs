use crate::coffee_types::{AudioChunk, AudioLayer};

pub struct PassthroughLayer {}

impl AudioLayer for PassthroughLayer {
    fn modulate_chunk(&mut self, _: &mut AudioChunk) {
        // passthrough does not modulate the chunk at all
    }
}
