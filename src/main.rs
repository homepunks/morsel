use std::env::args;

mod morse;

enum MorseError {
    InvalidChar,
    InvalidFile,
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
	match morse::translate_to_morse(&args[1]) {
	    Ok(()) => println!("INFO: Done! File MORSE is created!"),
	    Err(_) => println!("ERROR: Aborting..."),
	};
    } else {
	println!("INFO: Not enough arguments supplied.");
	println!("INFO: Example usage: ./target/debug/morse plain.txt");
    }
}
