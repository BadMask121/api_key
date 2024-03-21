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
pub struct StringGenerationOptions {
  pub min: Option<u8>,
  pub max: Option<u8>,
  pub length: Option<u8>,
  pub pool: Option<String>,
  pub prefix: Option<String>,
  // pub batch: String,
}

/**
 * Generation options for the "bytes" method.
 */
#[derive(Clone)]
pub struct BytesGenerationOptions  {
  pub min: Option<u8>,
  pub max: Option<u8>,
  pub length: Option<u8>,
  pub prefix: String,
  // pub batch: String,
}

/**
 * Generation options for the "base32" method.
 */
#[derive(Clone)]
pub struct Base32GenerationOptions {
  pub dashes: bool,
  pub prefix: String,
  // pub batch: String,
}

/**
 * Generation options for the "uuidv4" method.
 */
#[derive(Clone)]
pub struct UuidV4GenerationOptions {
  pub dashes: bool,
  pub prefix: String,
  // pub batch: String,
}

/**
 * Generation options for the "uuidv5" method.
 */
#[derive(Clone)]
pub struct UuidV5GenerationOptions {
  pub dashes: bool,
  pub prefix: String,
  // pub batch: String,
  pub name: String,
  pub namespace: String,
}