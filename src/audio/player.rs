use crate::{
    audio::sounds::SoundFiles,
    core::config::{Defines, SoundPack},
    core::constants::{FILE_PATH, KEY_MAP},
    core::errors::EchoErrors,
    core::utils::{is_audio_file, save_sound_buffers_to_json},
};
use rdev::{listen, Event, EventType};
use rodio::{buffer::SamplesBuffer, Decoder, OutputStreamHandle, Sink, Source};
use serde::Serialize;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{read_dir, DirEntry, File},
    io::BufReader,
    path::PathBuf,
    sync::{Arc, RwLock, RwLockWriteGuard},
    thread,
    time::Duration,
};

#[derive(Clone, Debug, Serialize)]
pub struct SoundData {
    samples: Vec<f32>,
    channels: u16,
    sample_rate: u32,
}

pub fn listen_and_play(
    debug: bool,
    sound: &SoundFiles,
    stream_handle: OutputStreamHandle,
    config: SoundPack,
) {
    let sound_buffers: Arc<RwLock<HashMap<String, SoundData>>> =
        Arc::new(RwLock::new(HashMap::new()));
    let mut handles: Vec<_> = vec![];

    let dir_path: PathBuf = FILE_PATH.join(SoundFiles::get_extract_dir(sound));
    for entry in read_dir(dir_path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();
        if path.is_file() && is_audio_file(&path) {
            let sound_buffers: Arc<RwLock<HashMap<String, SoundData>>> = Arc::clone(&sound_buffers);
            let path: PathBuf = path.to_path_buf();
            let handle: thread::JoinHandle<()> = thread::spawn(move || {
                let file: BufReader<File> = BufReader::new(File::open(&path).unwrap());
                let decoder: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
                let channels: u16 = decoder.channels();
                let sample_rate: u32 = decoder.sample_rate();
                let samples: Vec<f32> = decoder.convert_samples().collect();

                let file_name: String = path
                    .file_name()
                    .and_then(|os_str: &OsStr| os_str.to_str())
                    .map(|s: &str| s.to_string())
                    .unwrap_or_else(|| path.to_string_lossy().to_string());

                let mut sound_buffers: RwLockWriteGuard<'_, HashMap<String, SoundData>> =
                    sound_buffers.write().unwrap();
                sound_buffers.insert(
                    file_name,
                    SoundData {
                        samples,
                        channels,
                        sample_rate,
                    },
                );
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
    if debug {
        let output_path: PathBuf = FILE_PATH.join("sound_buffers.json");
        let output_path_str: &str = output_path.to_str().expect("Invalid UTF-8 in output path");
