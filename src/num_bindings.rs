use core::num;

use mountain_sakura::prelude::*;

pub fn num_bindings() -> Vec<MoSaBinding> {
    vec![
        MoSaBinding::new(
            "@std:num~>fromStr",
            |args| {
                let mut num_str = args[0].cast_string().unwrap().clone();
                if !num_str.ends_with(".0") {
                    num_str.push_str(".0");
                }
                match num_str.parse::<f64>() {
                    Ok(num) => RuntimeValue::Number(num),
                    Err(_) => RuntimeValue::Null,
                }
            }
        ),
        MoSaBinding::new(
            "@std:num~>toStr",
            |args| {
                let num = args[0].cast_number().unwrap().clone();

                let mut num_str = num.to_string();

                if num_str.ends_with(".0") {
                    num_str.pop();
                    num_str.pop();
                }

                RuntimeValue::String(num_str)
            }
        )
    ]
}