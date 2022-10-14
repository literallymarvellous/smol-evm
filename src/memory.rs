use bytes::BytesMut;

const PAGE_SIZE: usize = 4 * 1024;

#[derive(Debug, Clone, Default)]
pub struct Memory(BytesMut);

impl Memory {
  pub fn new() -> Self {
    Self(BytesMut::with_capacity(PAGE_SIZE))
  }

  pub fn grow(&mut self, size: usize) {
    let cap = self.0.capacity();
    if size > cap {
        let required_pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        self.0.reserve((PAGE_SIZE * required_pages) - self.0.len());
    }
    self.0.resize(size, 0);
  }
}