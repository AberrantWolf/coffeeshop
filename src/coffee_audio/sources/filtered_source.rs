// use sfml::audio::SoundStream;

// use crate::coffee_audio::types::{AudioChunk, AudioLayer};

// pub struct FilteredSource<S: SoundStream> {
//     base: Box<S>,
//     filters: Vec<Box<dyn AudioLayer>>,
//     chunk: AudioChunk,
// }

// impl<T: SoundStream> FilteredSource<T> {
//     pub fn new(base: T) -> Self {
//         FilteredSource {
//             base: Box::new(base),
//             filters: vec![],
//             chunk: AudioChunk::new(),
//         }
//     }

//     pub fn add_filter<A: 'static + AudioLayer>(&mut self, filter: A) {
//         self.filters.push(Box::from(filter));
//     }
// }

// impl<T: SoundStream> SoundStream for FilteredSource<T> {
//     fn get_data(&mut self) -> (&mut [i16], bool) {
//         let channels = self.base.channel_count();
//         let rate = self.base.sample_rate();
//         let (data, has_more) = self.base.get_data();
//         self.chunk = AudioChunk::new_from_data(channels, rate, data.to_vec());
//         for f in self.filters.iter_mut() {
//             f.modulate_chunk(&mut self.chunk);
//         }

//         (&mut self.chunk.buffer_mut()[..], has_more)
//     }
//     fn seek(&mut self, time: sfml::system::Time) {
//         self.base.seek(time)
//     }
//     fn channel_count(&self) -> u32 {
//         // TODO: Precalculate final filter's channel count
//         // NOTE: Maybe filters should return whether they change the channel count?
//         self.base.channel_count()
//     }
//     fn sample_rate(&self) -> u32 {
//         // TODO: Precalculate final filter's sample rate
//         // NOTE: Maybe filters should return whether they change the sample rate?
//         self.base.sample_rate()
//     }
// }
