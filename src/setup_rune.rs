use std::sync::Arc;

use rune::{
    termcolor::{ColorChoice, StandardStream},
    ContextError, Diagnostics, Module, Source, Sources, Vm,
};
use sim_core::{print, Color};

fn init_rune_module() -> Result<Module, ContextError> {
    let mut m = Module::new();

    m.function(["trace"], |a: &str| {
        println!("{}", a);
    })?;

    m.function(["cls"], |color: (u64, u64, u64)| {
        let color = (color.0 as u8, color.1 as u8, color.2 as u8);
        print("", 0, 0, Some(Color::from_rgb_tuple(color)), None);
    })?;

    m.function(
        // FIXME: "print" is a reserved keyword in Rune, so we use "printa" instead.
        ["printa"],
        |a: &str, x: i32, y: i32, color: (u64, u64, u64)| {
            let color = (color.0 as u8, color.1 as u8, color.2 as u8);
            print(a, x, y, Some(Color::from_rgb_tuple(color)), None);
        },
    )?;

    Ok(m)
}

pub fn get_vm() -> Vm {
    let mut context = rune_modules::default_context().unwrap();
    context.install(&init_rune_module().unwrap()).unwrap();
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
    Vm::new(runtime, Arc::new(unit))
}


