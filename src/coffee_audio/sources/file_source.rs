use std::fmt;

use sfml::audio::SoundBuffer;
use sfml::audio::SoundStream;

use crate::coffee_audio::types::AudioChunk;

const SAMPLES_PER_CHUNK: usize = 20000;

pub struct FileSource {
    data: AudioChunk,
    play_head: usize,
}

impl fmt::Debug for FileSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("BufferStream")
            .field("channel_count", &self.data.channel_count())
            .field("sample_rate", &self.data.sample_rate())
            .field("buffer", &format!("Vec<i16>: {}", self.data.buffer().len()))
            .field("play_head", &self.play_head)
            .finish()
    }
}

impl FileSource {
    // TODO: Okay, this should probably log an error or return invalid or
    // soemthing if it doesn't manage to properly load the file rather than
    // failing silently... literally.
    pub fn new(file: &str) -> Self {
        // Load a sample from a file (or an empty sample if file couldn't be loaded)
        match SoundBuffer::from_file(file) {
            Some(b) => FileSource {
                data: AudioChunk::new_from_data(
                    b.channel_count(),
                    b.sample_rate(),
                    b.samples().to_vec(),
                ),
                play_head: 0usize,
            },
            None => FileSource {
                data: AudioChunk::new(),
                play_head: 0usize,
            },
        }
    }
}

impl SoundStream for FileSource {
    // Seek does nothing, this stream just plays until it ends
    fn seek(&mut self, _: sfml::system::Time) {}

    fn get_data(&mut self) -> (&mut [i16], bool) {
        // Calculate remaining samples
        let remaining = self.data.buffer().len() - self.play_head;
        let (size, keep_playing) = if remaining >= SAMPLES_PER_CHUNK {
            (SAMPLES_PER_CHUNK, true)
        } else {
            (remaining, false)
        };

        // Grab a slice of samples to play
        let end = self.play_head + size;
        let sl = &mut self.data.buffer_mut()[self.play_head..end];

        // Move the play head forward
        self.play_head += size;

        (sl, keep_playing)
    }

    fn channel_count(&self) -> u32 {
        self.data.channel_count()
    }

    fn sample_rate(&self) -> u32 {
        self.data.sample_rate()
    }
}
