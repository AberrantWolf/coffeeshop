extern crate sfml;
extern crate tui;

mod file_source;
mod user_interface;

use sfml::audio::{SoundStatus, SoundStreamPlayer};

use std::error::Error;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // XXX: I'm playing with UI stuff, so just ignore this for now...
    // let result = user_interface::run_ui();

    // Show the default audio input device so we know we have something, at least
    let default_audio_in_device = sfml::audio::capture::default_device();
    println!("Default Audio Input Device: {}", default_audio_in_device);

    let mut stream = file_source::FileSource::new("resources/stereo_test.ogg");
    let mut player = SoundStreamPlayer::new(&mut stream);
    player.play();

    while player.status() == SoundStatus::Playing {
        // Display the playing position
        println!("\rPlaying... {:.2}", player.playing_offset().as_seconds());
        let _ = std::io::stdout().flush();

        ::std::thread::sleep(::std::time::Duration::from_millis(100));
    }

    println!("Goodbye, world! {:?}", player.status());

    Ok(())
}
