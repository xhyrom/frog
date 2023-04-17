mod comments;
mod macros;

pub fn preprocess(input: &str) -> String {
  let mut output = String::new();

  output = comments::remove_comments(input);
  output = macros::replace_macros(&output);

  output
}