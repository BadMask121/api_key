#![allow(
dead_code,
unused_imports
)]

mod constants;
mod types;

mod generate_api_keys {
  use rand::Rng;
  use crate::{constants::{get_default_character, DEFAULT_MAX_LENGTH, DEFAULT_MIN_LENGTH}, types::StringGenerationOptions};
  

  pub fn random_string(options: &StringGenerationOptions) -> String {

    let default_pool = get_default_character();
    let min = options.min.unwrap_or(DEFAULT_MIN_LENGTH);
    let max = options.max.unwrap_or(DEFAULT_MAX_LENGTH);

    let pool= match &options.pool {
        Some(s) => s,
        None => &default_pool
    };

    let prefix = options.prefix.clone().unwrap_or("".to_string());
    let length = match options.length {
        Some(l) => l,
        None => {
          rand::thread_rng().gen_range(min ..=max)
        }
    };

    let mut g = generate_random_string(pool,  length);

    add_prefix( &mut g, &prefix)
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

  fn add_prefix(start_string: &mut String, concat: &String) -> String {
    start_string.insert_str(0, &concat);

    start_string.to_string()
  }
}

#[cfg(test)]
mod tests{
  use crate::{generate_api_keys::random_string, types::StringGenerationOptions};

  #[test]
  fn test_random_string_with_prefix(){
    let options = StringGenerationOptions {
      length:None,
      min: None,
      max: None,
      pool: None,
      prefix: Some(String::from("PREFIX-")),
    };

    let result = random_string(&options);
    assert!(&result.starts_with("PREFIX"));
  }

  #[test]
  fn test_random_string_with_min_max() {
    let options = StringGenerationOptions {
      length:None,
      min: Some(5),
      max: Some(8),
      pool: None,
      prefix: None,
    };

    let result = random_string(&options);
    let result_length = result.len();
    assert!(result_length >= 5 || result_length >= 8);
  }

  #[test]
  fn test_random_string_with_length() {
    let options = StringGenerationOptions {
      length:Some(10),
      min: None,
      max: None,
      pool: None,
      prefix: None,
    };

    let result = random_string(&options);

    assert!(result.len() >= 10);
  }
}