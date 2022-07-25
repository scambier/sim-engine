use std::num::ParseIntError;

use crate::Color;

pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new(palette: &str) -> Result<Self, ParseIntError> {
        let chars: Vec<char> = palette.chars().collect();
        let mut colors: Vec<Color> = vec![];
        for chunk in chars.chunks(6) {
            let str = chunk.iter().collect::<String>();
            let hex = u32::from_str_radix(&str, 16)?;
            colors.push(Color::from_u32(hex));
        }
        Ok(Self { colors })
    }

    pub fn get(&self, idx: usize) -> Color {
        *self.colors.get(idx).unwrap()
    }

    pub fn color_idx(&self, idx: usize) -> Color {
        self.get(idx % self.colors.len())
    }
}
