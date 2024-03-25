use std::path::Path;

use boa_engine::{
    js_string, native_function::NativeFunctionPointer, Context, JsArgs, JsBigInt, JsResult,
    JsValue, NativeFunction, Source,
};
use sim_core::{clear_screen, draw_rect, draw_rect_fill, print, Color};

use crate::FRAMERATE;

// https://github.com/boa-dev/boa/blob/041a0304815eefed9f0fb23b493287fb9be3f615/examples/src/bin/synthetic.rs

fn api_cls(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let color = args.get_or_undefined(0).to_u32(ctx).unwrap_or(0);

    clear_screen(Color::from_hex(color));
    Ok(JsValue::undefined())
}

fn api_print(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let txt = args.get_or_undefined(0).to_string(ctx).unwrap();

    let x = args.get_or_undefined(1).to_i32(ctx).unwrap_or(0);
    let y = args.get_or_undefined(2).to_i32(ctx).unwrap_or(0);

    let color = args.get_or_undefined(3);
    let color = if color.is_undefined() {
        JsValue::Integer(0xffffff).to_u32(ctx).unwrap()
    } else {
        color.to_u32(ctx).unwrap()
    };

    print(
        &txt.to_std_string().unwrap(),
        x,
        y,
        Some(Color::from_hex(color)),
        None,
    );
    Ok(JsValue::undefined())
}

fn api_draw_rect(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let x = args.get_or_undefined(0).to_i32(ctx).unwrap_or(0);
    let y = args.get_or_undefined(1).to_i32(ctx).unwrap_or(0);
    let w = args.get_or_undefined(2).to_i32(ctx).unwrap_or(0);
    let h = args.get_or_undefined(3).to_i32(ctx).unwrap_or(0);

    let color = args.get_or_undefined(4);
    let color = if color.is_undefined() {
        JsValue::Integer(0xffffff).to_u32(ctx).unwrap()
    } else {
        color.to_u32(ctx).unwrap()
    };

    draw_rect(x, y, w, h, Color::from_hex(color));
    Ok(JsValue::undefined())
}

fn api_draw_rect_fill(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let x = args.get_or_undefined(0).to_i32(ctx).unwrap_or(0);
    let y = args.get_or_undefined(1).to_i32(ctx).unwrap_or(0);
    let w = args.get_or_undefined(2).to_i32(ctx).unwrap_or(0);
    let h = args.get_or_undefined(3).to_i32(ctx).unwrap_or(0);

    let color = args.get_or_undefined(4);
    let color = if color.is_undefined() {
        JsValue::Integer(0xffffff).to_u32(ctx).unwrap()
    } else {
        color.to_u32(ctx).unwrap()
    };

    draw_rect_fill(x, y, w, h, Color::from_hex(color));
    Ok(JsValue::undefined())
}

fn api_get_framerate(_this: &JsValue, _args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let framerate = unsafe { FRAMERATE };
    Ok(JsValue::Rational(framerate))
}

pub fn get_context() -> Context {
    let mut context = Context::default();

    register_function(&mut context, "cls", api_cls);
    register_function(&mut context, "print", api_print);
    register_function(&mut context, "drawRect", api_draw_rect);
    register_function(&mut context, "drawRectFill", api_draw_rect_fill);
    register_function(&mut context, "getFramerate", api_get_framerate);

    // let console = Console::init(&mut context);
    let path = Path::new("game/main.js");
    let src = Source::from_filepath(path);
    context.eval(src.unwrap()).unwrap();
    context
}

fn register_function(
    context: &mut Context,
    name: &str,
    func: NativeFunctionPointer,
)  {
    let _ = context.register_global_builtin_callable(name.into(), 0, NativeFunction::from_fn_ptr(func));
}
