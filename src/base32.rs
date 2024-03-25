use crate::{
  types::{Base32Generator, Default},
  utils::{add_prefix, random_16_bytes, to_base32, uuid4},
};
use regex::Regex;

impl Default for Base32Generator {
  fn default() -> Self {
    Base32Generator {
      prefix: "".to_string(),
      dashes: false,
      batch: 0,
    }
  }
}

/**
 * Base32 using Crockford hashing
*/
impl Base32Generator {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn gen(&self) -> String {
    let bytes: [u8; 16] = random_16_bytes();
    let uuid = uuid4(bytes);

    // split uuid into different parts
    let uuid_parts = uuid.split("-").into_iter();
    //unwrap into vectors
    let uuid_vec: Vec<&str> = uuid_parts.map(|char| char).collect();

    if let [first_part, second_part, third_part, fourth_part, last_part] = uuid_vec[..] {
      // build parts
      let parts = [
        first_part,
        &(second_part.to_owned() + third_part),
        &(fourth_part.to_owned() + &last_part[..4]),
        &last_part[..4],
      ];

      let key = parts.map(|part| {
        let every_two_character = Regex::new(r".{1,2}").unwrap();
        let cap_characters = &every_two_character.captures(part).unwrap();

        // convert two character into number for base32
        let based_num: Vec<u8> = cap_characters
          .iter()
          .map(|cap| {
            let char = cap.unwrap().as_str();
            i32::from_str_radix(char, 16).unwrap() as u8
          })
          .into_iter()
          .collect();

        //create base32 string
        to_base32(&based_num.as_slice())
      });

      if key.len() > 0 {
        let mut api_key = match self.dashes {
          true => key.join("-"),
          _ => key.join(""),
        };

        add_prefix(&mut api_key, &self.prefix)
      } else {
        panic!("unable to generate base32 api key")
      }
    } else {
      panic!("unable to generate base32 api key")
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::types::{ Base32Generator, Default};

  #[test]
  fn generate_key_with_prefix() {
    let options = Base32Generator {
      prefix: String::from("PREFIX-"),
      ..Base32Generator::default()
    };

    let result = Base32Generator::gen(&options);
    assert!(result.starts_with("PREFIX-"))
  }

  #[test]
  fn generate_key_with_dashes() {
    let options = Base32Generator {
      dashes: true,
      ..Base32Generator::default()
    };

    let result = Base32Generator::gen(&options);
    assert!(result.contains("-"))
  }
}
