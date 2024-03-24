// Character pools.
pub const LOWER_CHAR_POOL: &str = "abcdefghijklmnopqrstuvwxyz";
pub const UPPER_CHAR_POOL: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const BASE_32_CHAR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
pub const HEX_CHAR: &str = "0123456789abcdef";
pub const NUMBER_CHAR_POOL: &str = "0123456789";
pub const SPECIAL_CHAR_POOL: &str = "-._~+/";

/**
 * The default minimum length for an API key.
 */
pub const DEFAULT_MIN_LENGTH: u8 = 16;

/**
 * The default maximum length for an API key.
 */
pub const DEFAULT_MAX_LENGTH: u8 = 32;

pub fn get_default_character() -> String {
  [
    LOWER_CHAR_POOL,
    UPPER_CHAR_POOL,
    NUMBER_CHAR_POOL,
    SPECIAL_CHAR_POOL,
  ]
  .map(|c| c.to_string())
  .concat()
}

/**
 * Base62 character pool.
 */
pub fn get_base62_char_pool() -> String {
  [NUMBER_CHAR_POOL, LOWER_CHAR_POOL, UPPER_CHAR_POOL]
    .map(|c| c.to_string())
    .concat()
}
