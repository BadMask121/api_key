use rand::Rng;
use hex;
use crate::{constants::{get_default_character, DEFAULT_MAX_LENGTH, DEFAULT_MIN_LENGTH}, types::{BytesGenerator, Default }, utils::add_prefix};

impl Default for BytesGenerator {
    fn default() -> Self {
        BytesGenerator { 
          min: DEFAULT_MIN_LENGTH,
          max: DEFAULT_MAX_LENGTH,
          prefix: "".to_string(),
          length: None
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
    let length:usize = match self.length {
          Some(l) => l.into(),
          None => {
            rand::thread_rng().gen_range(self.min ..=self.max).into()
          }
      };

    // splice byte string to same amount of length
    // since bytes string == length * 2
    
    let byte_splice = &generate_random_bytes(length)[..length];

    add_prefix( &mut String::from(byte_splice), &self.prefix)
  }
}

fn generate_random_bytes(length: usize) -> String {

  let random_bytes: Vec<u8> = (0..length).map(|_| rand::thread_rng().gen()).collect();

  // Convert bytes to hexadecimal string
  hex::encode(&random_bytes)
}

#[cfg(test)]
mod tests {
  use crate::types::{self, BytesGenerator};

  #[test]
  fn test_random_bytes_with_prefix(){
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