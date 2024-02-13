use std::any::type_name;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
pub enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParsePersonError {
    fn from(err: ParseIntError) -> Self {
        ParsePersonError::ParseInt(err)
    }
}

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0 {
            return Err(ParsePersonError::Empty);
        }
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() % 2 != 0 {
            return Err(ParsePersonError::BadLen);
        }
        let name = parts.get(0).unwrap();
        if name.len() == 0 {
            return Err(ParsePersonError::NoName);
        }
        let age = parts.get(1).unwrap().parse();
        if age.is_err() {
            return Err(ParsePersonError::ParseInt(age.err().unwrap()))
        }
        let age= age.unwrap();
        Ok(Person{
            name: name.to_string(),
            age
        })



    }
}
