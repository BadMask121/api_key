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
  use crate::{ random_bytes, random_string, types::{BytesGenerationOptions, StringGenerationOptions, Default}};

  fn string(options: StringGenerationOptions) -> String {
    StringGenerationOptions::gen(&options)
  }

  fn bytes(options: BytesGenerationOptions) -> String {
    BytesGenerationOptions::gen(&options)
  }
}