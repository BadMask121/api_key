#![feature(const_trait_impl)]

mod constants;
mod utils;
mod types;
use crate::types::{ StringGenerationOptions };

pub fn random_string(_options: &StringGenerationOptions) -> String {
    
    generate_string_key()
  }

  fn generate_string_key() ->String {
    "hello".to_string()
  }

