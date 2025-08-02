use mountain_sakura::prelude::MoSaBinding;

mod io_bindings;
mod num_bindings;

pub fn bindings() -> Vec<MoSaBinding> {
    let mut bindings = vec![];
    
    bindings.append(&mut io_bindings::io_bindings());
    bindings.append(&mut num_bindings::num_bindings());
    
    bindings
}

#[cfg(test)]
mod tests {
    use super::*;
    use mountain_sakura::prelude::*;

    #[test]
    fn test() {
        MoSaRunner::new("./src_mosa/main.mosa")
            .add_bindings(bindings())
            .add_lib("std", "./std")
            .run().unwrap();
    }
}
