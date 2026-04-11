use crate::{
    core::config::SoundPack,
    core::constants::PID_FILE_PATH,
    core::errors::EchoErrors,
    audio::player::listen_and_play,
    audio::sounds::SoundFiles,
};
use rodio::OutputStream;
use std::{error::Error, fs};

pub fn daemon(index: usize, debug: bool) -> Result<(), Box<dyn Error>> {
    let pid = std::process::id();
    fs::write(&*PID_FILE_PATH, pid.to_string())?;

    if debug {
        println!("Background daemon running on PID {}", pid);
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

