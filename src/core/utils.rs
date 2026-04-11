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

pub fn download_file(url: &str, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let response: ureq::Response = ureq::get(url).call()?;
    let total_size: u64 = response
        .header("content-length")
        .ok_or("Response doesn't include the content length")?
        .parse::<u64>()?;
    let mut file: File = File::create(path)?;
    let pb: ProgressBar = ProgressBar::new(total_size);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    let mut downloaded: u64 = 0;
    let mut buffer: [u8; 8192] = [0; 8192];

    let mut reader: Box<dyn Read + Send + Sync> = response.into_reader();

