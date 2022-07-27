use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::io::Cursor;

use crate::{assets::Assets, get_context};

pub fn play_audio(filename: &str) {
    let manager = &mut get_context().audio_manager;
    if let Some(data) = Assets::load_audio(filename) {
        let sound_data =
            match StaticSoundData::from_cursor(Cursor::new(data), StaticSoundSettings::default()) {
                Ok(data) => data,
                Err(e) => {
                    log::error!("{}", e);
                    panic!();
                }
            };
        match manager.play(sound_data.clone()) {
            Ok(_) => {}
            Err(e) => log::error!("{}", e),
        };
    }
}
