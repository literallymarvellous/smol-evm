use crate::{stack::Stack, execution_context::ExecutionContext};
use bytes::Bytes;
use ethnum::U256;


pub fn push1(context: &mut ExecutionContext) {
    let v = context.read_code(1).unwrap();
    context.stack.push(U256::from(v));
}

pub fn add(stack: &mut Stack) {
    let a = stack.pop().unwrap(); 
    let b = stack.pop().unwrap();
    stack.push(a.overflowing_add(b).0);
}

pub fn mul(stack: &mut Stack) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(a.overflowing_mul(b).0);
}

pub fn stop(context: &mut ExecutionContext) {
    context.stop()
}

pub fn mstore8(context: &mut ExecutionContext) {
    let index = context.stack.pop().unwrap();
    let value = context.stack.pop().unwrap();

    context.memory.store(index.as_usize(), value, 1);
}

pub fn mstore(context: &mut ExecutionContext) {
    let index = context.stack.pop().unwrap();
    let value = context.stack.pop().unwrap();

    context.memory.store(index.as_usize(), value, 32);
}

pub fn return_data(context: &mut ExecutionContext) {
    let offset = context.stack.pop().unwrap();
    let length = context.stack.pop().unwrap();

    let data = context.memory.load(offset.as_usize(), length.as_usize());
    context.return_data = Bytes::from(data);
}