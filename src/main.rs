use std::env;
use bytes::{BytesMut, BufMut, Bytes};
use ethnum::U256;
use hex_literal::hex;
use smol_evm::execution_context::{ExecutionContext, execute};
use utils::str_to_vec;
mod utils;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args_slice = args[0].as_str();

    let bytecode = str_to_vec(args_slice).unwrap();

    let mut execution_context = ExecutionContext::new(bytecode);
    execute(&mut execution_context);

    // let mut buf = vec![255u8; 32];

    // let mut bytes = BytesMut::new();
    

    // let bb =  U256::from(42u32).to_be_bytes();
    // println!("bb {:?}", bb);

    // bytes.put(&bb[..]);
    // bytes[0..0+1].copy_from_slice(&bb[31..32]);
    // println!("word bytes: {:?}", bytes.to_vec());
    // // let byte8 = bytes[0..32].try_into().unwrap();

    // // println!("u8 get {:?}", U256::from_be_bytes(byte8));
    // // assert!(U256::MAX == max)
}
