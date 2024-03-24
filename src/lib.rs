#![allow(
dead_code,
unused_imports
)]

mod constants;
mod types;
mod utils;


mod random_string;
mod random_bytes;

pub mod generate_api_keys {
  use std::mem::ManuallyDrop;

  use crate:: types::{BytesGenerator, GenerationMethods, StringGenerator};

  fn string(options: StringGenerator) -> String {
    StringGenerator::gen(&options)
  }

  fn bytes(options: BytesGenerator) -> String {
    BytesGenerator::gen(&options)
  }
}
