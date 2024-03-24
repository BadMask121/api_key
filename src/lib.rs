#![allow(
dead_code,
unused_imports
)]

mod constants;
mod types;
mod utils;


mod string;
mod bytes;
mod base32;

pub mod generate_api_keys {
  use crate:: types::{BytesGenerator, GenerationMethods, StringGenerator};

  fn string(options: StringGenerator) -> String {
    StringGenerator::gen(&options)
  }

  fn bytes(options: BytesGenerator) -> String {
    BytesGenerator::gen(&options)
  }
}
