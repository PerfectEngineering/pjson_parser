/// Seeing as there would be so much work to implement the deserializer logic
/// I'll suspend this for now and work on it later (or never)
/// But I've learn't what I want to learn with respect to rust, its attributs and macros system.

extern crate queues;
use queues::*;

use crate::values::Token;

trait CanDeserialize {
  
}

trait Deserializer {
  
  fn deserialize(token: &Token, deserialize: &mut CanDeserialize);
}

struct JSONDeserializer

impl Deserializer for JSONDeserializer {
  fn deserialize(token: &Token, deserialize: &mut CanDeserialize) {

  }
}

/// For simplicity: 
/// - maps would deserialized in structs
/// - arrays would be deserialized into vectors
/// - strings into strings
/// - numbers into string if field type is a string or else parsed into float or int type

// Example to Structure
#[derive(CanDeserialize)]
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
  eth_balance: String // in base unit, so big number
}

struct PersonCanDeserialize;

impl From<PersonCanDeserialize> for Person {
  pub from(d: PersonCanDeserialize) -> Person {
    Person {
      first_name: 
    }
  }
}
