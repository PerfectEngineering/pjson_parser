use std::iter::Peekable;
use std::str::CharIndices;

use crate::values::constants::*;
use crate::values::ArrayToken;
use crate::values::NumberToken;
use crate::values::ObjectToken;
use crate::values::ObjectTokenPair;
use crate::values::StringToken;
use crate::values::Token;

pub mod errors;

use errors::ParserError;

pub struct JSONParser<'ja> {
  current_position: usize,
  prev_char: Option<char>,
  current_char: Option<char>,
  json_iter: Peekable<CharIndices<'ja>>,
}

impl<'ja> JSONParser<'ja> {
  pub fn new(json_string: &'ja str) -> JSONParser<'ja> {
    let mut json_iter = json_string.trim().char_indices().peekable();

    JSONParser {
      current_position: 0,
      prev_char: None,
      current_char: match json_iter.next() {
        Some((_, c)) => Some(c),
        None => None,
      },
      json_iter,
    }
  }

  /// Parses the parse in character and returns a Token Enum which represents a graph of 
  /// all types in the graph
  /// Some drawbacks right now:
  /// - The parsing uses recursion, which can become really bad to deeply nested JSON
  /// - Does not map to rust Types (yet)
  pub fn parse(&mut self) -> Result<Token, ParserError> {
    self.parse_next_token()
  }

  fn parse_object(&mut self) -> Result<ObjectToken, ParserError> {
    let start = self.current_position;
    let optional_first_char = self.current_char;

    match optional_first_char {
      Some(first_char) if first_char == OBJECT_START_CHAR => {
        let mut pair_values = vec![];
        loop {
          self.next_char();
          self.skip_spaces();

          // handle empty objects
          if self.current_char == Some(OBJECT_END_CHAR) {
            break;
          }

          // parse key
          // key must be a string
          let key_string_token = self.parse_string()?;
          self.next_char();
          self.skip_spaces();

          match self.current_char {
            Some(OBJECT_PAIR_SEPARATOR_CHAR) => {
              self.next_char();
            }
            Some(c) => {
              return Err(ParserError::UnexpectedCharacterError(
                c,
                self.current_position,
              ));
            }
            None => {
              return Err(ParserError::UnexpectedEndError);
            }
          };
          self.skip_spaces();

          let value_token = self.parse_next_token()?;

          let object_token_pair = ObjectTokenPair {
            key: key_string_token.value,
            value: value_token,
          };
          pair_values.push(object_token_pair);

          // move to next possible character (,) while skipping white_spaces
          match self.peek_char() {
            Some(c) if *c == COMMA_CHAR => {
              self.next_char();
            }
            Some(c) if *c == SPACE_CHAR => {
              self.next_char();
              self.skip_spaces();
            }
            None => return Err(ParserError::UnexpectedEndError),
            _ => continue,
          }
        }

        Ok(ObjectToken {
          values: pair_values,
          start,
          end: self.current_position,
        })
      }
      _ => Err(ParserError::UnexpectedCharacterError(
          self.current_char.unwrap_or_default(),
          self.current_position)
        ),
    }
  }

  fn parse_array(&mut self) -> Result<ArrayToken, ParserError> {
    let start = self.current_position;
    let optional_first_char = self.current_char;

    match optional_first_char {
      Some(first_char) if first_char == ARRAY_START_CHAR => {
        let mut array_values = vec![];
        loop {
          self.next_char();
          self.skip_spaces();

          // handle empty arrays
          if self.current_char == Some(ARRAY_END_CHAR) {
            break;
          }

          let value_token = self.parse_next_token()?;

          array_values.push(value_token);

          // move to next possible character (,) while skipping white_spaces
          match self.peek_char() {
            Some(c) if *c == COMMA_CHAR => {
              self.next_char();
            }
            Some(c) if *c == SPACE_CHAR => {
              self.next_char();
              self.skip_spaces();
            }
            None => return Err(ParserError::UnexpectedEndError),
            _ => continue,
          }
        }

        Ok(ArrayToken {
          values: array_values,
          start,
          end: self.current_position,
        })
      }
      _ => Err(ParserError::UnexpectedCharacterError(
          self.current_char.unwrap_or_default(),
          self.current_position)
        ),
    }
  }

  fn parse_next_token(&mut self) -> Result<Token, ParserError> {
    match self.current_char {
      Some(c) if c.is_digit(10) => self.parse_number().map(|t| Token::from(t)),
      Some(STRING_START_CHAR) => self.parse_string().map(|t| Token::from(t)),
      Some(OBJECT_START_CHAR) => self.parse_object().map(|t| Token::from(t)),
      Some(ARRAY_START_CHAR) => self.parse_array().map(|t| Token::from(t)),
      None => Err(ParserError::UnexpectedEndError),
      _ => Err(ParserError::UnexpectedCharacterError(
        self.current_char.unwrap_or_default(),
        self.current_position,
      )),
    }
  }

  fn parse_number(&mut self) -> Result<NumberToken, ParserError> {
    let start = self.current_position;
    let optional_first_char = self.current_char;

    let mut value_str = String::new();

    match optional_first_char {
      Some(first_char) if first_char.is_digit(10) => {
        value_str.push(first_char);

        loop {
          match self.peek_char() {
            Some(c) if c.is_digit(10) || *c == DOT_CHAR => value_str.push(*c),
            _ => break,
          }
          self.next_char();
        }

        Ok(NumberToken {
          value: value_str,
          start,
          end: self.current_position,
        })
      }
      _ => Err(ParserError::UnexpectedEndError),
    }
  }

  fn parse_string(&mut self) -> Result<StringToken, ParserError> {
    let optional_first_char = self.current_char;

    match optional_first_char {
      Some(first_char) if first_char == STRING_START_CHAR => {
        self.next_char();
        let start = self.current_position;
        let mut value_str = String::new();
        let mut closing_char_found = false;

        loop {
          match self.current_char {
            None => break,
            Some(c) if c == STRING_END_CHAR => {
              // ensure previous character is not a skip '\'
              match self.prev_char {
                Some(prev_c) if prev_c == SKIP_CHAR => value_str.push(c),
                _ => {
                  closing_char_found = true;
                  break;
                }
              }
            }
            Some(c) => value_str.push(c),
          }
          self.next_char();
        }

        let mut end = self.current_position - 1;
        if start == self.current_position {
          end = start;
        }

        if closing_char_found {
          Ok(StringToken {
            value: value_str,
            start,
            end,
          })
        } else {
          Err(ParserError::UnexpectedEndError)
        }
      }
      None => Err(ParserError::UnexpectedEndError),
      _ => Err(ParserError::UnexpectedCharacterError(
        self.current_char.unwrap(),
        self.current_position,
      )),
    }
  }

  fn next_char(&mut self) -> Option<char> {
    match self.json_iter.next() {
      Some((pos, next_char)) => {
        self.prev_char = self.current_char;
        self.current_char = Some(next_char);

        self.current_position = pos;
        Some(next_char)
      }
      None => {
        self.prev_char = self.current_char;
        self.current_char = None;
        None
      }
    }
  }

  fn peek_char(&mut self) -> Option<&char> {
    match self.json_iter.peek() {
      Some((_, next_char)) => Some(next_char),
      None => None,
    }
  }

  fn skip_spaces(&mut self) {
    while self.current_char == Some(SPACE_CHAR) {
      self.next_char();
    }
  }
}

#[cfg(test)]
mod tests {
  use std::convert::TryInto;
  use super::JSONParser;
  use super::ObjectToken;
  use super::NumberToken;
  use super::StringToken;

  #[test]
  fn json_parser_parses_empty_array() {
    let mut parser = JSONParser::new("[]");
    let array_token = parser.parse_array().unwrap();
    assert_eq!(array_token.values.len(), 0);
  }

  #[test]
  fn json_parser_parses_array_with_different_types() {
    let mut parser = JSONParser::new(r#"[1, "good" ,{"a": 1}]"#);
    let array_token = parser.parse_array().unwrap();
    assert_eq!(array_token.values.len(), 3);

    let number_token: NumberToken = (&array_token.values[0]).clone()
      .try_into()
      .unwrap();
    assert_eq!(number_token.value, "1");

    let string_token: StringToken = (&array_token.values[1]).clone()
      .try_into()
      .unwrap();
    assert_eq!(string_token.value, "good");
  }

  #[test]
  fn json_parser_parses_empty_object() {
    let mut parser = JSONParser::new("{}");
    let object_token = parser.parse_object().unwrap();
    assert_eq!(object_token.values.len(), 0);
  }

   #[test]
  fn json_parser_parses_object_with_leading_and_trailing_spaces() {
    let mut parser = JSONParser::new("   {} ");
    let object_token = parser.parse_object().unwrap();
    assert_eq!(object_token.values.len(), 0);
  }

  #[test]
  fn json_parser_parses_object_with_single_pair() {
    let mut parser = JSONParser::new(r#"{"hello": 1}"#);
    let object_token = parser.parse_object().unwrap();
    assert_eq!(object_token.values.len(), 1);

    let mut parser = JSONParser::new(r#"{"name" : "james_bond"}"#);
    let object_token = parser.parse_object().unwrap();
    assert_eq!(object_token.values.len(), 1);
  }

  #[test]
  fn json_parser_parses_object_with_multiple_and_nested_pair() {
    let mut parser = JSONParser::new(r#"{"a":1 , "b": {"c":2.5}}"#);
    let object_token = parser.parse_object().unwrap();
    assert_eq!(object_token.values.len(), 2);

    // second token value should have one child and 2 as number
    let second_object_token = &object_token.values[1];
    assert_eq!(second_object_token.key, "b");

    let second_object_token_value_object_token: ObjectToken = second_object_token.value.clone()
      .try_into()
      .unwrap();
    
    assert_eq!(second_object_token_value_object_token.values.len(), 1);
    assert_eq!(second_object_token_value_object_token.values[0].key, "c");

    let second_number_token: NumberToken = second_object_token_value_object_token.values[0].value
      .clone()
      .try_into()
      .unwrap();
    assert_eq!(second_number_token.value, "2.5");
  }

  #[test]
  fn json_parser_parses_number() {
    let mut parser = JSONParser::new("1234");
    let number = parser.parse_number().unwrap();
    assert_eq!(number.value, "1234");
    assert_eq!(number.length(), 4);
  }

  #[test]
  fn json_parser_parses_floating_point_number() {
    let mut parser = JSONParser::new("12.34");
    let number = parser.parse_number().unwrap();
    assert_eq!(number.value, "12.34");
    assert_eq!(number.length(), 5);
  }

  #[test]
  fn json_parser_parses_zero_number() {
    let mut parser = JSONParser::new("0");
    let number = parser.parse_number().unwrap();
    assert_eq!(number.value, "0");
    assert_eq!(number.length(), 1);
  }

  #[test]
  fn json_parser_parses_string() {
    let mut parser = JSONParser::new("\"tell me more\"");
    let string = parser.parse_string().unwrap();
    assert_eq!(string.value, "tell me more");
    assert_eq!(string.length(), 12);
  }

  #[test]
  fn json_parser_parses_string_contain_quotes() {
    let mut parser = JSONParser::new("\"tell me \\\"more\"");
    let string = parser.parse_string().unwrap();
    assert_eq!(string.value, "tell me \\\"more");
    assert_eq!(string.length(), 14);
  }

  #[test]
  fn json_parser_parses_empty_string() {
    let mut parser = JSONParser::new("\"\"");
    let string = parser.parse_string().unwrap();
    assert_eq!(string.value, "");
    assert_eq!(string.length(), 0);
  }

  #[test]
  fn json_parser_parses_single_char_string() {
    let mut parser = JSONParser::new("\"a\"");
    let string = parser.parse_string().unwrap();
    assert_eq!(string.value, "a");
    assert_eq!(string.length(), 1);
  }
}
