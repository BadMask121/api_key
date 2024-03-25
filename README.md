# api_key
A Rust utility for generating API keys including random strings, UUIDs, and keys based on specific encoding schemes like Base32 and Base62.

Rust port of https://github.com/pauldenver/generate-api-key


# Example for generating random string with prefix

```rust
 let options = StringGenerator {
    prefix: String::from("PREFIX"),
    ..StringGenerator::default()
  };

  let key: ApiKeyResults = api_key::string(options);

  assert!(match key {
    ApiKeyResults::String(d) => d.starts_with("PREFIX"),
    _ => false,
  })
```
- [ ] improve crate documentation
- [ ] improve README