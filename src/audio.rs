use std::process::Command;

pub const DIT: &str = "dit.ogg";
pub const DAH: &str = "dah.ogg";
pub const SPACE_LETTERS: &str = "space_letters.ogg";
pub const SPACE_WORDS: &str = "space_words.ogg";

pub fn create_audio() -> Result<(), Box<dyn std::error::Error>> {
    let audio_script = "audio/INSTRUCTIONS";

    println!("INFO: Constructing the Morse code audio...");
    let mut cmd = Command::new("ffmpeg");   
    cmd.args([
	"-f", "concat",
	"-i", audio_script,
	"-c", "copy",
	"-y",
	"MORSE.ogg",
    ]);

    let output = cmd.output()?;
    if output.status.success() {
	println!("INFO: Morse code audio is created successfully!");
    } else {
	eprintln!("ERROR: FFmpeg command failed with status: {}", output.status);
	// eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        // eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
