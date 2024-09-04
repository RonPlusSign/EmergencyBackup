use std::io::Cursor;
use rodio::{Decoder, OutputStream, Source};
use std::thread::sleep;
use std::time::Duration;

enum SoundList{
    Start,
    Correct,
    Completed,
    Stop
}

// Function to convert a string into the corresponding enum variant
fn string_to_command(command: &str) -> Option<SoundList> {
    match command {
        "start" => Some(SoundList::Start),
        "stop" => Some(SoundList::Stop),
        "correct" => Some(SoundList::Correct),
        "completed" => Some(SoundList::Completed),
        _ => None, // Return None for unmatched strings
    }
}

pub fn use_audio(case : &str) {

    //let activation_sound = include_bytes!("../sounds/activation.wav");
    //let correct_sound = include_bytes!("../sounds/correct.wav");
    //let stop_sound = include_bytes!("../sounds/error_stop.wav");


    // Set up the audio output
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sound = match string_to_command(case) {
        Some(SoundList::Start) => include_bytes!("../sounds/start.wav") as &[u8],
        Some(SoundList::Stop) => include_bytes!("../sounds/error_stop.wav") as &[u8],
        Some(SoundList::Correct) => include_bytes!("../sounds/correct.wav") as &[u8],
        Some(SoundList::Completed) => include_bytes!("../sounds/backup_completed.wav") as &[u8],
        None => include_bytes!("../sounds/start.wav") as &[u8], // se non è corretto che si fa?
    };

    // Decode and play the embedded audio
    let cursor = Cursor::new(sound.as_ref());

    let source = Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    sleep(Duration::from_secs(2));

}