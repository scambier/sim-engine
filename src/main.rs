#![feature(lazy_cell)]
#![feature(let_chains)]

use assets::*;
use boa_engine::{Context as BoaContext, Source as BoaSource};
use sim_core::{
    add_camera, context::Context, get_dt, load_font, print, App, Camera, Color, CoreGame,
};
use statics::*;

mod assets;
mod api_manager_js;
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
            javascript: api_manager_js::get_context(),
        }
    }
}

impl CoreGame for Game {
    fn startup(&mut self) {}

    fn update(&mut self, _ctx: &Context) {
        // Pre-update
        self.update_framerate();
        self.javascript.eval(BoaSource::from_bytes("update()"));
    }
}

fn main() {
    let mut app = App::new(WIDTH, HEIGHT, WINDOW_SCALE, "Sim Engine".to_string());

    load_font(0, Assets::load_font("dinofive_font.png", 5, 6, 1, 1));
    add_camera(0, Camera::new(WIDTH, HEIGHT));

    // app.set_scanlines(true);

    app.run(Game::default());
}
