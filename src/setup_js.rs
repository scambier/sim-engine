use boa_engine::{
    native_function::NativeFunctionPointer, Context, JsObject, JsValue, NativeFunction, Source,
};
use sim_core::{print, Color};

pub fn get_context() -> Context {
    let mut context = Context::default();
    let _ = context.register_global_callable(
        "print".into(),
        0,
        NativeFunction::from_copy_closure(move |this, args, ctx| {
            log::debug!("print: {:?}", args);
            let txt = args.get(0).unwrap().to_string(ctx).unwrap();
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
        }),
    );

    // let console = Console::init(&mut context);
    context
        .eval(Source::from_bytes(
            r"
                function update() {
                    print('Hello from Boa!', 10, 10);
                }
            ",
        ))
        .unwrap();
    context
}
