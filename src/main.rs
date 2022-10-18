use std::env;


use smol_evm::execution_context::{ExecutionContext, execute};
mod utils;
use utils::str_to_vec;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args_slice = args[0].as_str();

    let bytecode = str_to_vec(args_slice).unwrap();

    let mut execution_context = ExecutionContext::new(bytecode);
    execute(&mut execution_context);
}

