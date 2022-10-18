use bytes::Bytes;

use crate::{stack::Stack, memory::Memory, opcodes::Opcode, instructions::{push1, push32, add, mul, stop, mstore8, return_data, push, mstore, jump, jumpi, dup, swap, sub}};

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

  pub fn read_code(&mut self, num_bytes: usize) -> u8 {
    if self.pc < self.code.len() {
        let start = self.pc;
        let end = self.pc + num_bytes;
        let value = &self.code[start..end];

        self.pc += num_bytes;
        value[0]
    } else {
      0u8
    }
  }

  pub fn read_push_code(&mut self, num_bytes: usize) -> Result<Vec<u8>, &str> {
    let mut value = Vec::new();
    
    let mut count: usize = 0;

    loop {
      count += 1;

      if self.pc == self.code.len() {
        break;
      }

      let byte = &self.code[self.pc..self.pc + 1];
      value.push(byte[0]);
      self.pc += 1;

      if count == num_bytes {
        break;
      }
    } 

    if value.is_empty() {
      value.push(0);
    }
    
    Ok(value)

  }
}

pub fn execute(context: &mut ExecutionContext) {

  while !context.stopped {
        let pc_before = context.pc;
        let byte = context.read_code(1);
        let opcode_str = hex::encode([byte]);

        let opcode = Opcode::new(&opcode_str);

        match opcode {
            Opcode::Push1 => {
              push1(context);
            },
            Opcode::Push2 => {
              push::<2>(context);
            },
            Opcode::Push3 => {
              push::<3>(context);
            },
            Opcode::Push4 => {
              push::<4>(context);
            },
            Opcode::Push5 => {
              push::<5>(context);
            },
            Opcode::Push6 => {
              push::<6>(context);
            },
            Opcode::Push7 => {
              push::<7>(context);
            },
            Opcode::Push8 => {
              push::<8>(context);
            },
            Opcode::Push9 => {
              push::<9>(context);
            },
            Opcode::Push10 => {
              push::<10>(context);
            },
            Opcode::Push11 => {
              push::<11>(context);
            },
            Opcode::Push12 => {
              push::<12>(context);
            },
            Opcode::Push13 => {
              push::<13>(context);
            },
            Opcode::Push14 => {
              push::<14>(context);
            },
            Opcode::Push15 => {
              push::<15>(context);
            },
            Opcode::Push16 => {
              push::<16>(context);
            },
            Opcode::Push17 => {
              push::<17>(context);
            },
            Opcode::Push18 => {
              push::<18>(context);
            },
            Opcode::Push19 => {
              push::<19>(context);
            },
            Opcode::Push20 => {
              push::<20>(context);
            },
            Opcode::Push21 => {
              push::<21>(context);
            },
            Opcode::Push22 => {
              push::<22>(context);
            },
            Opcode::Push23 => {
              push::<23>(context);
            },
            Opcode::Push24 => {
              push::<24>(context);
            },
            Opcode::Push25 => {
              push::<25>(context);
            }, 
            Opcode::Push26 => {
              push::<26>(context);
            },
            Opcode::Push27 => {
              push::<27>(context);
            },
            Opcode::Push28 => {
              push::<28>(context);
            },
            Opcode::Push29 => {
              push::<29>(context);
            },
            Opcode::Push30 => {
              push::<30>(context);
            },
            Opcode::Push31 => {
              push::<31>(context);
            },
            Opcode::Push32 => {
              push32(context);
            },
            Opcode::Dup1 => {
              dup::<1>(&mut context.stack);
            },
            Opcode::Dup2 => {
              dup::<2>(&mut context.stack);
            },
            Opcode::Dup3 => {
              dup::<3>(&mut context.stack)
            },
            Opcode::Swap1 => {
              swap::<1>(&mut context.stack);
            },
            Opcode::Add =>  {
              add(&mut context.stack);
            },
            Opcode::Mul =>  {
              mul(&mut context.stack);
            },
            Opcode::Sub => {
              sub(&mut context.stack)
            },
            Opcode::Stop => {
              stop(context);
            },
            Opcode::Mstore8 =>  {
              mstore8(context);
            },
            Opcode::Mstore =>  {
              mstore(context);
            },
            Opcode::Jump => {
              jump(context);
            },
            Opcode::Jumpi => {
              jumpi(context);
            },
            Opcode::Jumpdest =>  {
              continue;
            },
            Opcode::Return =>  {
              return_data(context);
            },
            _  => panic!("Invalid opcode: {:?}", opcode) 
        }

        println!("{:?} @ pc={}", opcode.op_string(), pc_before);
        println!("{}", context.stack);
        println!("{}", context.memory);
        println!();
  }

  let data = format!("{}{}", "0x", hex::encode(&context.return_data));
  println!("return data {:?} : {:?}", data, context.return_data.to_vec());

}