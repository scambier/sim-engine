use deno_core::op;
use deno_core::serde_json::Number;
use deno_core::serde_json::Value;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use winit::event::VirtualKeyCode;

use crate::{
    assets::Scripts, audio::play_audio, get_context, keyboard::u32_to_keycode, CANVAS, FONT,
    HEIGHT, PALETTE, WIDTH,
};

pub fn init_deno() -> JsRuntime {
    let ext = Extension::builder()
        .ops(vec![
            api_trace::decl(),
            api_clear_screen::decl(),
            api_print::decl(),
            api_get_delta::decl(),
            api_draw_rect::decl(),
            api_draw_rect_fill::decl(),
            api_draw_circ::decl(),
            api_draw_circ_fill::decl(),
        ])
        .build();

    // Initialize a runtime instance
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });
    runtime
        .execute_script(
            "<usage>",
            r#"
        const clearScreen = Deno.core.ops.api_clear_screen;
        const print = Deno.core.ops.api_print;
        const getDelta = Deno.core.ops.api_get_delta;
        const drawRect = Deno.core.ops.api_draw_rect;
        const drawRectFill = Deno.core.ops.api_draw_rect_fill;
        const drawCirc = Deno.core.ops.api_draw_circ;
        const drawCircFill = Deno.core.ops.api_draw_circ_fill;

        // function clearScreen(color) {
        //     Deno.core.ops.api_clear_screen(color);
        // }
    "#,
        )
        .unwrap();

    // Register constants
    runtime
        .execute_script(
            "<usage>",
            &format!("WIDTH = {}; HEIGHT= {}", WIDTH, HEIGHT).to_string(),
        )
        .unwrap();

    // Load game
    runtime
        .execute_script("<usage>", &Scripts::load_game())
        .unwrap();
}

#[op]
fn api_trace(str: String) -> Result<(), deno_core::error::AnyError> {
    log::debug!("{}", str);
    return Ok(());
}

#[op]
fn api_print(
    text: Value,
    x: Number,
    y: Number,
    color: Option<Number>,
    border: Option<Number>,
) -> Result<(), deno_core::error::AnyError> {
    let text = match text {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s,
        Value::Array(_) => "<array>".to_string(),
        Value::Object(_) => "<object>".to_string(),
    };
    let color = match color {
        Some(color) => Some(PALETTE.color_idx(color.as_u64().unwrap() as usize)),
        None => None,
    };

    let border = match border {
        Some(border) => Some(PALETTE.color_idx(border.as_u64().unwrap() as usize)),
        None => None,
    };

    CANVAS.lock().unwrap().print(
        &text,
        x.as_i64().unwrap() as i32,
        y.as_i64().unwrap() as i32,
        &FONT,
        color,
        border,
    );
    return Ok(());
}

#[op]
fn api_get_delta() -> Result<f64, deno_core::error::AnyError> {
    let context = get_context();
    return Ok(context.time.delta_seconds_f64());
}

#[op]
fn api_clear_screen(color: Option<Number>) -> Result<(), deno_core::error::AnyError> {
    let color = color.unwrap_or(15.into()).as_u64().unwrap();
    let color = PALETTE.color_idx(color as usize);
    CANVAS.lock().unwrap().clear(color);
    return Ok(());
}

#[op]
fn api_draw_rect(
    x: Number,
    y: Number,
    width: Number,
    height: Number,
    color: Number,
) -> Result<(), deno_core::error::AnyError> {
    let color = PALETTE.color_idx(color.as_u64().unwrap() as usize);
    CANVAS.lock().unwrap().draw_rect(
        x.as_i64().unwrap() as i32,
        y.as_i64().unwrap() as i32,
        width.as_i64().unwrap() as i32,
        height.as_i64().unwrap() as i32,
        color,
    );
    return Ok(());
}

#[op]
fn api_draw_rect_fill(
    x: Number,
    y: Number,
    width: Number,
    height: Number,
    color: Number,
) -> Result<(), deno_core::error::AnyError> {
    let color = PALETTE.color_idx(color.as_u64().unwrap() as usize);
    CANVAS.lock().unwrap().draw_rect_fill(
        x.as_f64().unwrap() as i32,
        y.as_f64().unwrap() as i32,
        width.as_f64().unwrap() as i32,
        height.as_f64().unwrap() as i32,
        color,
    );
    return Ok(());
}

#[op]
fn api_draw_circ(
    x: Number,
    y: Number,
    r: Number,
    color: Number,
) -> Result<(), deno_core::error::AnyError> {
    let color = PALETTE.color_idx(color.as_u64().unwrap() as usize);
    CANVAS.lock().unwrap().draw_circ(
        x.as_i64().unwrap() as i32,
        y.as_i64().unwrap() as i32,
        r.as_i64().unwrap() as i32,
        color,
    );
    return Ok(());
}

#[op]
fn api_draw_circ_fill(
    x: Number,
    y: Number,
    r: Number,
    color: Number,
) -> Result<(), deno_core::error::AnyError> {
    let color = PALETTE.color_idx(color.as_u64().unwrap() as usize);
    CANVAS.lock().unwrap().draw_circ_fill(
        x.as_i64().unwrap() as i32,
        y.as_i64().unwrap() as i32,
        r.as_i64().unwrap() as i32,
        color,
    );
    return Ok(());
}

#[op]
fn api_is_key_just_pressed(key_nb: Option<Number>) -> Result<bool, deno_core::error::AnyError> {
    let context = get_context();
    match key_nb {
        Some(key_nb) => {
            let key: Option<VirtualKeyCode> = u32_to_keycode(key_nb.as_u64().unwrap() as u32);
            match key {
                Some(key) => {
                    return Ok(context.input.key_pressed(key));
                }
                None => {
                    return Ok(false);
                }
            }
        }
        None => {
            return Ok(false);
        }
    }
}

// #[op]
// fn api_is_key_down(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
//     let context = get_context();
//     match arg_i32(args.get(0)) {
//         Some(key_nb) => {
//             let key: Option<VirtualKeyCode> = u32_to_keycode(key_nb as u32);
//             match key {
//                 Some(key) => {
//                     return Ok(JsValue::Boolean(context.input.key_held(key)));
//                 }
//                 None => {
//                     return Ok(JsValue::Boolean(false));
//                 }
//             }
//         }
//         None => {
//             return Ok(JsValue::Boolean(false));
//         }
//     }
// }

// #[op]
// fn api_play_audio(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
//     let filename = arg_str(args.get(0));
//     play_audio(&filename);
//     return Ok(());
// }
