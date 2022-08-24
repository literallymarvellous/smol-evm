use primitive_types::U256;

use crate::{stack::Stack, memory::Memory};

#[derive(Debug)]
pub struct ExecutionContext {
  pub code: Vec<u8>,
  pub stack: Stack,
  pub memory: Memory,
  pub pc: usize,
  pub stopped: bool
}

impl ExecutionContext {
  pub fn new(code: Vec<u8>, stack: Stack, memory: Memory) -> Self {
    Self {
      code,
      stack,
      memory,
      pc: 0,
      stopped: false
    }
  }

  pub fn stop(&mut self) {
    self.stopped = true;
  }

  pub fn read_code(&mut self, num_bytes: usize) -> Result<u8, &str> {
    let start = self.pc;
    let end = self.pc + num_bytes;
    let value = &self.code[start..end];

    self.pc += num_bytes;
    Ok(value[0])
  }
}