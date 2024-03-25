extern crate api_key;

use api_key::types::{ApiKeyResults, Default, UuidV4Generator};

fn main() {
  let api = api_key::uuid4(UuidV4Generator {
    batch: 2,
    ..UuidV4Generator::default()
  });

  assert!(match api {
    ApiKeyResults::StringArray(d) => d.len() == 2,
    _ => false,
  })
}
