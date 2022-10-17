use std::fmt;

use arrayvec::ArrayVec;
use ethnum::U256;

#[derive(Clone, Default, Debug)]
pub struct Stack {
  stack: ArrayVec<U256, 1024>,
}

impl Stack {
  pub fn new() -> Self {
    Self {
      stack: ArrayVec::new(),
    }
  }

  pub fn push(&mut self, item: U256) {
    self.stack.push(item);
  }

  pub fn pop(&mut self) -> Option<U256> {
      self.stack.pop()
  }

  pub fn len(&self) -> usize {
      self.stack.len()
  }

  pub fn is_empty(&self) -> bool {
    self.stack.len() == 0
  }

  pub fn peek(&self) -> Option<&U256> {
    self.stack.get(self.len() - 1)
  }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack: {:?}", self.stack)
    }
}