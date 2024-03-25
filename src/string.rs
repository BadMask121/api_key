use rand::Rng;
use crate::{constants::{get_default_character, DEFAULT_MAX_LENGTH, DEFAULT_MIN_LENGTH}, types::{Default, StringGenerator}, utils::add_prefix};

impl Default for StringGenerator {
  fn default() -> Self {
        StringGenerator { 
          min: DEFAULT_MIN_LENGTH,
          max: DEFAULT_MAX_LENGTH,
          pool: get_default_character(),
          prefix: "".to_string(),
          length: None,
          batch: 0,
        }
    }
}

impl StringGenerator {
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

    add_prefix( &mut generate_random_string(&self.pool, length), &self.prefix)
  }
}

fn generate_random_string(pool: &String, length: u8) -> String {
  let mut rng = rand::thread_rng();
  (0..length)
      .map(|_| {
          let idx = rng.gen_range(0..pool.len());
          pool.chars().nth(idx).unwrap()
      })
      .collect()
}

#[cfg(test)]
mod tests {
  use crate::types::{self, StringGenerator};

  #[test]
  fn test_random_string_with_prefix(){
    let options = StringGenerator {
      prefix: String::from("PREFIX-"),
      ..types::Default::default()
    };

    let result = options.gen();
    assert!(&result.starts_with("PREFIX"));
  }

  #[test]
  fn test_random_string_with_min_max() {
    let options = StringGenerator {
      min: 5,
      max: 8,
      ..types::Default::default()
    };

    let result = options.gen();
    let result_length = result.len();
    assert!(result_length >= 5 && result_length <= 8);
  }

  #[test]
  fn test_random_string_with_length() {
    let options = StringGenerator {
      length: Some(10),
      ..types::Default::default()
    };

    let result = options.gen();

    assert!(result.len() >= 10);
  }

  #[test]
  fn test_random_string_with_defaults() {
    let options = StringGenerator::new();
    let result = options.gen();

    assert!(result.len() >= 16 && result.len() <= 32 );
  }
}
