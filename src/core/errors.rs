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
