#![allow(dead_code, unused_imports)]

mod constants;
mod types;
mod utils;

mod base32;
mod base62;
mod bytes;
mod string;

pub mod generate_api_keys {
  use crate::{
    base62,
    types::{Base32Generator, BytesGenerator, GenerationMethods, StringGenerator},
  };

  fn string(options: StringGenerator) -> String {
    StringGenerator::gen(&options)
  }

  fn bytes(options: BytesGenerator) -> String {
    BytesGenerator::gen(&options)
  }

  fn base32(options: Base32Generator) -> String {
    Base32Generator::gen(&options)
  }

  fn base62() -> String {
    base62::gen()
  }
}
