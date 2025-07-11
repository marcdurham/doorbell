use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    // Get the default audio output device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create a new sink (audio playback handler)
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Open the WAV file
    let file = File::open("sound.wav").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Play the sound
    sink.append(source);

    // Keep the program running until the sound finishes
    sink.sleep_until_end();
}

