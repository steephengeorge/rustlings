use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
pub enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    pub fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value.cmp(&0_i64) {
            Ordering::Greater => Ok(PositiveNonzeroInteger(value as u64)),
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Less => Err(CreationError::Negative)
        }
    }
}
