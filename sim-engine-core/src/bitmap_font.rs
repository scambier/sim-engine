use image::{DynamicImage, GenericImageView};

pub struct BitmapFont {
    pub font: DynamicImage,
    pub glyph_width: u32,
    pub glyph_height: u32,
    pub space_width: u32,
    pub letter_spacing: u32,
    pub nb_glyphs: u32,
    /// (leftmost non-empty pixel, rightmost non-empty pixel)
    widths: Vec<(u32, u32)>,
}

impl BitmapFont {
    pub fn new(
        font: DynamicImage,
        glyph_width: u32,
        glyph_height: u32,
        space_width: u32,
        letter_spacing: u32,
    ) -> Self {
        let nb_glyphs = (font.width() / glyph_width) * (font.height() / glyph_height);

        // Get the real width of each glyph
        // by getting the leftmost and rightmost non-empty pixels
        let mut widths = vec![];
        for c in 0..nb_glyphs {
            let x = c % (font.width() / glyph_width);
            let y = c / (font.width() / glyph_width);
            let glyph = font.crop_imm(x * glyph_width, y * glyph_height, glyph_width, glyph_height);

            let mut first_pixel = glyph_width;
            let mut last_pixel = 0;
            for (x_offset, _y_offset, pixel) in glyph.pixels() {
                // // Non-transparent pixel
                if pixel.0[3] != 0 {
                    if x_offset < first_pixel {
                        first_pixel = x_offset;
                    }
                    if x_offset > last_pixel {
                        last_pixel = x_offset;
                    }
                }
            }

            // Handle empty glyphs
            if last_pixel < first_pixel {
                first_pixel = 0;
                last_pixel = space_width;
            }

            widths.push((first_pixel, last_pixel));
        }

        // for (k, v) in widths.iter().enumerate() {
        //     println!("{} : {:?}", k as char, v);
        // }

        BitmapFont {
            font,
            glyph_width,
            glyph_height,
            widths,
            space_width,
            letter_spacing,
            nb_glyphs,
        }
    }

    /// Returns the corresponding DynamicImage, and its width on screen;
    /// The width INCLUDES the letter spacing
    pub fn get_glyph(&self, mut c: u32, monospace: bool) -> (DynamicImage, u32) {
        if c > self.nb_glyphs {
            c = '?' as u32;
        }
        let gw = self.glyph_width;
        let gh = self.glyph_height;
        let x = c % (self.font.width() / gw);
        let y = c / (self.font.width() / gw);

        let mut first_pixel = x * gw;
        let mut width = self.glyph_width;
        if !monospace {
            let real_size = self.widths.get(c as usize).unwrap();
            first_pixel += real_size.0;
            width = real_size.1 - real_size.0 + 1;
        }

        let glyph = self.font.crop_imm(first_pixel, y * gh, width, gh);

        (glyph, width + self.letter_spacing)
    }
}
