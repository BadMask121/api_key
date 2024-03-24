# api_key
A Rust utility for generating API keys including random strings, UUIDs, and keys based on specific encoding schemes like Base32 and Base62.

Rust port of https://github.com/pauldenver/generate-api-key


# Example for generating random string

```rust
let username = StringGenerator {
  min: 10,
  max: 20
};

username.gen();
```