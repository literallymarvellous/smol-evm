use execution_context::ExecutionContext;
use instruction::{Instruction, Opcode};

pub mod stack;
pub mod memory;
pub mod execution_context;
pub mod instruction;

pub fn decode_opcode(ctx: &ExecutionContext, instruction: &Instruction) -> Result<Opcode, &str> {
    if ctx.pc >= ctx.code.len() {
      return Err("Invalid Code Offset");
    }
    let opcode = ctx.read_code(1).unwrap();
    let opcode = match instruction.instructions_by_opcode.get(&opcode) {
      Some(op) => op,
      None => Err("Unknown Opcode")
    };

    opcode
}