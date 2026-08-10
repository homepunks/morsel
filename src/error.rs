use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum MorseError {
    InvalidChar(char),
    InvalidFile(PathBuf),
}

impl fmt::Display for MorseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChar(c) => {
                write!(
                    f,
                    "could not process symbol {c:?}: only ASCII letters and digits are supported"
                )
            }
            Self::InvalidFile(path) => {
                write!(
                    f,
                    "could not read {}: are you sure you've supplied a valid text file?",
                    path.display()
                )
            }
        }
    }
}

impl Error for MorseError {}
