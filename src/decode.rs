use thiserror::Error;

pub mod module;

#[derive(Debug, Clone, Error)]
pub enum DecodeError {
  #[error("Unecpected value at {} line, {} char: {}", line, char, value)]
  UnexpectedValue { line: u64, char: u64, value: u8 },
  #[error("Unexpected End")]
  UnexpectedEndOfFile,
}

pub type DecodeResult<T> = std::result::Result<T, DecodeError>;

pub struct Input {
  pub line: u64,
  pub char: u64,
  pub pos: u64,
  len: u64,
  pub data: Vec<u8>,
}

impl Input {
  pub fn new(data: Vec<u8>) -> Self {
    let len = data.len() as u64;
    Self {
      line: 0,
      char: 0,
      pos: 0,
      len,
      data,
    }
  }
  pub fn next(&mut self) -> Option<u8> {
    let next_pos = self.pos + 1;
    if next_pos >= self.len {
      None
    } else {
      let t = self.data[next_pos as usize];
      self.pos += 1;
      if t == 0x0A {
        // LF
        self.char = 0;
        self.line += 1;
      } else {
        self.char += 1;
      }
      Some(t)
    }
  }
  pub fn is_end(&self) -> bool {
    self.pos + 1 >= self.len
  }
}

pub trait Decode {
  fn decode(input: &mut Input) -> DecodeResult<Self>
  where
    Self: Sized;
}
