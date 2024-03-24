pub fn add_prefix(start_string: &mut String, concat: &String) -> String {
  start_string.insert_str(0, &concat);
  start_string.to_string()
}