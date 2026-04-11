use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum EchoErrors {
    CouldNotCreateDataDirectory { err: io::Error },
    UnableToPlayFile { err: rodio::PlayError },
    UnableToDownloadFile { err: Box<dyn Error> },
    UnwantedSelectionIndex { index: usize },
    UnzipError { err: io::Error },
    RemoveFileError { err: io::Error },
    JSONReadError { err: io::Error },
    JSONParseError { err: serde_json::Error },
}

impl Display for EchoErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EchoErrors::CouldNotCreateDataDirectory { err } => {
                writeln!(f, "Error creating data directory: {}", err)
            }
            EchoErrors::UnableToPlayFile { err } => {
                writeln!(f, "Unable to play file: {}", err)
            }
            EchoErrors::UnableToDownloadFile { err } => {
                writeln!(f, "Error: Downloading Sound File: {}", err)
            }
            EchoErrors::UnwantedSelectionIndex { index } => {
                writeln!(f, "Error: Unwanted Selection: {}", index)
            }
            EchoErrors::UnzipError { err } => {
                writeln!(f, "Error: Couldn't extract zip: {}", err)
            }
            EchoErrors::RemoveFileError { err } => {
                writeln!(f, "Error: Couldn't remove zip file: {}", err)
            }
            EchoErrors::JSONReadError { err } => {
                writeln!(f, "Error: Couldn't read json file: {}", err)
            }
            EchoErrors::JSONParseError { err } => {
                writeln!(f, "Error: Couldn't parse json file: {}", err)
            }
        }
    }
}

impl Error for EchoErrors {}

// Sys constraints applied
