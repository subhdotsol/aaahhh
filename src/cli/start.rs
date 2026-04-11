use crate::{
    audio::sounds::SoundFiles,
    core::constants::FILE_PATH,
    core::errors::EchoErrors,
    core::utils::{download_file, unzip_sounds},
};
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::PathBuf;
use std::{error::Error, fs, io, process::{Command, Stdio}};

fn spawn_daemon(index: usize, debug: bool) {
    let mut cmd = Command::new(std::env::current_exe().unwrap());
    cmd.arg("daemon").arg(index.to_string());
    if debug {
        cmd.arg("--debug");
    }
    cmd.stdin(Stdio::null())
       .stdout(Stdio::null())
       .stderr(Stdio::null());
       
    match cmd.spawn() {
        Ok(child) => println!("Aaahhh is playing smoothly in the background! (PID: {}) \nRun `aaahhh stop` to terminate it.", child.id()),
        Err(e) => eprintln!("Failed to start background process: {}", e),
    }
}

pub fn start(debug: bool) -> Result<(), Box<dyn Error>> {
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
