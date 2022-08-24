use std::collections::HashMap;

use crate::execution_context::ExecutionContext;

pub trait Execute {
    fn execute(&self, ctx: ExecutionContext);
}

pub struct Instruction {
  pub instructions: Vec<Box<dyn Execute>>,
  pub instructions_by_opcode: HashMap<u8, Box<dyn Execute>>
}

impl Instruction {
  pub fn new() -> Self {
    Self {
      instructions: Vec::new(),
      instructions_by_opcode: HashMap::new()
    }
  }

  pub fn update(&mut self, opcode: Opcode) {
    let op = opcode.clone();
    self.instructions.push(Box::new(opcode));
    if !self.instructions_by_opcode.contains_key(&op.opcode) {
      self.instructions_by_opcode.insert(op.opcode, Box::new(op));
    }
  }
}

#[derive(Debug, Clone)]
pub struct Opcode {
    name: String,
    opcode: u8,
    f: fn(ExecutionContext)
}

impl Opcode {
  pub fn new(opcode: u8, name: &str, f: fn(ExecutionContext)) -> Self {
    Self {
      opcode,
      name: name.to_owned(),
      f
    }
  }
}

impl Execute for Opcode {
  fn execute(&self, ctx: ExecutionContext) {
    (self.f)(ctx);
  }
}

pub fn register_instruction(opcode: u8, name: &str, f: fn(ExecutionContext), instruction: &mut Instruction) {
    let op = Opcode::new(opcode, name, f);
    instruction.update(op);
}