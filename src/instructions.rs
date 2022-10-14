use crate::stack::Stack;
use ethnum::U256;

pub fn push<L: usize>(stack: &mut Stack, code: &[u8]) {
    stack.push(U256::from(&code[..L]));
} 

pub fn push1(stack: &mut Stack, code: &[u8]) {
    stack.push(U256::from(&code[..1]));
}

pub fn add(stack: &mut Stack) {
    let a = stack.pop(); 
    let b = stack.pop();
    stack.push(a.overflowing_add(b));
}

pub fn mul(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();
    stack.push(a.overflowing_mul(b).0);
}