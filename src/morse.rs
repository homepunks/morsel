use std::io::Write;
use std::fs::{self, File};

use crate::MorseError;

pub fn translate_to_morse(file: &String) -> Result<(), MorseError> {
    let mut buf: Vec<&str> = Vec::new();
    
    let input_text = match fs::read_to_string(file) {
	Ok(plain_text) => plain_text,
	Err(_) => {
	    eprintln!("ERROR: Could not read file {file}. Are you sure you've supplied a valid text file?");
	    return Err(MorseError::InvalidFile)
	}
    };
    
    for c in input_text.chars() {
	if !c.is_ascii_alphanumeric() && !c.is_whitespace() {
	    eprintln!("ERROR: Could not process symbol {c}");
	    eprintln!("ERROR: Only use ASCII alphanumerics for international Morse code.");
	    return Err(MorseError::InvalidChar)
	}

	let c_morse = match match_morse(c) {
	    Some(c) => c,
	    None => continue,
	};
	
	buf.push(c_morse);
	buf.push(" ");
    }
    buf.truncate(buf.len() - 3); // i couldn't think of a smarter solution yet

    let mut output_file = File::create("MORSE").unwrap();
    for c in &buf {
	write!(output_file, "{}", c).unwrap();
    }
    
    let write: String = buf.into_iter().collect();
    println!("MORSE OUTPUT: {}", write);
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
	' ' | '\n' | '\t' => Some("/"),
        _ => None,
    }
}
