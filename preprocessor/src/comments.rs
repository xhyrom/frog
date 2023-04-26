use regex::Regex;

pub fn remove_comments(input: &str) -> String {
  let mut output = input.to_string();

  let single_line_regex = Regex::new(r"(?m)\/\/.*").unwrap();
  output = single_line_regex.replace_all(input, "").to_string();

  let multiline_regex = Regex::new(r"(?s)\/\*.*?\*\/").unwrap();
  output = multiline_regex.replace_all(&output, "").to_string();

  output
}

mod tests {
  use super::*;

  #[test]
  fn test_remove_comments_singleline_starts_with() {
    let input = "// This is a comment";
    let expected = "";
    let actual = remove_comments(input);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_remove_comments_singleline_ends_with() {
    let input = "val lol = 1 // This is a comment";
    let expected = "val lol = 1 ";
    let actual = remove_comments(input);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_remove_comments_multiline() {
    let input = "/* This is a comment */";
    let expected = "";
    let actual = remove_comments(input);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_remove_comments_multiline_with_code() {
    let input = "/* This is a comment */val lol = 1";
    let expected = "val lol = 1";
    let actual = remove_comments(input);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_remove_comments_multiline_with_multilines() {
    let input = "/* This is\n\n\n a comment */val lol = 1";
    let expected = "val lol = 1";
    let actual = remove_comments(input);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_remove_comments_singleline_in_multiline() {
    // TODO: fix this, comment in multiline comment throws error
    let input = "/* This is a // comment */val lol = 1";
    let expected = "val lol = 1";
    let actual = remove_comments(input);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_remove_comments_more_lines() {
    let input = "/* This is a */val lol = 1\nval x = 8";
    let expected = "val lol = 1\nval x = 8";
    let actual = remove_comments(input);
    assert_eq!(expected, actual);
  }
}