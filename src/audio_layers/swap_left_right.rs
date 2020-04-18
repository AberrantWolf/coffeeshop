use crate::coffee_types::{AudioChunk, AudioLayer};

pub struct SwapLRLayer {}

impl AudioLayer for SwapLRLayer {
    fn modulate_chunk(&mut self, chunk: &mut AudioChunk) {
        let data = chunk.buffer_mut();
        for c in data.chunks_exact_mut(2) {
            c.swap(0, 1);
        }
    }
}
