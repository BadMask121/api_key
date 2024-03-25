use crate::{
  constants::{DEFAULT_MAX_LENGTH, DEFAULT_MIN_LENGTH},
  types::{BytesGenerator, Default},
  utils::{add_prefix, random_bytes, to_hex},
};
use rand::Rng;

impl Default for BytesGenerator {
  fn default() -> Self {
    BytesGenerator {
      min: DEFAULT_MIN_LENGTH,
      max: DEFAULT_MAX_LENGTH,
      prefix: "".to_string(),
      length: None,
      batch: 0,
    }
  }
}

impl BytesGenerator {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn gen(&self) -> String {
    let length: usize = match self.length {
      Some(l) => l.into(),
      None => rand::thread_rng().gen_range(self.min..=self.max).into(),
    };

    // splice byte string to same amount of length
    // since bytes string == length * 2

    let byte_splice = &generate_random_bytes(length)[..length];

    add_prefix(&mut String::from(byte_splice), &self.prefix)
  }
}

fn generate_random_bytes(length: usize) -> String {
  // Convert bytes to hexadecimal string
  to_hex(&random_bytes(length))
}

#[cfg(test)]
mod tests {
  use crate::types::{self, BytesGenerator};

  #[test]
  fn test_random_bytes_with_prefix() {
    let options = BytesGenerator {
      prefix: String::from("PREFIX-"),
      ..types::Default::default()
    };

    let result = BytesGenerator::gen(&options);
    assert!(&result.starts_with("PREFIX"));
  }

  #[test]
  fn test_random_bytes_with_min_max() {
    let options = BytesGenerator {
      min: 10,
      max: 20,
      ..types::Default::default()
    };

    let result = BytesGenerator::gen(&options);
    let result_length = result.len();

    assert!(result_length >= 10 && result_length <= 20);
  }

  #[test]
  fn test_random_bytes_with_length() {
    let options = BytesGenerator {
      length: Some(10),
      ..types::Default::default()
    };

    let result = BytesGenerator::gen(&options);

    assert!(result.len() >= 10);
  }
}
