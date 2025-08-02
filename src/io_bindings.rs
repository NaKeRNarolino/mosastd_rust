use std::io::Read;
use mountain_sakura::prelude::{MoSaBinding, RuntimeValue};

pub fn io_bindings() -> Vec<MoSaBinding> {
    vec![
        MoSaBinding::new(
            "@std~>printLn",
            |args| {
                println!("{}", args[0]);

                RuntimeValue::Null
            }
        ),
        MoSaBinding::new(
            "@std~>print",
            |args| {
                print!("{}", args[0]);

                RuntimeValue::Null
            }
        ),
        MoSaBinding::new(
            "@std~>readLn",
            |_| {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();

                RuntimeValue::String(buf.trim().to_string())
            }
        ),
    ]
}