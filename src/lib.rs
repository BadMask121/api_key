#![allow(dead_code)]

mod constants;
mod types;
mod utils;

mod base32;
mod base62;
mod bytes;
mod string;
mod uuid4;
mod uuid5;

pub mod generate_api_keys {
  use crate::{
    types::{
      ApiKeyResults, Base32Generator, Base62Generator, BytesGenerator, StringGenerator,
      UuidV4Generator, UuidV5Generator,
    },
    utils::Utility,
  };

  pub fn string(options: StringGenerator) -> ApiKeyResults {
    get_result(options)
  }

  pub fn bytes(options: BytesGenerator) -> ApiKeyResults {
    get_result(options)
  }

  pub fn base32(options: Base32Generator) -> ApiKeyResults {
    get_result(options)
  }

  // pub fn base62(options: Base62Generator) -> ApiKeyResults {
  //   get_result(options)
  // }

  pub fn uuid4(options: UuidV4Generator) -> ApiKeyResults {
    get_result(options)
  }

  pub fn uuid5(options: UuidV5Generator) -> ApiKeyResults {
    get_result(options)

  }

  fn get_result<T: Utility>(options: T) -> ApiKeyResults {
    match options.batch_len() > 0 {
      true => ApiKeyResults::String(Utility::batch(&options)),
      false => ApiKeyResults::String(options.generate()),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    generate_api_keys,
    types::{Default, UuidV5Generator},
  };

  #[test]
  fn generate_string_api_key() -> String {
    let api_key: String = generate_api_keys::string(options).into();
    println!("{:?}", api_key);
  }

  #[test]
  fn generate_uuid5_api_key() -> String {
    let options = UuidV5Generator {
      batch: 2,
      ..Default::default()
    };

    let api_key: String = generate_api_keys::uuid5(options).into();
    println!("{:?}", api_key);
  }
}
