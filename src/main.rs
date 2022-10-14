use std::env;
use hex_literal::hex;
use utils::str_to_vec;
mod utils;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args_slice = args[0].as_str();

    let opcodes = str_to_vec(args_slice).unwrap();
    
    println!("{:?}", opcodes);  
}
