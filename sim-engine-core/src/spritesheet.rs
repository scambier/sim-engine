use image::DynamicImage;

pub struct Spritesheet {
    pub image: DynamicImage,
    pub sprites: Vec<DynamicImage>,
    pub tile_width: u32,
    pub tile_height: u32,
    pub tiles_per_row: u32,
}

impl Spritesheet {
    pub fn new(image: DynamicImage, tile_width: u32, tile_height: u32) -> Self {
        let tiles_per_row = image.width() / tile_width;
        let mut sprites = vec![];

        let nb_sprites = image.width() / tile_width * image.height() / tile_height;
        for i in 0..nb_sprites {
            let (x, y, w, h) = Self::get_sprite_rect(i, &image);
            let sprite = image.crop_imm(x, y, w, h);
            sprites.push(sprite);
        }

        Self {
            image,
            sprites,
            tile_width,
            tile_height,
            tiles_per_row,
        }
    }

    pub fn get_sprite_rect(id: u32, spritesheet: &DynamicImage) -> (u32, u32, u32, u32) {
        let w = spritesheet.width() / 8;
        let (x, y) = (id % w, id / w);
        return (x * 8, y * 8, 8, 8);
    }

    pub fn tile_idx(&self, x:u32, y:u32) -> usize {
        (y * self.tiles_per_row + x) as usize
    }

}
