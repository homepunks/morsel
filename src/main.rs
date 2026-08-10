use std::env::args;

mod audio;
mod error;
mod morse;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        match morse::translate_to_morse(&args[1]) {
            Ok(()) => println!("INFO: Done! File MORSE is created!"),
            Err(e) => println!("ERROR: {e}"),
        };
    } else {
        println!("INFO: Not enough arguments supplied.");
        println!("INFO: Example usage: ./target/debug/morsel plain.txt");
    }
}
