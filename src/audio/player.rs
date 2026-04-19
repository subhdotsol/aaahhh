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
    volume: f32,
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

        save_sound_buffers_to_json(&*sound_buffers.read().unwrap(), output_path_str, debug);
        println!("Map saved at: {:?}", output_path_str);
    }

    match config.key_define_type.as_str() {
        "multi" => {
            if debug {
                println!("Key define type: multi");
            }
            let pressed_keys = Arc::new(std::sync::Mutex::new(std::collections::HashSet::new()));
            listen(move |event: Event| match event.event_type {
                EventType::KeyPress(key) => {
                    let is_new_press = pressed_keys.lock().unwrap().insert(key);
                    if !is_new_press {
                        return;
                    }

                    let code: Option<&u64> = KEY_MAP.get(&key);
                    if let Some(code) = code {
                        if let Some(Defines::StringHashMap(map)) = &config.defines {
                            if let Some(file_name) = map.get(&code.to_string()) {
                                if let Some(sound_data) =
                                    sound_buffers.read().unwrap().get(file_name)
                                {
                                    let sound_source: SamplesBuffer<f32> = SamplesBuffer::new(
                                        sound_data.channels,
                                        sound_data.sample_rate,
                                        sound_data.samples.clone(),
                                    );
                                    if let Ok(sink) = rodio::Sink::try_new(&stream_handle) {
                                        sink.set_volume(volume);
                                        sink.append(sound_source.convert_samples::<f32>());
                                        sink.detach();
                                    } else if debug {
                                        eprintln!("Failed to create sink for file {}", file_name);
                                    }
                                } else if debug {
                                    eprintln!("Sound file {} not found in buffers", file_name);
                                }
                            } else if debug {
                                eprintln!("No file name mapped for key code: {}", code);
                            }
                        } else if debug {
                            eprintln!("Config defines is either None or not a StringHashMap!");
                        }
                    } else if debug {
                        eprintln!("No mapping found for key: {:?}", key);
                    }
                }
                EventType::KeyRelease(key) => {
                    pressed_keys.lock().unwrap().remove(&key);
                }
                _ => {}
            })
            .expect("Failed to start global key listener");
        }
        _ => {
            if debug {
                println!("Key define type: single");
            }

            let pressed_keys = Arc::new(std::sync::Mutex::new(std::collections::HashSet::new()));
            listen(move |event: Event| match event.event_type {
                EventType::KeyPress(key) => {
                    let is_new_press = pressed_keys.lock().unwrap().insert(key);
                    if !is_new_press {
                        return;
                    }

                    let code: Option<&u64> = KEY_MAP.get(&key);
                    if let Some(code) = code {
                        if let Some(Defines::U64HashMap(map)) = &config.defines {
                            if let Some(sound_segment) = map.get(&code.to_string()) {
                                if let Some(sound_data) =
                                    sound_buffers.read().unwrap().get(&config.sound)
                                {
                                    let start_ms: f32 = sound_segment[0] as f32 / 1000.0;
                                    let duration: Duration =
                                        Duration::from_millis(sound_segment[1] as u64);
                                        
                                    // Slice the audio vector natively to avoid `SeekError::NotSupported` panics
                                    let sample_rate = sound_data.sample_rate as f32;
                                    let channels = sound_data.channels as f32;
                                    let start_index = (start_ms * sample_rate * channels) as usize;
                                    let dur_samples = (duration.as_secs_f32() * sample_rate * channels) as usize;
                                    
                                    let end_index = (start_index + dur_samples).min(sound_data.samples.len());
                                    let slice = if start_index < sound_data.samples.len() {
                                        sound_data.samples[start_index..end_index].to_vec()
                                    } else {
                                        vec![]
                                    };

                                    let sound_source: SamplesBuffer<f32> = SamplesBuffer::new(
                                        sound_data.channels,
                                        sound_data.sample_rate,
                                        slice,
                                    );
                                    
                                    let stream_handle_clone: OutputStreamHandle =
                                        stream_handle.clone();
                                        
                                    thread::spawn(move || {
                                        let sink = Sink::try_new(&stream_handle_clone).unwrap();
                                        sink.set_volume(volume);
                                        sink.append(sound_source.convert_samples::<f32>());
                                        std::thread::sleep(duration);
                                        sink.stop();
                                    });
                                }
                            }
                        }
                    }
                }
                EventType::KeyRelease(key) => {
                    pressed_keys.lock().unwrap().remove(&key);
                }
                _ => {}
            })
            .expect("Failed to start global key listener");
        }
    }

    std::thread::park();
}

// Handle stream context wrapping

// Thread lock tracking
