use rust_embed::RustEmbed;
use sim_core::{image, BitmapFont, Spritesheet};

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;
impl Assets {
    pub fn load_binary(filename: &str) -> Vec<u8> {
        let file = Self::get(filename).unwrap_or_else(|| panic!("Couldn't load {}", filename));
        file.data.into()
    }

    pub fn load_font(
        filename: &str,
        glyph_width: u32,
        glyph_height: u32,
        space_width: u32,
        letter_spacing: u32,
    ) -> BitmapFont {
        let img = Self::get(filename).expect("Couldn't load font");
        BitmapFont::new(
            image::load_from_memory(&img.data).unwrap(),
            glyph_width,
            glyph_height,
            space_width,
            letter_spacing,
        )
    }

    pub fn load_spritesheet(filename: &str, tile_size: u32) -> Spritesheet {
        let img = Self::get(filename).unwrap_or_else(|| panic!("Couldn't load {}", filename));
        Spritesheet::new(image::load_from_memory(&img.data).unwrap(), tile_size, tile_size)
    }

    // pub fn load_ron<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    //     let data = Self::load_binary(filename);
    //     ron::from_str(&String::from_utf8(data).unwrap()).unwrap()
    // }
}
