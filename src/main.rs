#![feature(lazy_cell)]
#![feature(let_chains)]
#![feature(drain_filter)]

use std::sync::Arc;

use api_rune::*;
use assets::*;
use rune::termcolor::{ColorChoice, StandardStream};
use sim_core::clear_screen;
use statics::*;

use rune::{Diagnostics, Source, Sources, Vm};
use sim_core::runner::CoreGame;
use sim_core::{
    add_camera, get_dt, get_input, get_rng, load_font, load_spritesheet, print, runner::Runner,
    Camera, Color, VirtualKeyCode,
};

mod api_rune;
mod assets;
mod statics;

pub struct Game {
    vm: Vm,
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
        let mut context = rune_modules::default_context().unwrap();
        context
            .install(&api_rune::init_rune_module().unwrap())
            .unwrap();
        let runtime = Arc::new(context.runtime());

        let mut sources = Sources::new();
        sources.insert(Source::new(
            "script",
            r#"
        pub fn update() {
            printa("Hello from Rune!", 10, 10, (255,255,255));
        }
        "#,
        ));

        let mut diagnostics = Diagnostics::new();

        let result = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();

        if !diagnostics.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Always);
            diagnostics.emit(&mut writer, &sources).unwrap();
        }

        let unit = result.unwrap();
        let vm = Vm::new(runtime, Arc::new(unit));

        Self { vm }
    }
}

impl CoreGame for Game {
    fn update(&mut self) {
        // Pre-update
        self.update_framerate();
        self.vm.call(&["update"], ()).unwrap();

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
    let mut runner = Runner::new(WIDTH, HEIGHT, WINDOW_SCALE, "Porklike".to_string());
    // runner.set_scanlines(true);

    load_font(0, Assets::load_font("dinofive_font.png", 5, 6, 1, 1));
    add_camera(0, Camera::new(WIDTH, HEIGHT));

    let game = Game::default();
    runner.run(Box::new(game)).await;
}
