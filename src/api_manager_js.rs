use std::path::Path;

use boa_engine::{object::FunctionObjectBuilder, Context, JsResult, JsValue, NativeFunction, Source};
use sim_core::{print, Color};

// https://github.com/boa-dev/boa/blob/041a0304815eefed9f0fb23b493287fb9be3f615/examples/src/bin/synthetic.rs

fn api_print(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let txt = args.first().unwrap().to_string(ctx).unwrap();
    let x = args
        .get(1)
        .unwrap_or(&JsValue::Integer(0))
        .to_i32(ctx)
        .unwrap();
    let y = args
        .get(2)
        .unwrap_or(&JsValue::Integer(0))
        .to_i32(ctx)
        .unwrap();

    // let default_color = JsObject::with_object_proto(context.intrinsics());
    // default_color.set(0, 0, false, &mut context);
    // default_color.set(1, 0, false, &mut context);
    // default_color.set(2, 0, false, &mut context);

    // let color = args
    //     .get(3)
    //     .unwrap_or(&JsValue::from(default_color))
    //     .to_object(ctx)
    //     .unwrap();
    let color = (255, 255, 255);
    print(
        &txt.to_std_string().unwrap(),
        x,
        y,
        Some(Color::from_rgb_tuple(color)),
        None,
    );
    Ok(JsValue::undefined())
}

pub fn get_context() -> Context {
    let mut context = Context::default();
    let _ =
        context.register_global_callable("print".into(), 0, NativeFunction::from_fn_ptr(api_print));

    // let console = Console::init(&mut context);
    let path = Path::new("game/main.js");
    let src = Source::from_filepath(path);
    context
        .eval(src.unwrap())
        .unwrap();
    context
}
