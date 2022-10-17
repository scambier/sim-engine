use boa_engine::{
    property::Attribute, syntax::Parser, vm::CodeBlock, Context as BoaContext, JsResult, JsValue,
};
use boa_gc::Gc;

use winit::event::VirtualKeyCode;

use crate::{
    assets::Scripts, audio::play_audio, get_context, keyboard::u32_to_keycode, CANVAS, FONT,
    HEIGHT, PALETTE, WIDTH,
};

pub fn init_boa() -> BoaContext {
    let mut context = BoaContext::default();
    match context.parse(Scripts::load_game()) {
        Ok(_) => {}
        Err(e) => {
            log::error!("Error while parsing game script: {}", e);
        }
    }
    // Register globals
    setup_api(&mut context);

    // Feed the script definition to the context.
    match context.eval(Scripts::load_game()) {
        Ok(_) => {}
        Err(e) => {
            // log::error!("Error while evaluating script: {:?}", e)
        }
    };

    context
}

pub fn compile_update(mut context: &mut BoaContext) -> Gc<CodeBlock> {
    let src = "update()".as_bytes();
    let parsing_result = Parser::new(src.as_ref())
        .parse_all(&mut context)
        .map_err(|e| e.to_string());
    let statement_list = parsing_result.unwrap();
    let code_block = context.compile(&statement_list).unwrap();
    code_block
}

fn arg_i32(v: Option<&JsValue>) -> Option<i32> {
    match v {
        Some(v) => match v {
            // JsValue::Null => None,
            // JsValue::Undefined => None,
            // JsValue::Boolean(_) => None,
            // JsValue::String(_) => None,
            JsValue::Rational(v) => Some(*v as i32),
            JsValue::Integer(v) => Some(*v),
            JsValue::BigInt(v) => Some(v.to_f64() as i32),
            // JsValue::Object(_) => None,
            // JsValue::Symbol(_) => None,
            _ => None,
        },
        None => None,
    }
}

fn arg_str(v: Option<&JsValue>) -> String {
    match v.unwrap_or(&JsValue::Integer(0)) {
        JsValue::Null => "[null]".to_string(),
        JsValue::Undefined => "[undefined]".to_string(),
        JsValue::Boolean(v) => {
            if *v {
                "[true]".to_string()
            } else {
                "[false]".to_string()
            }
        }
        JsValue::Object(_) => "[object Object]".to_string(),
        JsValue::String(v) => v.to_string(),
        JsValue::Rational(v) => v.to_string(),
        JsValue::Integer(v) => v.to_string(),
        JsValue::BigInt(v) => v.to_string(),
        JsValue::Symbol(v) => v.to_string(),
    }
}

fn setup_api(boa: &mut BoaContext) {
    boa.register_global_property("WIDTH", WIDTH, Attribute::complement(Attribute::READONLY));
    boa.register_global_property("HEIGHT", HEIGHT, Attribute::complement(Attribute::READONLY));
    boa.register_global_builtin_function("getDelta", 0, api_get_delta);
    boa.register_global_builtin_function("trace", 0, api_trace);

    boa.register_global_builtin_function("clearScreen", 0, api_clear_screen);
    boa.register_global_builtin_function("drawRect", 0, api_draw_rect);
    boa.register_global_builtin_function("drawRectFill", 0, api_draw_rect_fill);
    boa.register_global_builtin_function("drawCirc", 0, api_draw_circ);
    boa.register_global_builtin_function("drawCircFill", 0, api_draw_circ_fill);
    boa.register_global_builtin_function("print", 0, api_print);

    boa.register_global_builtin_function("isKeyJustPressed", 0, api_is_key_just_pressed);
    boa.register_global_builtin_function("isKeyDown", 0, api_is_key_down);

    boa.register_global_builtin_function("playAudio", 0, api_play_audio);
}

#[inline(always)]
fn api_trace(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
    let str = arg_str(args.get(0));
    log::debug!("{}", str);
    return Ok(JsValue::null());
}

#[inline(always)]
fn api_print(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
    let text = arg_str(args.get(0));
    let x = arg_i32(args.get(1)).unwrap_or(0);
    let y = arg_i32(args.get(2)).unwrap_or(0);

    let color = match arg_i32(args.get(3)) {
        Some(color) => Some(PALETTE.color_idx(color as usize)),
        None => None,
    };

    let border = match arg_i32(args.get(4)) {
        Some(border) => Some(PALETTE.color_idx(border as usize)),
        None => None,
    };

    CANVAS.lock().unwrap().print(&text, x, y, &FONT, color, border);
    return Ok(JsValue::Null);
}

#[inline(always)]
fn api_get_delta(_this: &JsValue, _args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
    let context = get_context();
    return Ok(JsValue::Rational(context.time.delta_seconds_f64()));
}

#[inline(always)]
fn api_clear_screen(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
    let color = args
        .get(0)
        .unwrap_or(&JsValue::Integer(15))
        .as_number()
        .unwrap();
    let color = PALETTE.color_idx(color as usize);
    CANVAS.lock().unwrap().clear(color);
    return Ok(JsValue::null());
}

#[inline(always)]
fn api_draw_rect(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
    let x = arg_i32(args.get(0)).unwrap_or(0);
    let y = arg_i32(args.get(1)).unwrap_or(0);
    let width = arg_i32(args.get(2)).unwrap_or(0);
    let height = arg_i32(args.get(3)).unwrap_or(0);
    let color = arg_i32(args.get(4)).unwrap_or(0);

    let color = PALETTE.color_idx(color as usize);
    CANVAS.lock().unwrap().draw_rect(x as i32, y as i32, width as i32, height as i32, color);
    return Ok(JsValue::null());
}

#[inline(always)]
fn api_draw_rect_fill(
    _this: &JsValue,
    args: &[JsValue],
    _ctx: &mut BoaContext,
) -> JsResult<JsValue> {
    let x = arg_i32(args.get(0)).unwrap_or(0);
    let y = arg_i32(args.get(1)).unwrap_or(0);
    let width = arg_i32(args.get(2)).unwrap_or(0);
    let height = arg_i32(args.get(3)).unwrap_or(0);
    let color = arg_i32(args.get(4)).unwrap_or(0);

    let color = PALETTE.color_idx(color as usize);
    CANVAS.lock().unwrap().draw_rect_fill(x as i32, y as i32, width as i32, height as i32, color);
    return Ok(JsValue::null());
}

#[inline(always)]
fn api_draw_circ(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
    let x = arg_i32(args.get(0)).unwrap_or(0);
    let y = arg_i32(args.get(1)).unwrap_or(0);
    let r = arg_i32(args.get(2)).unwrap_or(0);
    let color = arg_i32(args.get(3)).unwrap_or(0);

    let color = PALETTE.color_idx(color as usize);
    CANVAS.lock().unwrap().draw_circ(x as i32, y as i32, r as i32, color);
    return Ok(JsValue::null());
}

#[inline(always)]
fn api_draw_circ_fill(
    _this: &JsValue,
    args: &[JsValue],
    _ctx: &mut BoaContext,
) -> JsResult<JsValue> {
    let x = arg_i32(args.get(0)).unwrap_or(0);
    let y = arg_i32(args.get(1)).unwrap_or(0);
    let r = arg_i32(args.get(2)).unwrap_or(0);
    let color = arg_i32(args.get(3)).unwrap_or(0);

    let color = PALETTE.color_idx(color as usize);
    CANVAS.lock().unwrap().draw_circ_fill(x as i32, y as i32, r as i32, color);
    return Ok(JsValue::null());
}

#[inline(always)]
fn api_is_key_just_pressed(
    _this: &JsValue,
    args: &[JsValue],
    _ctx: &mut BoaContext,
) -> JsResult<JsValue> {
    let context = get_context();
    match arg_i32(args.get(0)) {
        Some(key_nb) => {
            let key: Option<VirtualKeyCode> = u32_to_keycode(key_nb as u32);
            match key {
                Some(key) => {
                    return Ok(JsValue::Boolean(context.input.key_pressed(key)));
                }
                None => {
                    return Ok(JsValue::Boolean(false));
                }
            }
        }
        None => {
            return Ok(JsValue::Boolean(false));
        }
    }
}

#[inline(always)]
fn api_is_key_down(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
    let context = get_context();
    match arg_i32(args.get(0)) {
        Some(key_nb) => {
            let key: Option<VirtualKeyCode> = u32_to_keycode(key_nb as u32);
            match key {
                Some(key) => {
                    return Ok(JsValue::Boolean(context.input.key_held(key)));
                }
                None => {
                    return Ok(JsValue::Boolean(false));
                }
            }
        }
        None => {
            return Ok(JsValue::Boolean(false));
        }
    }
}

#[inline(always)]
fn api_play_audio(_this: &JsValue, args: &[JsValue], _ctx: &mut BoaContext) -> JsResult<JsValue> {
    let filename = arg_str(args.get(0));
    play_audio(&filename);
    return Ok(JsValue::null());
}
