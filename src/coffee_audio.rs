pub mod layers;
pub mod sources;
pub mod types;

use tokio::sync::{broadcast, mpsc};
type StdRwLock<T> = std::sync::RwLock<T>;
type StdArc<T> = std::sync::Arc<T>;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream};

use itertools::Itertools;

use ringbuf::RingBuffer;

#[derive(Clone)]
pub struct AudioController {
    inner: StdArc<StdRwLock<AudioControllerInner>>,
}

#[derive(Clone)]
pub enum AudioMessage {}

// #[derive(Debug)]
struct AudioControllerInner {
    host: Host,
    input_device: Device,
    input_stream: Stream,
    output_device: Device,
    output_stream: Stream,
    broadcast_tx: broadcast::Sender<AudioMessage>,
    mpsc_tx: mpsc::Sender<AudioMessage>,
}

// TODO: ALL of this stuff needs to be configurable...
// Not exclusively, but including:
// * What input to use
// * What output to use
// * How to convert incoming audio to match output format
// * Buffer size needs to be big enough for formats (44kHz can be smaller than 192kHz)
impl AudioController {
    pub fn new() -> Self {
        let host = cpal::default_host();

        // Input
        let input_device = host
            .default_input_device()
            .expect("Failed to find a default input device.");
        let input_config = input_device
            .default_input_config()
            .expect("Error getting default input config");
        println!("Default input config: {:?}", input_config);

        // Output
        let output_device = host
            .default_output_device()
            .expect("Failed to find a default output device.");
        let output_config = output_device
            .default_output_config()
            .expect("Error getting default output config");
        println!("Default output config: {:?}", output_config);

        let (btx, _brx) = broadcast::channel::<AudioMessage>(16);
        let (mtx, mrx) = mpsc::channel::<AudioMessage>(100);

        let input_ring = RingBuffer::<f32>::new(8192);
        let output_ring = RingBuffer::<f32>::new(32_768);

        let (mut in_producer, mut in_consumer) = input_ring.split();

        // NOTE: Maybe we don't want a ring buffer, but rather we collect
        // some number of samples and then send a packet for playback?
        let in_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mono = data
                .iter()
                .step_by(2)
                .interleave(data.iter().step_by(2))
                .cloned()
                .collect_vec();
            let count = in_producer.push_slice(&mono[..]);
            if count < data.len() {
                println!("push overrun");
            }
        };

        let out_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let size = in_consumer.pop_slice(data);
            if size < data.len() {
                println!("Producer fell behind");
            }
        };

        let input_stream = input_device
            .build_input_stream(&input_config.into(), in_fn, |err: cpal::StreamError| {
                println!("Recording error: {}", err)
            })
            .expect("Error creating input stream");

        let output_stream = output_device
            .build_output_stream(&output_config.into(), out_fn, |err: cpal::StreamError| {
                println!("Playing error: {}", err)
            })
            .expect("Error creating output stream");

        input_stream.play().expect("Error starting input stream");
        output_stream.play().expect("Error starting output stream");

        let ac = AudioController {
            inner: StdArc::new(StdRwLock::new(AudioControllerInner {
                host,
                input_device,
                input_stream,
                output_device,
                output_stream,
                broadcast_tx: btx,
                mpsc_tx: mtx,
            })),
        };

        println!("Audio stuff created...");

        // ac.start_mpsc(mrx);

        ac
    }

    pub fn get_input_config(&self) -> cpal::StreamConfig {
        self.inner
            .read()
            .expect("lock err")
            .input_device
            .default_input_config()
            .expect("Error getting default input config")
            .into()
    }

    pub fn get_output_config(&self) -> cpal::StreamConfig {
        self.inner
            .read()
            .expect("lock err")
            .output_device
            .default_output_config()
            .expect("Error getting default input config")
            .into()
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
