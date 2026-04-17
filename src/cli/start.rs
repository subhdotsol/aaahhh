use crate::{
    audio::sounds::SoundFiles,
    core::constants::FILE_PATH,
    core::errors::EchoErrors,
    core::utils::{download_file, unzip_sounds},
};
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::PathBuf;
use std::{error::Error, fs, io, process::{Command, Stdio}};

fn spawn_daemon(index: usize, debug: bool, volume: f32) {
    let mut cmd = Command::new(std::env::current_exe().unwrap());
    cmd.arg("daemon").arg(index.to_string());
    if debug {
        cmd.arg("--debug");
    }
    cmd.arg("--volume").arg(volume.to_string());
    cmd.stdin(Stdio::null())
       .stdout(Stdio::null())
       .stderr(Stdio::null());
       
    match cmd.spawn() {
        Ok(child) => println!("Aaahhh is playing smoothly in the background! (PID: {}) \nRun `aaahhh stop` to terminate it.", child.id()),
        Err(e) => eprintln!("Failed to start background process: {}", e),
    }
}

pub fn start(debug: bool, volume: f32) -> Result<(), Box<dyn Error>> {
    let selection_array: Vec<String> = vec![
        SoundFiles::get_name(&SoundFiles::CherryMxRed),
        SoundFiles::get_name(&SoundFiles::GateronBlack),
        SoundFiles::get_name(&SoundFiles::HolyPanda),
    ];

    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a version")
        .default(0)
        .items(&selection_array)
        .interact()
        .unwrap();

    match &selection {
        // Cherry MX Red (formerly Apex Pro)
        0 => {
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::CherryMxRed));
            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::CherryMxRed));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKWDOt4d6pqQg5G0DIVMYr8sEyXJfZvWjaxmUz",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| EchoErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::CherryMxRed));
                unzip_sounds(&zip_path, &dir_path)
                    .map_err(|err: std::io::Error| EchoErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| EchoErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
                }
            }

            spawn_daemon(0, debug, volume);
        }

        // Gateron Black (formerly EG Orea)
        1 => {
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::GateronBlack));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::GateronBlack));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKyfs4UbnDO3HC1SAaFYcT5QKPzN4dxUG9bpEq",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| EchoErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::GateronBlack));
                let output_dir: PathBuf = PathBuf::from(FILE_PATH.to_str().unwrap());
                unzip_sounds(&zip_path, &output_dir)
                    .map_err(|err: std::io::Error| EchoErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| EchoErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
                }
            }

            spawn_daemon(1, debug, volume);
        }

        // Holy Panda (formerly Fall Out)
        2 => {
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::HolyPanda));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::HolyPanda));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKiaZz5PCUoHzBesmgDYlfWx4Fa5r37bXCZ6M2",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| EchoErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::HolyPanda));
                let output_dir: PathBuf = PathBuf::from(FILE_PATH.to_str().unwrap());
                unzip_sounds(&zip_path, &output_dir)
                    .map_err(|err: std::io::Error| EchoErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| EchoErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
                }
            }

            spawn_daemon(2, debug, volume);
        }

        a => Err(EchoErrors::UnwantedSelectionIndex { index: *a })?,
    }
    Ok(())
}

// Checked URLs for pandas

// Color palette mapping limits

// Loop patch
