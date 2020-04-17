/// An AudioChunk represens _some_ amount of audio samples, with a channel
/// count and sample rate. This could be anything from a single sample to
/// a whole autio file stored in memory.
pub struct AudioChunk {
    channel_count: u32,
    sample_rate: u32,
    buffer: Vec<i16>,
}

impl AudioChunk {
    pub fn new() -> Self {
        AudioChunk {
            channel_count: 1,
            sample_rate: 44_100,
            buffer: vec![],
        }
    }
    pub fn new_from_data(channel_count: u32, sample_rate: u32, buffer: Vec<i16>) -> Self {
        AudioChunk {
            channel_count,
            sample_rate,
            buffer,
        }
    }

    pub fn channel_count(&self) -> u32 {
        self.channel_count
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn buffer(&self) -> &Vec<i16> {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Vec<i16> {
        &mut self.buffer
    }
}

/// An AudioLayer takes an AudioChunk as input and modifies the chunk.
/// Your original data is lost/changed by this process
trait AudioLayer {
    fn modulate_chunk(chunk: &mut AudioChunk);
}
