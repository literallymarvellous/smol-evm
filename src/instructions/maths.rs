use crate::stack::Stack;

// math operations
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