use rune::{ContextError, Module};
use sim_core::{print, Color};

pub fn init_rune_module() -> Result<Module, ContextError> {
    let mut m = Module::new();

    m.function(["trace"], |a: &str| {
        println!("{}", a);
    })?;

    m.function(["cls"], |color: (u64, u64, u64)| {
        let color = (color.0 as u8, color.1 as u8, color.2 as u8);
        print("", 0, 0, Some(Color::from_rgb_tuple(color)), None);
    })?;

    m.function(["printa"], |a: &str, x: i32, y: i32, color: (u64, u64, u64)| {
        let color = (color.0 as u8, color.1 as u8, color.2 as u8);
        print(a, x, y, Some(Color::from_rgb_tuple(color)), None);
    })?;

    Ok(m)
}
