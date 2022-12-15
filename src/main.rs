#![feature(error_reporter)]
// #![deny(clippy::all)]
// #![forbid(unsafe_code)]

mod assets;
mod audio;
mod boa_manager;
mod keyboard;

use std::sync::Mutex;

use lazy_static::lazy_static;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use sim_core::{
    get_context, get_context_mut, set_context, window, BitmapFont, Canvas, Context, Palette,
};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::assets::Assets;
use crate::boa_manager::{compile_update, init_boa};

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

lazy_static! {
    // pub static ref CONTEXT: Mutex<Context> = Mutex::new(Context::new());
    pub static ref FONT: BitmapFont = Assets::load_font();
    pub static ref CANVAS: Mutex<Canvas> = Mutex::new(Canvas::new(WIDTH, HEIGHT));
    pub static ref PALETTE: Palette = Palette::new("f9fff2f5d6786be0bfbab19796d45bf56c77e0864a59909e5794609e48916e6660505273ab3737693c363a363d202026").unwrap();
}

fn main() {
    set_context(WIDTH, HEIGHT);
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Trace).expect("error initializing logger");
        wasm_bindgen_futures::spawn_local(run());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run());
    }
}

async fn run() {
    log::info!("Starting engine");

    let mut boa_vm = init_boa();
    let mut code_block = compile_update(&mut boa_vm);
    let mut show_update_error = true;

    match boa_vm.eval("init()") {
        Ok(_) => {}
        Err(_e) => {
            // error!("Error while running init(): {:?}", e);
        }
    };

    let event_loop = EventLoop::new();

    let window = window::get_window(WIDTH, HEIGHT, 3, "JS Game Engine", &event_loop);

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(WIDTH, HEIGHT, surface_texture)
            .await
            .expect("Pixels error")
    };

    event_loop.run(move |event, _, control_flow| {
        let context = get_context_mut();
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // world.draw(pixels.get_frame());
            // rune_vm.call(&["draw"], (&rune_game,)).unwrap();
            context.time.update();
            unsafe {
                let (_, frame, _) = pixels.get_frame_mut().align_to_mut::<u32>();
                CANVAS.lock().unwrap().blit(frame);
            }
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        // Handle input events
        if context.input.update(&event) {
            if context.input.held_control() && context.input.key_pressed(VirtualKeyCode::R) {
                // Reload the VM
                boa_vm = init_boa();
                code_block = compile_update(&mut boa_vm);
                match boa_vm.eval("init()") {
                    Ok(_) => {}
                    Err(_e) => {
                        // log::error!("Error running init(): {:?}", e);
                    }
                }
                show_update_error = true;
            }

            // Close events
            if context.input.key_pressed(VirtualKeyCode::Escape) || context.input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = context.input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            match boa_vm.execute(code_block.clone()) {
                Ok(_) => {}
                Err(e) => {
                    if show_update_error {
                        log::error!("Error while executing {}", e.display());
                        show_update_error = false;
                    }
                }
            }

            window.request_redraw();
        }
    });
}
