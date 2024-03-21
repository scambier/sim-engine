#![feature(lazy_cell)]
#![feature(let_chains)]

use assets::*;
use boa_engine::{Context as BoaContext, JsResult, Source as BoaSource};
use sim_core::{
    add_camera, context::Context, get_dt, load_font, print, App, Camera, Color, CoreGame, Runner,
};
use statics::*;

mod assets;
mod setup_js;
mod statics;

pub struct Game {
    javascript: BoaContext,
}

impl Game {
    fn update_framerate(&mut self) {
        unsafe {
            FRAMES += 1;
            if FRAMES % 30 == 0 {
                FRAMERATE = (1.0 / get_dt() as f64).round();
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            javascript: setup_js::get_context(),
        }
    }
}

impl CoreGame for Game {
    fn startup(&mut self) {}

    fn update(&mut self, ctx: &Context) {
        // Pre-update
        self.update_framerate();
        // self.rune.call(&["update"], ()).unwrap();
        self.javascript.eval(BoaSource::from_bytes("update()"));

        unsafe {
            print(
                format!("FPS: {}", FRAMERATE).as_str(),
                450,
                0,
                Some(Color::from_hex(0xffffff)),
                Some(Color::from_hex(0x000000)),
            );
        }
    }
}

fn main() {
    let mut app = App::new(WIDTH, HEIGHT, WINDOW_SCALE, "Sim Engine".to_string());

    load_font(0, Assets::load_font("dinofive_font.png", 5, 6, 1, 1));
    add_camera(0, Camera::new(WIDTH, HEIGHT));

    // app.set_scanlines(true);

    app.run(Game::default());
}
