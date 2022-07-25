use ahash::AHashMap;
use image::{DynamicImage, GenericImageView};

use crate::{BitmapFont, Color, Spritesheet};

/*
 * Several functions are adapted from https://github.com/egordorichev/pemsa and https://github.com/nesbox/TIC-80
 * The ellipse functions are a mix of both
 */

struct FillingLine {
    x1: i32,
    x2: i32,
    color: Color,
}

/// HashMap<y, {x1, x2}>
type Fillingbuffer = AHashMap<i32, FillingLine>;

pub struct Canvas {
    pub grid: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub fill_pattern: u16,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![0; width * height * 4],
            width,
            height,
            fill_pattern: u16::MAX,
        }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> Color {
        let w = self.width as u32;
        let i = (x * 4 + y * w * 4) as usize;
        let g = &self.grid;
        Color::from_arr(&[g[i], g[i + 1], g[i + 2], g[i + 3]])
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }
        let blend = if color.a > 0 && color.a < 255 {
            self.get_pixel(x as u32, y as u32).blend(color)
        } else {
            color
        };

        let x = x as u32;
        let y = y as u32;

        // Fill pattern
        if self.fill_pattern == 0 {
            return;
        } else if self.fill_pattern < u16::MAX {
            let px = x % 4;
            let py = y % 4;
            let idx = py * 4 + px;
            if self.fill_pattern & (1 << idx) == 0 {
                return;
            }
        }

        let i = (x * 4 + y * self.width as u32 * 4) as usize;
        self.grid[i..i + 4].copy_from_slice(&blend.as_arr());
    }

    pub fn sprite(
        &mut self,
        tx: u32,
        ty: u32,
        x: i32,
        y: i32,
        spritesheet: &Spritesheet,
        transparent: Option<Color>,
    ) {
        let idx = spritesheet.tile_idx(tx, ty);
        if let Some(img) = spritesheet.sprites.get(idx) {
            self.image(img, x, y, transparent)
        } else {
            println!("Cannot find sprite {},{}", tx, ty);
        }
    }

    pub fn image(
        &mut self,
        img: &DynamicImage,
        pos_x: i32,
        pos_y: i32,
        transparent: Option<Color>,
    ) {
        for (x, y, c) in img.pixels() {
            if let Some(transparent) = transparent {
                if c == image::Rgba(transparent.as_arr()) {
                    continue;
                }
            }
            self.set_pixel(pos_x + x as i32, pos_y + y as i32, Color::from_arr(&c.0));
        }
    }

    pub fn print_len(&self, text: &str, font: &BitmapFont) -> usize {
        text.chars()
            .map(|c| font.get_glyph(c.into(), false).1 as usize)
            .sum::<usize>()
    }

    pub fn print_center(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font: &BitmapFont,
        color: Option<Color>,
        border: Option<Color>,
    ) {
        let len = self.print_len(text, font);
        self.print(text, x - len as i32 / 2, y, font, color, border)
    }

    pub fn print(
        &mut self,
        text: &str,
        mut x: i32,
        y: i32,
        font: &BitmapFont,
        color: Option<Color>,
        border: Option<Color>,
    ) {
        if let Some(border) = border {
            for i in -1..=1 {
                for j in -1..=1 {
                    self.print(text, x + i, y + j, font, Some(border), None)
                }
            }
        }
        let color = color.unwrap_or(Color::from_rgb(255, 255, 255));

        text.chars().for_each(|c| {
            // println!("{} {}", c, c as u32);
            let glyph = font.get_glyph(c.into(), false);
            for (x_offset, y_offset, pixel) in glyph.0.pixels() {
                if pixel.0[3] == 0 {
                    continue;
                }
                self.set_pixel(x + x_offset as i32, y + y_offset as i32, color);
            }
            x += glyph.1 as i32;
        })
    }

    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: Color) {
        if y0 == y1 {
            return self.draw_horizontal_line(x0, y0, x1, color);
        }
        let mut steep = false;
        if (x1 - x0).abs() < (y1 - y0).abs() {
            (x0, y0) = (y0, x0);
            (x1, y1) = (y1, x1);
            steep = true;
        }

        if x0 > x1 {
            (x0, x1) = (x1, x0);
            (y0, y1) = (y1, y0);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let d_error = 2 * dy.abs();
        let mut err = 0;
        let mut y = y0;

        // int cx = drawStateModule->getCameraX();
        // int cy = drawStateModule->getCameraY();
        let cx = 0;
        let cy = 0;

        for x in x0..=x1 {
            if steep {
                self.set_pixel(y - cx, x - cy, color);
            } else {
                self.set_pixel(x - cx, y - cy, color);
            }

            err += d_error;

            if err > dx {
                y += if y1 > y0 { 1 } else { -1 };
                err -= dx * 2;
            }
        }
    }

    fn draw_horizontal_line(&mut self, mut x0: i32, y: i32, mut x1: i32, c: Color) {
        if x1 < x0 {
            (x0, x1) = (x0.min(x1), x0.max(x1));
        }
        for x in x0..=x1 {
            self.set_pixel(x, y, c);
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, mut w: i32, mut h: i32, color: Color) {
        w -= 1;
        h -= 1;
        self.draw_horizontal_line(x, y, x + w, color);
        self.draw_line(x + w, y, x + w, y + h, color);
        self.draw_horizontal_line(x + w, y + h, x, color);
        self.draw_line(x, y + h, x, y, color);
    }

    pub fn draw_rect_fill(&mut self, x: i32, y: i32, mut w: i32, mut h: i32, color: Color) {
        w -= 1;
        h -= 1;
        for i in y..=(y + h) {
            self.draw_horizontal_line(x, i, x + w, color)
        }
    }

    pub fn clear(&mut self, color: Color) {
        // let now = Instant::now();
        unsafe {
            let color = color.as_rgba_u32();
            self.grid.align_to_mut::<u32>().1.fill(u32::from_be(color));
        }
        // println!("{}", now.elapsed().as_nanos());
    }

    pub fn draw_ellipse(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, c: Color) {
        plot_ellipse(x0, y0, x1, y1, c, &mut |x, y, c| self.set_pixel(x, y, c));
    }

    pub fn draw_ellipse_fill(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, c: Color) {
        let mut fill_buffer = AHashMap::new();
        plot_ellipse(x0, y0, x1, y1, c, &mut |x, y, c| {
            fill_ellipse(x, y, c, &mut fill_buffer)
        });
        self.draw_fill_buffer(&mut fill_buffer);
    }

    fn draw_fill_buffer(&mut self, buffer: &mut Fillingbuffer) {
        for (y, value) in &*buffer {
            self.draw_horizontal_line(value.x1, *y, value.x2, value.color);
        }
        buffer.clear();
    }

    pub fn draw_circ(&mut self, ox: i32, oy: i32, r: i32, c: Color) {
        let mut x = r;
        let mut y = 0;
        let mut decision_over_2 = 1 - x;

        // ox -= drawStateModule->getCameraX();
        // oy -= drawStateModule->getCameraY();

        while y <= x {
            self.set_pixel(ox + x, oy + y, c);
            self.set_pixel(ox + y, oy + x, c);
            self.set_pixel(ox - x, oy + y, c);
            self.set_pixel(ox - y, oy + x, c);

            self.set_pixel(ox - x, oy - y, c);
            self.set_pixel(ox - y, oy - x, c);
            self.set_pixel(ox + x, oy - y, c);
            self.set_pixel(ox + y, oy - x, c);

            y += 1;

            if decision_over_2 < 0 {
                decision_over_2 += 2 * y + 1;
            } else {
                x -= 1;
                decision_over_2 += 2 * (y - x) + 1;
            }
        }
    }

    pub fn draw_circ_fill(&mut self, ox: i32, oy: i32, radius: i32, color: Color) {
        let mut x: i32 = radius;
        let mut y: i32 = 0;
        let mut error: i32 = 1 - radius;

        while y <= x {
            self.plot(ox, oy, x, y, color);

            if error < 0 {
                error += 2 * y + 3;
            } else {
                if x != y {
                    self.plot(ox, oy, y, x, color);
                }

                x -= 1;
                error += 2 * (y - x) + 3;
            }

            y += 1;
        }
    }

    fn plot(&mut self, cx: i32, cy: i32, x: i32, y: i32, c: Color) {
        self.draw_horizontal_line(cx - x, cy + y, cx + x, c);

        if y != 0 {
            self.draw_horizontal_line(cx - x, cy - y, cx + x, c);
        }
    }
}

fn plot_ellipse<F>(x0: i32, y0: i32, x1: i32, y1: i32, c: Color, mut draw: F)
where
    F: FnMut(i32, i32, Color),
{
    let width = ((x0 - x1) / 2).abs() as i64;
    let height = ((y0 - y1) / 2).abs() as i64;
    let ox = x0.min(x1) as i64 + width;
    let oy = y0.min(y1) as i64 + height;

    let mut x = 0;
    let mut y = height;
    let a2: i64 = width * width;
    let b2: i64 = height * height;
    let crit1: i64 = -(a2 / 4 + width % 2 + b2);
    let crit2: i64 = -(b2 / 4 + height % 2 + a2);
    let crit3: i64 = -(b2 / 4 + height % 2);
    let mut t: i64 = -a2 * y;
    let mut dxt: i64 = 2 * b2 * x;
    let mut dyt: i64 = -2 * a2 * y;
    let d2xt: i64 = 2 * b2;
    let d2yt: i64 = 2 * a2;

    while y >= 0 && x <= width {
        draw((ox + x) as i32, (oy + y) as i32, c);

        if x != 0 || y != 0 {
            draw((ox - x) as i32, (oy - y) as i32, c);

            if x != 0 && y != 0 {
                draw((ox + x) as i32, (oy - y) as i32, c);
                draw((ox - x) as i32, (oy + y) as i32, c);
            }
        }

        if t + b2 * x <= crit1 || t + a2 * y <= crit3 {
            x += 1;
            dxt += d2xt;
            t += dxt;
        } else if t - a2 * y > crit2 {
            y -= 1;
            dyt += d2yt;
            t += dyt;
        } else {
            x += 1;
            dxt += d2xt;
            t += dxt;
            y -= 1;
            dyt += d2yt;
            t += dyt;
        }
    }
}

fn fill_ellipse(x: i32, y: i32, c: Color, buffer: &mut Fillingbuffer) {
    // Don't insert lines outside of screen
    // TODO: also check max height
    // if y < 0 || y >= self.height as i32 {
    //     return;
    // }
    let hline = buffer.entry(y).or_insert(FillingLine {
        x1: i32::MAX,
        x2: i32::MIN,
        color: c,
    });
    if x < hline.x1 {
        hline.x1 = x;
    }
    if x > hline.x2 {
        hline.x2 = x;
    }
}
