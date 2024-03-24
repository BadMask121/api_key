use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

use crate::constants::{BASE_32_CHAR, HEX_CHAR};

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
