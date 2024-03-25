use rand::Rng;

use crate::{
  constants::HEX_CHAR,
  types::{
    Base32Generator, Base62Generator, BytesGenerator, StringGenerator, UuidV4Generator,
    UuidV5Generator,
  },
};
use uuid::{Builder, Variant, Version};

pub fn add_prefix(start_string: &mut String, concat: &String) -> String {
  start_string.insert_str(0, &concat);
  start_string.to_string()
}

pub fn random_bytes(length: usize) -> Vec<u8> {
  (0..length).map(|_| rand::thread_rng().gen()).collect()
}

pub fn to_base32(data: &[u8]) -> String {
  base32::encode(base32::Alphabet::Crockford, data)
}

pub fn to_hex(data: &[u8]) -> String {
  let hex_chars: &[u8] = HEX_CHAR.as_bytes();
  let mut result = String::with_capacity(data.len() * 2);

  for &byte in data {
    let hi = byte >> 4;
    let lo = byte & 0xF;
    result.push(hex_chars[hi as usize] as char);
    result.push(hex_chars[lo as usize] as char);
  }

  result
}

pub fn random_16_bytes() -> [u8; 16] {
  random_bytes(16).try_into().unwrap()
}

pub fn uuid4(bytes: [u8; 16]) -> String {
  Builder::from_bytes(bytes)
    .with_variant(Variant::RFC4122)
    .with_version(Version::Random)
    .into_uuid()
    .to_string()
}

pub fn uuid5(bytes: [u8; 16]) -> String {
  Builder::from_bytes(bytes)
    .with_variant(Variant::RFC4122)
    .with_version(Version::Sha1)
    .into_uuid()
    .to_string()
}

pub trait Utility {
  fn batch(&self) -> String {
    (0..self.batch_len()).map(|_| self.generate()).collect()
  }

  /**
   * generator function
   */
  fn generate(&self) -> String;

  /**
   * batch length
   */
  fn batch_len(&self) -> u8;
}

/**
 * Utility trait declaration
*/
impl Utility for UuidV5Generator {
  fn generate(&self) -> String {
    self.gen()
  }

  fn batch_len(&self) -> u8 {
    self.batch
  }
}

impl Utility for UuidV4Generator {
  fn generate(&self) -> String {
    self.gen()
  }

  fn batch_len(&self) -> u8 {
    self.batch
  }
}

impl Utility for StringGenerator {
  fn generate(&self) -> String {
    self.gen()
  }

  fn batch_len(&self) -> u8 {
    self.batch
  }
}

impl Utility for BytesGenerator {
  fn generate(&self) -> String {
    self.gen()
  }

  fn batch_len(&self) -> u8 {
    self.batch
  }
}

impl Utility for Base32Generator {
  fn generate(&self) -> String {
    self.gen()
  }

  fn batch_len(&self) -> u8 {
    self.batch
  }
}

// impl Utility for Base62Generator {
//   fn generate(&self) -> String {
//     self.gen()
//   }

//   fn batch_len(&self) -> u8 {
//     self.batch
//   }
// }
