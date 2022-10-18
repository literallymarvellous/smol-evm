use std::collections::HashSet;
use ethnum::U256;

use crate::{execution_context::ExecutionContext, opcodes::Opcode};

// intructions that manipulate the program counter
pub fn jump(context: &mut ExecutionContext) {
  let new_pc = context.stack.pop().unwrap();
  set_program_counter(context, new_pc);
}

pub fn jumpi(context: &mut ExecutionContext) {
  let new_pc = context.stack.pop().unwrap();
  let cond = context.stack.pop().unwrap();
  if cond != 0 {
    set_program_counter(context, new_pc)
  }
}

fn set_program_counter(context: &mut ExecutionContext, byte: U256) {
    let jumpdests = valid_jump_destinations(&context.code);

    if jumpdests.contains(&byte.as_usize()) {
      context.pc = byte.as_usize();
    }
}

fn valid_jump_destinations(bytecode: &[u8]) -> HashSet<usize> {
  let mut jumpdests = HashSet::new();
  let mut i = 0;

  while i < bytecode.len() {
      let opcode = bytecode[i];

      if opcode == Opcode::Jumpdest.to_u8() {
        jumpdests.insert(i);
      } else if Opcode::Push1.to_u8() <= opcode && opcode <= Opcode::Push32.to_u8() {
        i += (opcode - Opcode::Push1.to_u8() + 1) as usize;
      }

      i += 1
  }
  jumpdests
}