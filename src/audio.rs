use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{self, Command};

const DIT: &str = "dit.ogg";
const DAH: &str = "dah.ogg";
const SPACE_LETTERS: &str = "space_letters.ogg";
const SPACE_WORDS: &str = "space_words.ogg";

const ASSETS: [(&str, &[u8]); 4] = [
    (DIT, include_bytes!("../audio/dit.ogg")),
    (DAH, include_bytes!("../audio/dah.ogg")),
    (SPACE_LETTERS, include_bytes!("../audio/space_letters.ogg")),
    (SPACE_WORDS, include_bytes!("../audio/space_words.ogg")),
];

pub fn create_audio(code: &str) -> Result<(), Box<dyn Error>> {
    let workspace = env::temp_dir().join(format!("morsel-{}", process::id()));
    fs::create_dir_all(&workspace)?;

    let result = concat_assets(code, &workspace);
    let _ = fs::remove_dir_all(&workspace);

    result
}

fn concat_assets(code: &str, workspace: &Path) -> Result<(), Box<dyn Error>> {
    for (name, data) in ASSETS {
        fs::write(workspace.join(name), data)?;
    }

    let script = workspace.join("INSTRUCTIONS");
    let mut instructions = File::create(&script)?;
    for c in code.chars() {
        let asset = match c {
            '.' => DIT,
            '-' => DAH,
            ' ' => SPACE_LETTERS,
            '/' => SPACE_WORDS,
            _ => continue,
        };
        writeln!(instructions, "file '{asset}'")?;
    }

    println!("INFO: Constructing the Morse code audio...");
    let mut cmd = Command::new("ffmpeg");
    cmd.args([
        "-f".as_ref(),
        "concat".as_ref(),
        "-i".as_ref(),
        script.as_os_str(),
        "-c".as_ref(),
        "copy".as_ref(),
        "-y".as_ref(),
        "MORSE.ogg".as_ref(),
    ]);

    let output = cmd.output()?;
    if output.status.success() {
        println!("INFO: Morse code audio is created successfully!");
    } else {
        eprintln!(
            "ERROR: FFmpeg command failed with status: {}",
            output.status
        );
    }

    Ok(())
}
