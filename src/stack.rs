use primitive_types::H256;

#[derive(Clone, Debug)]
pub struct Stack {
  stack: Vec<H256>,
  max_depth: usize,
}

impl Default for Stack {
  fn default() -> Self {
    Self {
      stack: Vec::new(),
      max_depth: 1024
    }
  }
}

impl Stack {
  pub fn new() -> Self {
    Self {
      stack: Vec::new(),
      max_depth: 1024
    }
  }

  pub fn push(&mut self, item: H256) -> Result<(), &str> {
    
    if self.stack.len() + 1 > self.max_depth {
      return Err("Stack Overflow")
    } 

    self.stack.push(item);

    Ok(())
  }

  pub fn pop(&mut self) -> Result<Option<H256>, &str> {
      if self.stack.len() == 0 {
        return Err("Stack undeflow")
      }

      Ok(self.stack.pop())
  }
}