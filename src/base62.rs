extern crate base_x;

use crate::{
  constants::get_base62_char_pool,
  types::Base62Generator,
  utils::{random_16_bytes, uuid4},
};

impl Default for Base62Generator {
  fn default() -> Self {
    Base62Generator { batch: 0 }
  }
}

impl Base62Generator {
  pub fn gen() -> String {
    Self::gen_key(None)
  }

  pub fn mock_gen(input: String) -> String {
    Self::gen_key(Some(input))
  }

  fn gen_key(input: Option<String>) -> String {
    let bytes: [u8; 16] = random_16_bytes();

    let gen_input = match input {
      Some(user_input) => user_input,
      None => {
        let uuid = uuid4(bytes);

        uuid.replace("-", "")
      }
    };

    let dictionary = &get_base62_char_pool();
    let alphabet = gen_input.as_bytes();

    base_x::encode(&dictionary[..], alphabet)
  }
}

#[cfg(test)]
mod tests {
  use std::str::from_utf8;

  use crate::constants::get_base62_char_pool;

  #[test]
  fn generate_base62_string() {
    let dictionary = &get_base62_char_pool();
    let mock_input = "a70392c855ee4df5aa2eb2ea47c57af4";
    let encoded = super::mock_gen(mock_input.to_owned());

    let binding = base_x::decode(&dictionary[..], &encoded).unwrap();
    let decoded = binding.as_slice();

    assert_eq!(mock_input, from_utf8(decoded).unwrap());
  }
}
