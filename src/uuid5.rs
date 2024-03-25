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

fn times2() -> String {
  String::from("he")
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
    utils::{uuid4, Utility},
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
  fn generate_key_without_dashes() {
    let options = UuidV5Generator {
      dashes: false,
      namespace: Some(*Uuid::NAMESPACE_DNS.as_bytes()),
      ..types::Default::default()
    };

    let result = UuidV5Generator::gen(&options);
    assert!(!result.contains("-"))
  }

  #[test]
  fn generate_key_with_namespace() {
    let id = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8";

    let options = UuidV5Generator {
      batch: 0,
      namespace: Some(*Uuid([id.as_bytes()])),
      ..types::Default::default()
    };

    let result = UuidV5Generator::gen(&options);
    println!("{:?}", result);
    assert!(result.contains("-"))
  }

  #[test]
  fn generate_key_without_namespace() {
    let options = UuidV5Generator {
      batch: 2,
      namespace: None,
      ..types::Default::default()
    };

    let result = UuidV5Generator::gen(&options);
    println!("{:?}", result);
    assert!(result.contains("-"))
  }

  #[test]
  fn generate_key_batch() {
    let options = UuidV5Generator {
      batch: 2,
      namespace: None,
      ..types::Default::default()
    };

    let result = Utility::batch(&options);
    println!("{:?}", result);
    assert!(result.contains("-"))
  }
}
