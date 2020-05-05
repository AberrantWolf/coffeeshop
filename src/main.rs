mod coffee_audio;
mod coffee_network;
mod user_interface;

// use sfml::audio::{SoundStatus, SoundStreamPlayer};

// use coffee_audio::layers::{PassthroughLayer, SwapLRLayer};
// use coffee_audio::sources::{FileSource, FilteredSource};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // XXX: I'm playing with UI stuff, so just ignore this for now...
    user_interface::start_ui().await;

    // Show the default audio input device so we know we have something, at least
    // let default_audio_in_device = sfml::audio::capture::default_device();
    // println!("Default Audio Input Device: {}", default_audio_in_device);

    // let file_stream = FileSource::new("resources/stereo_test.ogg");
    // let mut stream = FilteredSource::new(file_stream);
    // stream.add_filter(PassthroughLayer {});
    // stream.add_filter(SwapLRLayer {});
    // let mut player = SoundStreamPlayer::new(&mut stream);
    // player.play();

    // while player.status() == SoundStatus::Playing {
    //     // Display the playing position
    //     println!("\rPlaying... {:.2}", player.playing_offset().as_seconds());
    //     let _ = std::io::stdout().flush();

    //     ::std::thread::sleep(::std::time::Duration::from_millis(100));
    // }

    // println!("Goodbye, world! {:?}", player.status());

    Ok(())
}
