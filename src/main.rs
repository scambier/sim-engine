#![feature(lazy_cell)]
#![feature(let_chains)]

use std::sync::Arc;

use assets::*;
use boa_engine::{Context, Source as BoaSource};
use rune::termcolor::{ColorChoice, StandardStream};
use sim_core::clear_screen;
use statics::*;

use rune::{Diagnostics, Source as RuneSource, Sources, Vm};
use sim_core::runner::CoreGame;
use sim_core::{
    add_camera, get_dt, get_input, get_rng, load_font, load_spritesheet, print, runner::Runner,
    Camera, Color, VirtualKeyCode,
};

mod assets;
mod setup_js;
mod setup_rune;
mod statics;

pub struct Game<'a> {
    rune: Vm,
    javascript: Context<'a>,
}

impl Game<'static> {
    fn update_framerate(&mut self) {
        unsafe {
            FRAMES += 1;
            if FRAMES % 30 == 0 {
                FRAMERATE = (1.0 / get_dt() as f64).round();
            }
        }
    }
}

impl Default for Game<'static> {
    fn default() -> Self {
        Self {
            rune: setup_rune::get_vm(),
            javascript: setup_js::get_context(),
        }
    }
}

impl CoreGame for Game<'static> {
    fn update(&mut self) {
        // Pre-update
        self.update_framerate();
        // self.rune.call(&["update"], ()).unwrap();
        self.javascript.eval(BoaSource::from_bytes("update()"));

        unsafe {
            print(
                format!("FPS: {}", FRAMERATE).as_str(),
                450,
                0,
                Some(Color::hex(0xffffff)),
                Some(Color::hex(0x000000)),
            );
        }
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Trace).expect("error initializing logger");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }

    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(start_main_loop());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(start_main_loop());
    }
}

async fn start_main_loop() {
    let mut runner = Runner::new(WIDTH, HEIGHT, WINDOW_SCALE, "Sim Engine".to_string());
    // runner.set_scanlines(true);

    load_font(0, Assets::load_font("dinofive_font.png", 5, 6, 1, 1));
    add_camera(0, Camera::new(WIDTH, HEIGHT));

    let game = Game::default();
    runner.run(Box::new(game)).await;
}
