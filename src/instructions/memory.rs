use bytes::Bytes;
use crate::execution_context::ExecutionContext;

// memory operations

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