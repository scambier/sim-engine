mod bitmap_font;
mod canvas;
mod color;
pub mod fill_pattern;
pub mod palette;
mod spritesheet;
mod time;
pub mod window;

use std::io::Cursor;

pub use bitmap_font::*;
pub use canvas::*;
pub use color::*;
pub use image;
pub use kira;
pub use palette::*;
pub use spritesheet::*;

use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use winit_input_helper::WinitInputHelper;

use crate::time::Time;

pub struct Context {
    pub canvas: Canvas,
    pub palette: Palette,
    pub input: WinitInputHelper,
    pub time: Time,
    /// The audio manager. This plays sounds and manages resources.
    pub audio_manager: AudioManager,
}
impl Context {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            canvas: Canvas::new(width, height),
            // Default palette: https://lospec.com/palette-list/oxygen-16
            palette: Palette::new("f9fff2f5d6786be0bfbab19796d45bf56c77e0864a59909e5794609e48916e6660505273ab3737693c363a363d202026").unwrap(),
            input: WinitInputHelper::new(),
            time: Time::default(),
            audio_manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap(),
        }
    }
}

pub fn load_audio(data: Vec<u8>) -> StaticSoundData {
    let data = match StaticSoundData::from_cursor(Cursor::new(data), StaticSoundSettings::default())
    {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            panic!();
        }
    };
    data
}
