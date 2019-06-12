use std::convert::{From, TryFrom};

pub mod constants;

#[derive(Debug, Clone)]
pub struct NumberToken {
  pub value: String,
  pub start: usize,
  pub end: usize,
}

impl NumberToken {
  pub fn length(&self) -> usize {
    self.end - self.start + 1
  }
}

impl TryFrom<Token> for NumberToken {
  type Error = &'static str;

  fn try_from(t: Token) -> Result<Self, Self::Error> {
    match t {
      Token::Number(token_value) => Ok(token_value),
      _ => Err("Cannot convert Token into NumberToken")
    }
  }
}

#[derive(Debug, Clone)]
pub struct StringToken {
  pub value: String,
  pub start: usize,
  pub end: usize,
}

impl StringToken {
  pub fn length(&self) -> usize {
    self.value.len()
  }
}

impl TryFrom<Token> for StringToken {
  type Error = &'static str;

  fn try_from(t: Token) -> Result<Self, Self::Error> {
    match t {
      Token::String(token_value) => Ok(token_value),
      _ => Err("Cannot convert Token into StringToken")
    }
  }
}

#[derive(Debug, Clone)]
pub struct ObjectTokenPair {
  pub key: String,
  pub value: Token,
}

#[derive(Debug, Clone)]
pub struct ObjectToken {
  pub values: Vec<ObjectTokenPair>,
  pub start: usize,
  pub end: usize,
}

impl TryFrom<Token> for ObjectToken {
  type Error = &'static str;

  fn try_from(t: Token) -> Result<Self, Self::Error> {
    match t {
      Token::Object(token_value) => Ok(token_value),
      _ => Err("Cannot convert Token into ObjectToken")
    }
  }
}

#[derive(Debug, Clone)]
pub struct ArrayToken {
  pub values: Vec<Token>,
  pub start: usize,
  pub end: usize,
}

impl TryFrom<Token> for ArrayToken {
  type Error = &'static str;

  fn try_from(t: Token) -> Result<Self, Self::Error> {
    match t {
      Token::Array(token_value) => Ok(token_value),
      _ => Err("Cannot convert Token into ArrayToken")
    }
  }
}


#[derive(Debug, Clone)]
pub enum Token {
  Number(NumberToken),
  String(StringToken),
  Object(ObjectToken),
  Array(ArrayToken),
}

// impl Token {
//   pub fn to_number() -> Option<NumberToken>
// }

impl From<NumberToken> for Token {
  fn from(token: NumberToken) -> Token {
    Token::Number(token)
  }
}

impl From<StringToken> for Token {
  fn from(token: StringToken) -> Token {
    Token::String(token)
  }
}

impl From<ObjectToken> for Token {
  fn from(token: ObjectToken) -> Token {
    Token::Object(token)
  }
}

impl From<ArrayToken> for Token {
  fn from(token: ArrayToken) -> Token {
    Token::Array(token)
  }
}
