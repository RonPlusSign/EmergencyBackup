use std::io::Cursor;
use rodio::{Decoder, OutputStream, Source};
use std::thread::sleep;
use std::time::Duration;

pub fn use_audio() {

    let audio_data = include_bytes!("../sounds/activation.wav");


    // Set up the audio output
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Decode and play the embedded audio
    let cursor = Cursor::new(audio_data.as_ref());

    let source = Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    sleep(Duration::from_secs(2));

}