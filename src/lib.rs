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

  pub fn base62(options: Base62Generator) -> ApiKeyResults {
    get_result(options)
  }

  pub fn uuid4(options: UuidV4Generator) -> ApiKeyResults {
    get_result(options)
  }

  pub fn uuid5(options: UuidV5Generator) -> ApiKeyResults {
    get_result(options)
  }

  fn get_result<T: Utility>(options: T) -> ApiKeyResults {
    match options.batch_len() > 0 {
      true => ApiKeyResults::StringArray(Utility::batch(&options)),
      false => ApiKeyResults::String(options.gen()),
    }
  }
}

#[cfg(test)]
mod api_key_test {
  use crate::{
    generate_api_keys,
    types::{ApiKeyResults, Default, StringGenerator},
  };

  #[test]
  fn generate_batch_string_api_key() {
    let options = StringGenerator {
      length: Some(2),
      batch: 3,
      ..Default::default()
    };

    let api_key: ApiKeyResults = generate_api_keys::string(options);

    assert!(match api_key {
      ApiKeyResults::StringArray(d) => d.len() == 3,
      _ => false,
    })
  }

  #[test]
  fn generate_string_api_key() {
    let options = StringGenerator {
      length: Some(2),
      prefix: String::from("PREFIX"),
      ..Default::default()
    };

    let api_key: ApiKeyResults = generate_api_keys::string(options);

    assert!(match api_key {
      ApiKeyResults::String(d) => d.starts_with("PREFIX"),
      _ => false,
    })
  }
}
