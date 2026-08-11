use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::audio;
use crate::error::MorseError;

pub fn translate_to_morse(file: &String) -> Result<(), MorseError> {
    let input_text =
        fs::read_to_string(file).map_err(|_| MorseError::InvalidFile(PathBuf::from(file)))?;

    let code = input_text
        .split_whitespace()
        .map(translate_word)
        .collect::<Result<Vec<_>, _>>()?
        .join(" / ");

    let mut output_file = File::create("MORSE").unwrap(); // store the code in text
    write!(output_file, "{code}").unwrap();

    println!("MORSE OUTPUT: #{code}#");
    let _ = audio::create_audio(&code);
    Ok(())
}

fn translate_word(word: &str) -> Result<String, MorseError> {
    let letters = word
        .chars()
        .map(|c| match_morse(c).ok_or(MorseError::InvalidChar(c)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(letters.join(" "))
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
        '.' => Some(".-.-.-"),
        ',' => Some("--..--"),
        ':' => Some("---..."),
        '?' => Some("..--.."),
        '\'' => Some(".----."),
        '-' => Some("-....-"),
        '/' => Some("-..-."),
        '(' => Some("-.--."),
        ')' => Some("-.--.-"),
        '"' => Some(".-..-."),
        '=' => Some("-...-"),
        '+' => Some(".-.-."),
        '@' => Some(".--.-."),
        '!' => Some("-.-.--"),
        '&' => Some(".-..."),
        ';' => Some("-.-.-."),
        '_' => Some("..--.-"),
        '$' => Some("...-..-"),
        _ => None,
    }
}
