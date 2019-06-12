mod parser;
mod values;

pub use parser::errors::ParserError;
pub use values::Token;

/// Parses the json string passed as parameter and returns a Result with the Token Enum type
/// 
/// # Examples
/// Here is an example of parsing json containing an object
/// 
/// ```
/// let result = parse_json(r#"{"a": 1}"#).unwrap();
///    if let Token::Object(object) = result {
///      assert_eq!(object.values.len(), 1);
///    } else {
///      panic!("Token parsed is not a Token::Object()");
///    }
/// ```
pub fn parse_json(json_string: &str) -> Result<Token, ParserError> {
  let mut parser = parser::JSONParser::new(json_string);
  parser.parse()
}

#[cfg(test)]
mod tests {
  use super::parse_json;
  use super::values::Token;

  #[test]
  fn test_parsing_works() {
    let result = parse_json(r#"{"a": 1}"#).unwrap();
    if let Token::Object(object) = result {
      assert_eq!(object.values.len(), 1);
    } else {
      panic!("Token parsed is not a Token::Object()");
    }
  }

  #[test]
  fn test_parsing_fails_for_incorrect_value() {
    let result = std::panic::catch_unwind(|| parse_json(r#"{a: 1}"#).unwrap());
    assert!(result.is_err());
  } 
}