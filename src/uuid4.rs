use crate::{
  types::{Default, UuidV4Generator},
  utils::{add_prefix, random_16_bytes, uuid4},
};

impl Default for UuidV4Generator {
  fn default() -> Self {
    UuidV4Generator {
      prefix: "".to_string(),
      dashes: true,
      batch: 0,
    }
  }
}

impl UuidV4Generator  {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn gen(&self) -> String {
    let bytes: [u8; 16] = random_16_bytes();
    let uuid = uuid4(bytes);

    let mut api_key = match self.dashes {
      true => uuid,
      _ => uuid.replace("-", ""),
    };

    add_prefix(&mut api_key, &self.prefix)
  }
}

#[cfg(test)]
mod tests {
  use crate::types::{Default, UuidV4Generator};

  #[test]
  fn generate_key_with_prefix() {
    let options = UuidV4Generator {
      prefix: String::from("PREFIX-"),
      ..UuidV4Generator::default()
    };

    let result = UuidV4Generator::gen(&options);
    assert!(result.starts_with("PREFIX-"))
  }

  #[test]
  fn generate_key_with_dashes() {
    let options = UuidV4Generator {
      ..UuidV4Generator::default()
    };

    let result = UuidV4Generator::gen(&options);
    assert!(result.contains("-"))
  }

  #[test]
  fn generate_key_without_dashes() {
    let options = UuidV4Generator {
      dashes: false,
      ..UuidV4Generator::default()
    };

    let result = UuidV4Generator::gen(&options);
    assert!(!result.contains("-"))
  }
}
