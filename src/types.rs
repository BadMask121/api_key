// Array of generation methods.
#[derive(Clone, Debug)]
pub enum GenerationMethods{
  String,
  Bytes,
  Base32,
  Base62,
  Uuidv4,
  Uuidv5
}

/**
 * Generation options for the "string" method.
 */
#[derive(Clone, Debug)]
pub struct StringGenerator {
  pub min: u8,
  pub max: u8,
  pub length: Option<u8>,
  pub pool: String,
  pub prefix: String,
  // pub batch: String,
}

/**
 * Generation options for the "bytes" method.
 */
#[derive(Clone)]
pub struct BytesGenerator  {
  pub min: u8,
  pub max: u8,
  pub length: Option<u8>,
  pub prefix: String,
  // pub batch: String,
}

/**
 * Generation options for the "base32" method.
 */
#[derive(Clone)]
pub struct Base32Generator {
  pub dashes: bool,
  pub prefix: String,
  // pub batch: String,
}

/**
 * Generation options for the "uuidv4" method.
 */
#[derive(Clone)]
pub struct UuidV4Generator {
  pub dashes: bool,
  pub prefix: String,
  // pub batch: String,
}

/**
 * Generation options for the "uuidv5" method.
 */
#[derive(Clone)]
pub struct UuidV5Generator {
  pub dashes: bool,
  pub prefix: String,
  // pub batch: String,
  pub name: String,
  pub namespace: String,
}

pub trait Default {
  fn default() -> Self;
}