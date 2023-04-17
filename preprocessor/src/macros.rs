use std::collections::HashMap;

pub fn replace_macros(input: &str) -> String {
  let mut output = String::new();

  let input_without_define: Vec<String> = input.lines()
    .filter(|line| !line.starts_with("#define"))
    .map(|line| line.to_string())
    .collect();

  let mut defines_map = HashMap::new();

  for line in input.lines().filter(|line| line.starts_with("#define")) {
    let define = line.replace("#define ", "");
    let define = define.split(" ").collect::<Vec<&str>>();
    let name = define[0].to_string();
    let value = define[1].to_string();
    defines_map.insert(name, value);
  }

  for (i, line) in input_without_define.iter().enumerate() {
    let mut line = line.to_string();

    for (name, value) in defines_map.iter() {
      line = line.replace(name, value);
    }

    output.push_str(&line);

    if i != input_without_define.len() - 1 {
      output.push_str("\n");
    }
  }

  output
}


mod tests {
  use super::*;

  #[test]
  fn test_replace_macros() {
    let input = "#define MAX_VALUE 40\nprint(MAX_VALUE)";
    let expected = "print(40)";
    let actual = replace_macros(input);
    assert_eq!(expected, actual); 
  }

  #[test]
  fn test_replace_macros_with_multiple_lines() {
    let input = "#define MAX_VALUE 40\nprint(MAX_VALUE)\nprint(MAX_VALUE)";
    let expected = "print(40)\nprint(40)";
    let actual = replace_macros(input);
    assert_eq!(expected, actual);
  }
}