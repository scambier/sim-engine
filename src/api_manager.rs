use deno_core::op;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use winit::event::VirtualKeyCode;

use crate::{
    assets::Scripts, audio::play_audio, get_context, keyboard::u32_to_keycode, CANVAS, FONT,
    HEIGHT, PALETTE, WIDTH,
};

pub fn init_deno() -> JsRuntime {
    let ext = Extension::builder().ops(vec![api_trace::decl()]).build();

    // Initialize a runtime instance
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });
    runtime.execute_script("<usage>", r#"let a = 1;
    Deno.core.ops.api_trace("started");
    function update(){
        ++a;
        Deno.core.ops.api_trace("" + a);
    }"#).unwrap();
    runtime
}

#[op]
fn api_trace(str: String) -> Result<(), deno_core::error::AnyError> {
    log::debug!("{}", str);
    return Ok(());
}

// #[inline(always)]
// fn api_print(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
//     let text = arg_str(args.get(0));
//     let x = arg_i32(args.get(1)).unwrap_or(0);
//     let y = arg_i32(args.get(2)).unwrap_or(0);

//     let color = match arg_i32(args.get(3)) {
//         Some(color) => Some(PALETTE.color_idx(color as usize)),
//         None => None,
//     };

//     let border = match arg_i32(args.get(4)) {
//         Some(border) => Some(PALETTE.color_idx(border as usize)),
//         None => None,
//     };

//     CANVAS
//         .lock()
//         .unwrap()
//         .print(&text, x, y, &FONT, color, border);
//     return Ok(JsValue::Null);
// }

// #[inline(always)]
// fn api_get_delta(_this: &JsValue, _args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
//     let context = get_context();
//     return Ok(JsValue::Rational(context.time.delta_seconds_f64()));
// }

// #[inline(always)]
// fn api_clear_screen(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
//     let color = args
//         .get(0)
//         .unwrap_or(&JsValue::Integer(15))
//         .as_number()
//         .unwrap();
//     let color = PALETTE.color_idx(color as usize);
//     CANVAS.lock().unwrap().clear(color);
//     return Ok(JsValue::null());
// }

// #[inline(always)]
// fn api_draw_rect(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
//     let x = arg_i32(args.get(0)).unwrap_or(0);
//     let y = arg_i32(args.get(1)).unwrap_or(0);
//     let width = arg_i32(args.get(2)).unwrap_or(0);
//     let height = arg_i32(args.get(3)).unwrap_or(0);
//     let color = arg_i32(args.get(4)).unwrap_or(0);

//     let color = PALETTE.color_idx(color as usize);
//     CANVAS
//         .lock()
//         .unwrap()
//         .draw_rect(x as i32, y as i32, width as i32, height as i32, color);
//     return Ok(JsValue::null());
// }

// #[inline(always)]
// fn api_draw_rect_fill(
//     _this: &JsValue,
//     args: &[JsValue],
//     _ctx: &mut BoaContext,
// ) -> JsResult<JsValue> {
//     let x = arg_i32(args.get(0)).unwrap_or(0);
//     let y = arg_i32(args.get(1)).unwrap_or(0);
//     let width = arg_i32(args.get(2)).unwrap_or(0);
//     let height = arg_i32(args.get(3)).unwrap_or(0);
//     let color = arg_i32(args.get(4)).unwrap_or(0);

//     let color = PALETTE.color_idx(color as usize);
//     CANVAS
//         .lock()
//         .unwrap()
//         .draw_rect_fill(x as i32, y as i32, width as i32, height as i32, color);
//     return Ok(JsValue::null());
// }

// #[inline(always)]
// fn api_draw_circ(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
//     let x = arg_i32(args.get(0)).unwrap_or(0);
//     let y = arg_i32(args.get(1)).unwrap_or(0);
//     let r = arg_i32(args.get(2)).unwrap_or(0);
//     let color = arg_i32(args.get(3)).unwrap_or(0);

//     let color = PALETTE.color_idx(color as usize);
//     CANVAS
//         .lock()
//         .unwrap()
//         .draw_circ(x as i32, y as i32, r as i32, color);
//     return Ok(JsValue::null());
// }

// #[inline(always)]
// fn api_draw_circ_fill(
//     _this: &JsValue,
//     args: &[JsValue],
//     _ctx: &mut BoaContext,
// ) -> JsResult<JsValue> {
//     let x = arg_i32(args.get(0)).unwrap_or(0);
//     let y = arg_i32(args.get(1)).unwrap_or(0);
//     let r = arg_i32(args.get(2)).unwrap_or(0);
//     let color = arg_i32(args.get(3)).unwrap_or(0);

//     let color = PALETTE.color_idx(color as usize);
//     CANVAS
//         .lock()
//         .unwrap()
//         .draw_circ_fill(x as i32, y as i32, r as i32, color);
//     return Ok(JsValue::null());
// }

// #[inline(always)]
// fn api_is_key_just_pressed(
//     _this: &JsValue,
//     args: &[JsValue],
//     _ctx: &mut BoaContext,
// ) -> JsResult<JsValue> {
//     let context = get_context();
//     match arg_i32(args.get(0)) {
//         Some(key_nb) => {
//             let key: Option<VirtualKeyCode> = u32_to_keycode(key_nb as u32);
//             match key {
//                 Some(key) => {
//                     return Ok(JsValue::Boolean(context.input.key_pressed(key)));
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

// #[inline(always)]
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

// #[inline(always)]
// fn api_play_audio(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
//     let filename = arg_str(args.get(0));
//     play_audio(&filename);
//     return Ok(JsValue::null());
// }
