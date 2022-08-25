use std::collections::HashMap;

use crate::execution_context::ExecutionContext;

// pub trait Execute {
//     fn execute(&self, ctx: &mut ExecutionContext);
// }

pub struct Instruction {
  pub instructions: Vec<Opcode>,
  pub instructions_by_opcode: HashMap<u8, Opcode>
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
    self.instructions.push(opcode);
    if !self.instructions_by_opcode.contains_key(&op.opcode) {
      self.instructions_by_opcode.insert(op.opcode, op);
    }
  }
}

#[derive(Clone)]
pub struct Opcode {
    pub name: String,
    pub opcode: u8,
    pub f: fn(&mut ExecutionContext)
}

impl Opcode {
  pub fn new(opcode: u8, name: &str, f: fn(&mut ExecutionContext)) -> Self {
    Self {
      opcode,
      name: name.to_owned(),
      f
    }
  }

  pub fn execute(&self, ctx: &mut ExecutionContext) {
    (self.f)(ctx);
  }
}

// impl Execute for Opcode {
//     fn execute(&self, ctx: &mut ExecutionContext) {
//     (self.f)(ctx);
//   }
// }

pub fn register_instruction(opcode: u8, name: &str, f: fn(&mut ExecutionContext), instruction: &mut Instruction) {
    let op = Opcode::new(opcode, name, f);
    instruction.update(op);
}