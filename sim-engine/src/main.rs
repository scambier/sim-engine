#![feature(error_reporter)]
// #![deny(clippy::all)]
// #![forbid(unsafe_code)]

mod assets;
mod audio;
mod boa_manager;
mod keyboard;

use sim_core::{window, BitmapFont};
use lazy_static::lazy_static;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use sim_core::Context;

use crate::assets::Assets;
use crate::boa_manager::{init_boa, compile_update};

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

lazy_static! {
    // pub static ref CONTEXT: Mutex<Context> = Mutex::new(Context::new());
    pub static ref FONT: BitmapFont = Assets::load_font();
}

static mut CONTEXT: Option<Context> = None;

fn get_context() -> &'static mut Context {
    unsafe { CONTEXT.as_mut().unwrap_or_else(|| panic!()) }
}

fn main() {
    unsafe {
        CONTEXT = Some(Context::new(WIDTH as usize, HEIGHT as usize));
    }

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

    let context = get_context();
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

    let input = &mut get_context().input;
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(WIDTH, HEIGHT, surface_texture)
            .await
            .expect("Pixels error")
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // world.draw(pixels.get_frame());
            // rune_vm.call(&["draw"], (&rune_game,)).unwrap();
            context.time.update();
            unsafe {
                let (_, frame, _) = pixels.get_frame().align_to_mut::<u32>();
                let (_, grid, _) = context.canvas.grid.align_to::<u32>();
                grid.iter().zip(frame.iter_mut()).for_each(|(g, s)| *s = *g);
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
        if input.update(&event) {
            if input.held_control() && input.key_pressed(VirtualKeyCode::R) {

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
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
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
