use ethnum::U256;
use crate::{execution_context::ExecutionContext, stack::Stack, utils::vec_u8_to_u256};
use super::stop;

// stack operations

pub fn push1(context: &mut ExecutionContext) {
    match context.read_push_code(1) {
      Ok(v) => {
        let value = U256::from(v[0]);
        context.stack.push(value);
      }
      Err(_) => stop(context)
    }
}

pub fn push32(context: &mut ExecutionContext) {
    match context.read_push_code(32) {
      Ok(v) => {
        let value = U256::from_be_bytes(v.try_into().unwrap());
        context.stack.push(value);
      }
      Err(_) => stop(context)
    }
}

// source: https://github.com/akula-bft/akula/blob/master/src/execution/evm/instructions/stack_manip.rs
pub fn push<const LEN: usize>(context: &mut ExecutionContext) {
    match context.read_push_code(LEN) {
      Ok(v) => {
        let v_u256 = vec_u8_to_u256(&v);
        context.stack.push(v_u256);
      }
      Err(_) => stop(context)
    }
}

pub fn pop(stack: &mut Stack) {
    stack.pop();
}

//source: https://github.com/akula-bft/akula/blob/master/src/execution/evm/instructions/stack_manip.rs
pub fn dup<const HEIGHT: usize>(stack: &mut Stack) {
  let v = stack.get(HEIGHT - 1).unwrap();
  stack.push(*v);
}

// source: //source: https://github.com/akula-bft/akula/blob/master/src/execution/evm/instructions/stack_manip.rs
pub(crate) fn swap<const HEIGHT: usize>(stack: &mut Stack) {
    stack.swap_top(HEIGHT);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    pub fn build_context(str: &str) -> (ExecutionContext, Vec<u8>) {
        let bytecode = hex::decode(str).unwrap();
        let bytes = bytecode.clone();

        (ExecutionContext::new(bytecode), bytes)
    }

    #[test]
    fn test_push1() {
        let (mut ctx, bytecode) = build_context("6080");
        push1(&mut ctx);

        let i = U256::from(bytecode[0]);
        assert_eq!(&i, ctx.stack.get(0).unwrap());
    }

    #[test]
    fn test_pop() {
        let (mut ctx, _bytecode) = build_context("6080");
        push1(&mut ctx);
        pop(&mut ctx.stack);
        assert_eq!(ctx.stack.len(), 0)
    }
}