mod coffee_app;
mod coffee_audio;
mod coffee_network;
mod coffee_ui;

// use sfml::audio::{SoundStatus, SoundStreamPlayer};

// use coffee_audio::layers::{PassthroughLayer, SwapLRLayer};
// use coffee_audio::sources::{FileSource, FilteredSource};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // The UI module is in charge of constructing the app context binding.
    // This is mostly because we need to know username and port number
    // before starting a server, and the UI shows an initial popup for
    // that purpose. In the future, it would be nice to construct the app
    // context separately and feed it into the UI instead, but that's
    // more work on that code than I'm wanting to put in right now.
    coffee_ui::start_ui();

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
