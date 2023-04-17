pub fn remove_comments(input: &str) -> String {
  let mut output = String::new();
  let per_line = input.lines();

  let mut multiline_comment = false;

  for line in per_line {
    let mut line = line.to_string();
    let sinleline_comment = line.find("//");
    let multiline_comment_start = line.find("/*");

    if let Some(index) = multiline_comment_start {
      let index_end = line.find("*/");

      if let Some(index_end) = index_end {
        line = format!("{}{}", &line[..index], &line[index_end + 2..]);
      } else {
        line = line[..index].to_string();
        multiline_comment = true;
      }
    } else if multiline_comment {
      let index = line.find("*/");

      if let Some(index) = index {
        line = line[index + 2..].to_string();
        multiline_comment = false;
      } else {
        line = "".to_string();
      }
    }

    if let Some(index) = sinleline_comment {
      line.truncate(index);
    }

    output.push_str(&line);
  }

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
    let input = "/* This is a // comment */val lol = 1";
    let expected = "val lol = 1";
    let actual = remove_comments(input);
    assert_eq!(expected, actual);
  }
}