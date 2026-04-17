use crate::{
    core::config::SoundPack,
    core::constants::PID_FILE_PATH,
    core::errors::EchoErrors,
    audio::player::listen_and_play,
    audio::sounds::SoundFiles,
};
use rodio::OutputStream;
use std::{error::Error, fs};

pub fn daemon(index: usize, debug: bool, volume: f32) -> Result<(), Box<dyn Error>> {
    let pid = std::process::id();
    fs::write(&*PID_FILE_PATH, pid.to_string())?;

    if debug {
        println!("Background daemon running on PID {}", pid);
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    match index {
        0 => {
            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::CherryMxRed, debug)?;
            listen_and_play(debug, &SoundFiles::CherryMxRed, stream_handle, config, volume);
        }
        1 => {
            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::GateronBlack, debug)?;
            listen_and_play(debug, &SoundFiles::GateronBlack, stream_handle, config, volume);
        }
        2 => {
            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::HolyPanda, debug)?;
            listen_and_play(debug, &SoundFiles::HolyPanda, stream_handle, config, volume);
        }
        a => Err(EchoErrors::UnwantedSelectionIndex { index: a })?,
    }

    let _ = fs::remove_file(&*PID_FILE_PATH);
    Ok(())
}

// Thread safety comments
