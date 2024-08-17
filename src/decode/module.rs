use super::{Decode, DecodeResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Module {
  pub magic: String,
  pub version: u32,
}

impl Default for Module {
  fn default() -> Self {
    Self {
      magic: "\0asm".to_string(),
      version: 1,
    }
  }
}

impl Decode for Module {
  fn decode(input: &mut super::Input) -> DecodeResult<Self>
  where
    Self: Sized,
  {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use crate::decode::module::Module;
  use crate::decode::{Decode, Input};
  #[test]
  fn decode_module() -> anyhow::Result<()> {
    let wasm = wat::parse_str("(module)")?;
    let mut input = Input::new(wasm);
    let module = Module::decode(&mut input)?;
    assert_eq!(module, Module::default());
    Ok(())
  }
}
