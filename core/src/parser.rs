pub struct Parser;

impl Parser {
  pub fn build(searchstring: String) -> Vec<String> {
    Vec::from_iter(
      searchstring.split_whitespace()
      .map(String::from)
      .map(|item| item.replace(",", ""))
      .filter(|item| item != "g.")
      .filter(|item| item != "Šv")
      .filter(|item| item != "k.")
      .filter(|item| item != "vs.")
      .filter(|item| item != "sen.")
      .filter(|item| item != "g.")
    )
  }
}

#[cfg(test)]
mod tests {
  use super::Parser;

  #[test]
  fn test_space_extract() {
    let parsed = Parser::build("saulu g. naudis".to_string());
    assert_eq!(parsed[1], "naudis".to_string());
  }

  #[test]
  fn test_comma_remove() {
    let parsed = Parser::build("saulu, g. naudis".to_string());
    assert_eq!(parsed[1], "naudis".to_string());
    let parsed2 = Parser::build("saulu, g., naudis".to_string());
    assert_eq!(parsed2[0], "saulu".to_string());
    assert_eq!(parsed2[1], "naudis".to_string());
  }
}