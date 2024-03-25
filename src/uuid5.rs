use crate::{
  types::{Default, UuidV5Generator},
  utils::{add_prefix, random_16_bytes, uuid5},
};

impl Default for UuidV5Generator {
  fn default() -> Self {
    UuidV5Generator {
      prefix: "".to_string(),
      namespace: None,
      dashes: true,
      batch: 0,
    }
  }
}

impl UuidV5Generator {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn gen(&self) -> String {
    if self.batch <= 0 {
      if !self.namespace.is_some() {
        panic!("The required 'namespace' option must be present");
      }
    }

    let bytes: [u8; 16] = match self.batch {
      0 => self.namespace.unwrap(),
      _ => random_16_bytes(),
    };

    let uuid = uuid5(bytes);

    let mut api_key = match self.dashes {
      true => uuid,
      _ => uuid.replace("-", ""),
    };

    add_prefix(&mut api_key, &self.prefix)
  }
}

#[cfg(test)]
mod tests {
  use uuid::Uuid;

  use crate::{
    types::{self, UuidV5Generator},
    utils::Utility,
  };

  #[test]
  fn generate_key_with_prefix() {
    let options = UuidV5Generator {
      prefix: String::from("PREFIX-"),
      namespace: Some(*Uuid::NAMESPACE_DNS.as_bytes()),
      ..types::Default::default()
    };

    let result = UuidV5Generator::gen(&options);
    assert!(result.starts_with("PREFIX-"))
  }

  #[test]
  fn generate_key_with_dashes() {
    let options = UuidV5Generator {
      namespace: Some(*Uuid::NAMESPACE_DNS.as_bytes()),
      ..types::Default::default()
    };

    let result = UuidV5Generator::gen(&options);
    assert!(result.contains("-"))
  }

  #[test]
  fn generate_key_without_dashes_and_with_namespace() {
    let options = UuidV5Generator {
      dashes: false,
      namespace: Some(*Uuid::NAMESPACE_DNS.as_bytes()),
      ..types::Default::default()
    };

    let result = UuidV5Generator::gen(&options);
    assert!(!result.contains("-"))
  }

  #[test]
  fn generate_key_with_dashes_and_with_namespace() {
    let options = UuidV5Generator {
      dashes: true,
      namespace: Some(*Uuid::NAMESPACE_URL.as_bytes()),
      ..types::Default::default()
    };

    let result = UuidV5Generator::gen(&options);
    assert!(match Uuid::parse_str(&result) {
      Ok(_) => true,
      Err(_) => false,
    });

    assert!(result.contains("-"));
  }

  #[test]
  fn generate_key_without_namespace() {
    let options = UuidV5Generator {
      batch: 2,
      namespace: None,
      ..types::Default::default()
    };

    let result = UuidV5Generator::gen(&options);
    assert!(match Uuid::parse_str(&result) {
      Ok(_) => true,
      Err(_) => false,
    })
  }

  #[test]
  fn generate_key_batch() {
    let options = UuidV5Generator {
      batch: 2,
      namespace: None,
      ..types::Default::default()
    };

    let result = Utility::batch(&options);
    assert!(result.len() == 2)
  }
}
