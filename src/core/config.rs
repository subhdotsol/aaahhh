use crate::{audio::sounds::SoundFiles, core::constants::FILE_PATH, core::errors::EchoErrors};
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs, io, path::PathBuf};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Defines {
    U64HashMap(HashMap<String, Vec<u64>>),
    StringHashMap(HashMap<String, String>),
}

#[derive(Deserialize, Debug)]
pub struct SoundPack {
    pub key_define_type: String,
    pub sound: String,
    pub defines: Option<Defines>,
}

impl SoundPack {
    pub fn parse_config_file(sound_type: &SoundFiles, debug: bool) -> Result<Self, Box<dyn Error>> {
