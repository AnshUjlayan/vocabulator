// Audio playback module
// Handles sound effects for correct/wrong answers, marking, and menu navigation

use std::io::Cursor;

/// Play the correct answer sound effect
pub fn play_correct_sound() {
    std::thread::spawn(|| {
        if let Err(e) = play_sound_internal(include_bytes!("../assets/sounds/correct.mp3").to_vec()) {
            eprintln!("Failed to play correct sound: {}", e);
        }
    });
}

/// Play the wrong answer sound effect
pub fn play_wrong_sound() {
    std::thread::spawn(|| {
        if let Err(e) = play_sound_internal(include_bytes!("../assets/sounds/wrong.mp3").to_vec()) {
            eprintln!("Failed to play wrong sound: {}", e);
        }
    });
}

/// Play the mark/bookmark sound effect
pub fn play_mark_sound() {
    std::thread::spawn(|| {
        if let Err(e) = play_sound_internal(include_bytes!("../assets/sounds/mark.mp3").to_vec()) {
            eprintln!("Failed to play mark sound: {}", e);
        }
    });
}

/// Play the menu navigation sound effect
pub fn play_menu_sound() {
    std::thread::spawn(|| {
        if let Err(e) = play_sound_internal(include_bytes!("../assets/sounds/gta-menu.mp3").to_vec()) {
            eprintln!("Failed to play menu sound: {}", e);
        }
    });
}

fn play_sound_internal(audio_data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    use rodio::{Decoder, OutputStream, Sink};
    
    // Get an output stream handle
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    // Create a cursor from the embedded audio data
    let cursor = Cursor::new(audio_data);
    
    // Decode the audio
    let source = Decoder::new(cursor)?;
    
    // Play the sound
    sink.append(source);
    sink.sleep_until_end();
    
    Ok(())
}
