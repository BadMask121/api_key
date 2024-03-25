#![allow(dead_code)]
#[warn(unused_comparisons)]

mod constants;
mod types;
mod utils;

mod base32;
mod base62;
mod bytes;
mod string;
mod uuid4;
mod uuid5;

pub mod api_key {
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
    if options.batch_len() > 0 {
      ApiKeyResults::StringArray(Utility::batch(&options))
    } else {
      ApiKeyResults::String(options.gen())
    }
  }
}

#[cfg(test)]
mod api_key_test {
  use crate::{
    api_key,
    types::{ApiKeyResults, Default, StringGenerator, UuidV4Generator},
  };

  #[test]
  fn generate_batch_string_api_key() {
    let options = StringGenerator {
      length: 2,
      batch: 3,
      ..StringGenerator::default()
    };

    let key: ApiKeyResults = api_key::string(options);

    assert!(match key {
      ApiKeyResults::StringArray(d) => d.len() == 3,
      _ => false,
    })
  }

  #[test]
  fn generate_string_api_key() {
    let options = StringGenerator {
      length: 2,
      prefix: String::from("PREFIX"),
      ..StringGenerator::default()
    };

    let key: ApiKeyResults = api_key::string(options);

    assert!(match key {
      ApiKeyResults::String(d) => d.starts_with("PREFIX"),
      _ => false,
    })
  }

  #[test]
  fn generate_uuid4_api_key() {
    let options = UuidV4Generator {
      prefix: String::from("PREFIX"),
      ..UuidV4Generator::default()
    };

    let key: ApiKeyResults = api_key::uuid4(options);

    assert!(match key {
      ApiKeyResults::String(res) => res.starts_with("PREFIX") && res.contains("-"),
      _ => false,
    })
  }
}
