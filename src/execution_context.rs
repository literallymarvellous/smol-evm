use bytes::Bytes;

use crate::{stack::Stack, memory::Memory, opcodes::Opcode, instructions::{push1, add, mul, stop, mstore8, return_data}};

#[derive(Debug, Clone, Default)]
pub struct ExecutionContext {
  pub code: Vec<u8>,
  pub stack: Stack,
  pub memory: Memory,
  pub pc: usize,
  pub stopped: bool,
  pub return_data: Bytes
}

impl ExecutionContext {
  pub fn new(code: Vec<u8>) -> Self {
    Self {
      code,
      stack: Stack::new(),
      memory: Memory::new(),
      pc: 0,
      stopped: false,
      return_data: Bytes::new(),
    }
  }

  pub fn stop(&mut self) {
    self.stopped = true;
  }

  pub fn read_code(&mut self, num_bytes: usize) -> Result<u8, &str> {

    if self.pc < self.code.len() {
        let start = self.pc;
        let end = self.pc + num_bytes;
        let value = &self.code[start..end];

        self.pc += num_bytes;
        Ok(value[0])
    } else {
      Ok(0)
    }


  }
}

pub fn execute(context: &mut ExecutionContext) {

  while context.stopped == false {
        let pc_before = context.pc;
        let opcode_str = match context.read_code(1) {
          Ok(byte) => hex::encode([byte]),
          Err(e) => panic!("Error reading code: {:?}", e),
        };

        let opcode = Opcode::new(&opcode_str);

        match opcode {
            Opcode::Push1 => {
              push1(context);
            },
            Opcode::Add =>  {
              add(&mut context.stack);
            },
            Opcode::Mul =>  {
              mul(&mut context.stack);
            },
            Opcode::Stop => {
              stop(context);
            },
            Opcode::Mstore8 =>  {
              mstore8(context);
            },
            Opcode::Return =>  {
              return_data(context);
            },
            _  => panic!("Invalid opcode: {:?}", opcode) 
        }

        println!("{:?} @ pc={}", opcode.op_string(), pc_before);
        println!("{}", context.stack);
        println!("{}", context.memory);
        println!("");
  }

  let data = format!("{}{}", "0x", hex::encode(context.return_data.to_vec()));
  println!("return data {:?} : {:?}", data, context.return_data.to_vec());

}