use sim_core::{image, BitmapFont};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;
impl Assets {
    pub fn load_font() -> BitmapFont {
        let img = Self::get("dinofive_font.png").expect("couldn't load image");
        BitmapFont::new(image::load_from_memory(&img.data).unwrap(), 5, 6, 2, 1)
    }

    pub fn load_audio(filename: &str) -> Option<Vec<u8>> {
        match Self::get(filename) {
            Some(file) => Some(file.data.to_vec()),
            None => {
                log::error!("couldn't load audio file: {}", filename);
                None
            }
        }
    }
}

#[derive(RustEmbed)]
#[folder = "game/"]
pub struct Scripts;

impl Scripts {
    pub fn load_game() -> String {
        let file = Self::get("build/game.js").expect("could not load game/game.js");
        String::from_utf8(file.data.into()).unwrap()
    }
}
