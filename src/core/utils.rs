use crate::audio::player::SoundData;
use crate::core::errors::EchoErrors;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{
    error::Error,
    fs::{self, File},
    io,
    path::PathBuf,
};
use zip::read::ZipFile;
use zip::ZipArchive;

pub fn create_data_directory() -> Result<(), Box<dyn Error>> {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to get home directory");
    let data_dir: PathBuf = home_dir.join(".aaahhh");
    let _ = fs::create_dir(&data_dir)
        .map_err(|err: io::Error| EchoErrors::CouldNotCreateDataDirectory { err });

    let config_file: PathBuf = data_dir.join("config.yaml");
    File::create(config_file)
        .map_err(|err: io::Error| EchoErrors::CouldNotCreateDataDirectory { err })?;
    Ok(())
}
