use std::fmt;

use bytes::{BytesMut, BufMut, Bytes};
use ethnum::U256;

const PAGE_SIZE: usize = 4 * 1024;

#[derive(Debug, Clone, Default)]
pub struct Memory(BytesMut);

impl Memory {
  pub fn new() -> Self {
    Self(BytesMut::new())
  }

  fn grow(&mut self) {
    self.0.put(&U256::from(0u32).to_be_bytes()[..]);
  }

  pub fn store(&mut self, offset: usize, value: U256, range: usize) {
    if (self.0.len() == 0) || ((range == 32) & (offset > 0)) {
      self.grow();
    }

    match range {
      1 => {
        println!("by {:?}", &value.to_be_bytes());
        let vb = &value.to_be_bytes()[31..32];
        println!("v {:?}", vb);
        println!("offset {:?}", offset);
        self.0[offset..1].copy_from_slice(vb);
      },
      32 => {
        self.0[offset..offset + range].copy_from_slice(&value.to_be_bytes()[..range]);
      },
      _ => panic!("Invalid range: {}", range)
    }

  }

  pub fn load(&mut self, offset: usize, length: usize) -> BytesMut {
    if offset + length > self.0.len() {
      self.grow();
    }

    self.0[offset..offset + length].try_into().unwrap()
  }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Memory: {:?}", self.0.to_vec())
    }
}