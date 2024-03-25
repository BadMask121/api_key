/**
 * Generation options for the "string" method.
*/
#[derive(Clone, Debug)]
pub struct StringGenerator {
  pub min: u8,
  pub max: u8,
  pub length: u8,
  pub pool: String,
  pub prefix: String,
  pub batch: u8,
}

/**
 * Generation options for the "bytes" method.
*/
#[derive(Clone, Debug)]
pub struct BytesGenerator {
  pub min: u8,
  pub max: u8,
  pub length: u8,
  pub prefix: String,
  pub batch: u8,
}

/**
 * Generation options for the "base32" method.
*/
#[derive(Clone, Debug)]
pub struct Base32Generator {
  pub dashes: bool,
  pub prefix: String,
  pub batch: u8,
}

/**
 * Generation options for the "base62" method.
*/
#[derive(Clone, Debug)]
pub struct Base62Generator {
  pub batch: u8,
}

/**
 * Generation options for the "uuidv4" method.
*/
#[derive(Clone, Debug)]
pub struct UuidV4Generator {
  pub dashes: bool,
  pub prefix: String,
  pub batch: u8,
}

/**
 * Generation options for the "uuidv5" method.
*/
#[derive(Clone, Debug)]
pub struct UuidV5Generator {
  pub dashes: bool,
  pub prefix: String,
  pub batch: u8,
  pub namespace: Option<[u8; 16]>,
}

pub trait Default {
  fn default() -> Self;
}

#[derive(Clone, Debug)]
pub enum ApiKeyResults {
  String(String),
  StringArray(Vec<String>),
}
