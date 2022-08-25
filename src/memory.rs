use primitive_types::U256;

#[derive(Debug, Clone)]
pub struct Memory {
  memory: Vec<u8>
}

impl Memory {
  pub fn new() -> Self {
    Self {
      memory: Vec::new()
    }
  }

  pub fn store(&mut self, offset: U256, value: u8) -> Result<(), &str> {
    if offset.as_usize() >= self.memory.len() {

      self.memory.extend([0; 32].iter())
    }

    // self.memory[offset] = value;

    Ok(())
  }

  pub fn load(&mut self, offset: U256) -> Result<u8, &str> {
    if offset.as_usize() >= self.memory.len() {
      return Ok(0);
    }
    todo!()
    // self.memory[offset]
  }
}