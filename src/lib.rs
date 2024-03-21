mod constants;
mod types;

mod generate_api_keys {
  use rand::Rng;
  use crate::{constants::{get_default_character, DEFAULT_MAX_LENGTH, DEFAULT_MIN_LENGTH}, types::StringGenerationOptions};
  

  pub fn random_string(options: &StringGenerationOptions) -> String {

    let default_pool = get_default_character();

    let min = options.min.unwrap_or(DEFAULT_MIN_LENGTH);
    let max = options.min.unwrap_or(DEFAULT_MAX_LENGTH);
    let pool= match &options.pool {
        Some(s) => s,
        None => &default_pool
    };
    let prefix = &options.prefix.clone().unwrap();
    let mut g = generate_random_string(pool,  min,  max);

    add_prefix( &mut g, &prefix)
  }
  
  fn generate_random_string(pool: &String, min_length: u8, max_length: u8) -> String {
      let mut rng = rand::thread_rng();
      let length = rng.gen_range(min_length ..max_length + 1);
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
  fn test_random_string(){
    let options = StringGenerationOptions {
      length:None,
      max: None,
      min: None,
      pool: None,
      prefix: Some(String::from("PREFIX-")),
    };

    let result = random_string(&options);

    assert!(&result.starts_with("PREFIX"))
  }
}