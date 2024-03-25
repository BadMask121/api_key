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
    build(options)
  }

  pub fn bytes(options: BytesGenerator) -> ApiKeyResults {
    build(options)
  }

  pub fn base32(options: Base32Generator) -> ApiKeyResults {
    build(options)
  }

  pub fn base62(options: Base62Generator) -> ApiKeyResults {
    build(options)
  }

  pub fn uuid4(options: UuidV4Generator) -> ApiKeyResults {
    build(options)
  }

  pub fn uuid5(options: UuidV5Generator) -> ApiKeyResults {
    build(options)
  }

  fn build<T: Utility>(options: T) -> ApiKeyResults {
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
    types::{ApiKeyResults, Default, StringGenerator, UuidV4Generator},
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

  #[test]
  fn generate_uuid4_api_key() {
    let options = UuidV4Generator {
      prefix: String::from("PREFIX"),
      ..Default::default()
    };

    let api_key: ApiKeyResults = generate_api_keys::uuid4(options);

    assert!(match api_key {
      ApiKeyResults::String(res) => res.starts_with("PREFIX") && res.contains("-"),
      _ => false,
    })
  }
}
