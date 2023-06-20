use rune::{ContextError, Module};

pub fn init_rune_module() -> Result<Module, ContextError> {
    let mut m = Module::new();
    m.function(["trace"], |a: &str| {
        println!("{}", a);
    })?;

    Ok(m)
}
