use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::audio;
use crate::error::MorseError;

pub fn translate_to_morse(file: &String) -> Result<(), MorseError> {
    let input_text =
        fs::read_to_string(file).map_err(|_| MorseError::InvalidFile(PathBuf::from(file)))?;

    for c in input_text.chars() {
        if !c.is_ascii_alphanumeric() && !c.is_whitespace() {
            return Err(MorseError::InvalidChar(c));
        }
    }

    let code = input_text
        .split_whitespace()
        .map(|word| {
            word.chars()
                .filter_map(match_morse)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" / ");

    let mut output_file = File::create("MORSE").unwrap(); // store the code in text
    let mut sound_build_script = File::create("audio/INSTRUCTIONS").unwrap(); // prepare the instructions for audio creating

    write!(output_file, "{code}").unwrap();

    for c in code.chars() {
        match c {
            '.' => writeln!(sound_build_script, "file '{}'", audio::DIT).unwrap(),
            '-' => writeln!(sound_build_script, "file '{}'", audio::DAH).unwrap(),
            ' ' => writeln!(sound_build_script, "file '{}'", audio::SPACE_LETTERS).unwrap(),
            '/' => writeln!(sound_build_script, "file '{}'", audio::SPACE_WORDS).unwrap(),
            _ => {}
        }
    }

    println!("MORSE OUTPUT: #{code}#");
    let _ = audio::create_audio();
    Ok(())
}

fn match_morse(c: char) -> Option<&'static str> {
    match c.to_ascii_uppercase() {
        'A' => Some(".-"),
        'B' => Some("-..."),
        'C' => Some("-.-."),
        'D' => Some("-.."),
        'E' => Some("."),
        'F' => Some("..-."),
        'G' => Some("--."),
        'H' => Some("...."),
        'I' => Some(".."),
        'J' => Some(".---"),
        'K' => Some("-.-"),
        'L' => Some(".-.."),
        'M' => Some("--"),
        'N' => Some("-."),
        'O' => Some("---"),
        'P' => Some(".--."),
        'Q' => Some("--.-"),
        'R' => Some(".-."),
        'S' => Some("..."),
        'T' => Some("-"),
        'U' => Some("..-"),
        'V' => Some("...-"),
        'W' => Some(".--"),
        'X' => Some("-..-"),
        'Y' => Some("-.--"),
        'Z' => Some("--.."),
        '0' => Some("-----"),
        '1' => Some(".----"),
        '2' => Some("..---"),
        '3' => Some("...--"),
        '4' => Some("....-"),
        '5' => Some("....."),
        '6' => Some("-...."),
        '7' => Some("--..."),
        '8' => Some("---.."),
        '9' => Some("----."),
        _ => None,
    }
}
