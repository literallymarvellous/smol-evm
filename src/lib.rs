use execution_context::ExecutionContext;
use instruction::{Instruction, Opcode};

pub mod stack;
pub mod memory;
pub mod execution_context;
pub mod instruction;

pub fn decode_opcode<'a>(ctx: &'a mut ExecutionContext, instruction: &'a Instruction) -> Result<(&'a Opcode, &'a mut ExecutionContext), &'a str> {
    if ctx.pc >= ctx.code.len() {
      return Err("Invalid Code Offset");
    }
    let opcode = ctx.read_code(1).unwrap();
    match instruction.instructions_by_opcode.get(&opcode) {
      Some(op) => {
        return Ok((op, ctx));
      },
      None => {
        return Err("Unknwon opcode");
      }
    }
}

pub fn run(code: Vec<u8>, instruction: &Instruction) {
  let stack = stack::Stack::new();
  let memory = memory::Memory::new();
  let mut context = ExecutionContext::new(code, stack, memory);

  while !context.stopped {
    let pc_before = context.clone().pc;
    let (instruction, context) = decode_opcode(&mut context, instruction).unwrap();
    instruction.execute(context);

    println!("{:?}, @ pc={}", instruction.name, pc_before);
    println!("{:?}", context);
    println!();
  }
}