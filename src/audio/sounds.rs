pub enum SoundFiles {
    CherryMxRed,
    GateronBlack,
    HolyPanda,
}

impl SoundFiles {
    pub fn get_name(sound: &SoundFiles) -> String {
        match &sound {
            SoundFiles::CherryMxRed => "Cherry MX Red".to_string(),
            SoundFiles::GateronBlack => "Gateron Black".to_string(),
            SoundFiles::HolyPanda => "Holy Panda".to_string(),
        }
    }

    pub fn get_zip_path(sound: &SoundFiles) -> String {
        match &sound {
            SoundFiles::CherryMxRed => "cherry_mx_red.zip".to_string(),
            SoundFiles::GateronBlack => "gateron_black.zip".to_string(),
            SoundFiles::HolyPanda => "holy_panda.zip".to_string(),
        }
    }

    pub fn get_extract_dir(sound: &SoundFiles) -> String {
        match &sound {
            SoundFiles::CherryMxRed => "CHERRYRED".to_string(),
            SoundFiles::GateronBlack => "EGOREA".to_string(),
            SoundFiles::HolyPanda => "FALLOUT".to_string(),
        }
    }
}

// Cherry Red definitions built

// Gat Black routing built
