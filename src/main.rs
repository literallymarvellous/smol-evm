use primitive_types::{U256, H256, U512};
use smol_evm::execution_context::ExecutionContext;
use smol_evm::instruction::register_instruction;
use smol_evm::run;
use smol_evm::{stack::Stack, instruction::Instruction, decode_opcode};
use std::ops::{Shl, Add, Div, Rem};
use std::process; 
use std::env;

fn u256_to_H256(x: U256) -> H256 {
    let mut numb = x;
    let vec_u256 = x.0.to_vec();

    let mut vec_h256 = vec![0u8; 32];
    // let mut vec_h256 = [0u8; 32];

    let mut index: usize = 32;

    for i in vec_u256.iter().rev() {
        if *i == 0u64 {
            vec_h256.splice(0..8, vec![0u8; 8]);
        }

        if i % 255u64 == 0 {
            let a = i / 255u64;
            vec_h256.append(&mut vec![255; a.try_into().unwrap()])
        }
    }

    todo!()
    // H256::from(vec_h256.as_slice())
}

fn stop(ctx: &mut ExecutionContext) {
    ctx.stop();
}
fn push1(ctx: &mut ExecutionContext) {
    let item = ctx.read_code(1).unwrap();
    let mut arr = [0u8; 32];
        if let Some(last) = arr.last_mut() {
        *last = item;
    };
    let item = H256::from(arr);
    // ctx.stack.push(item);
}
fn add(ctx: &mut ExecutionContext) {
    let a = ctx.stack.pop().unwrap().unwrap();
    let b = ctx.stack.pop().unwrap().unwrap();

    let a = a.as_bytes().last().unwrap();
    let b = b.as_bytes().last().unwrap();

    let c = U256::from(*a).overflowing_add(U256::from(*b));
    let c = c.0;
    // ctx.stack.push(c.0.0.);
}
fn mul(ctx: &mut ExecutionContext) {
    ctx.stop();
}

fn main() {
    // let mut instruction = Instruction::new();
    // register_instruction(0x00, "STOP", stop, &mut instruction);

    

    // let args: Vec<String> = env::args().collect();
    // let code = hex::decode(&args[1]).unwrap();
    // run(code, &instruction);
    
    let mut arr = [0u8; 32];
    if let Some(last) = arr.last_mut() {
        *last = 8;
    };

    let bytes = H256::from(arr);

    // println!("{:?}", U512::from(u8::MAX).rem(u8::MAX));

    let max_256 = hex::decode("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();

    println!("{:?}", max_256);

    let slic_256 = max_256.as_slice();

    let max_u256 = U256::from(slic_256);

    println!("{:?}", max_u256);

    // println!("{:?}", U512::from(slic).div_mod(U512::from(u8::MAX)));

    println!("{:?}", U256::from(2).overflowing_add(U256::from(1)).0.0);

    // println!("{:?}", 255u64.to_be_bytes());
    // println!("{:?}", U512::from(255).0.first().unwrap().to_be_bytes().last().unwrap());
    
}
