#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub const fn from_u32(v: u32) -> Self {
        let r = ((v >> 16) & 0xFF) as u8;
        let g = ((v >> 8) & 0xFF) as u8;
        let b = (v & 0xFF) as u8;
        let a = 255;
        Self { r, g, b, a }
    }

    pub fn from_arr(arr: &[u8; 4]) -> Self {
        Self {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: arr[3],
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn as_arr(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn as_rgba_u32(&self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | (self.a as u32)
    }

    pub fn blend(&self, other: Color) -> Self {
        if other.a == 255 {
            return other;
        }
        if other.a == 0 {
            return *self;
        }
        let a = other.a as u32;
        Self::from_rgb(
            ((self.r as u32 * (255 - a) + other.r as u32 * a) / 255) as u8,
            ((self.g as u32 * (255 - a) + other.g as u32 * a) / 255) as u8,
            ((self.b as u32 * (255 - a) + other.b as u32 * a) / 255) as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u32() {
        assert_eq!(Color::from_u32(0xFFAA11), Color::from_rgb(255, 0xaa, 0x11));
    }

    #[test]
    fn test_as_rgba_u32() {
        assert_eq!(Color::from_rgb(255, 0xaa, 0x11).as_rgba_u32(), 0xFFAA11FF);
    }
}
