use rand::Rng;
use hex;
use crate::{constants::{get_default_character, DEFAULT_MAX_LENGTH, DEFAULT_MIN_LENGTH}, types::{BytesGenerationOptions, Default }, utils::add_prefix};

impl Default for BytesGenerationOptions {
    fn default() -> Self {
        BytesGenerationOptions { 
          min: DEFAULT_MIN_LENGTH,
          max: DEFAULT_MAX_LENGTH,
          prefix: "".to_string(),
          length: None
        }
    }
}

impl BytesGenerationOptions {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn gen(&self) -> String {
    let length = match self.length {
          Some(l) => l,
          None => {
            rand::thread_rng().gen_range(self.min ..=self.max)
          }
      };

    add_prefix( &mut generate_random_bytes(length), &self.prefix)
  }
}

fn generate_random_bytes(length: u8) -> String {

  let random_bytes: Vec<u8> = (0..length).map(|_| rand::thread_rng().gen()).collect();

  // Convert bytes to hexadecimal string
  hex::encode(&random_bytes)
}

#[cfg(test)]
mod tests {
  use crate::types::{self, BytesGenerationOptions};

  #[test]
  fn test_random_bytes_with_prefix(){
    let options = BytesGenerationOptions {
      prefix: String::from("PREFIX-"),
      ..types::Default::default()
    };

    let result = BytesGenerationOptions::gen(&options);
    assert!(&result.starts_with("PREFIX"));
  }

  #[test]
  fn test_random_bytes_with_min_max() {
    let options = BytesGenerationOptions {
      min: 10,
      max: 20,
      ..types::Default::default()
    };

    let result = BytesGenerationOptions::gen(&options);
    let result_length = result.len() / 2;

    assert!(result_length >= 10 && result_length <= 20);
  }

  #[test]
  fn test_random_bytes_with_length() {
    let options = BytesGenerationOptions {
      length: Some(10),
      ..types::Default::default()
    };

    let result = BytesGenerationOptions::gen(&options);

    assert!(result.len() >= 10);
  }
}