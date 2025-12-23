use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    /// Here is the noob way of doing it
    ///
    /// ```
    /// if value < 0 {
    ///     return Err(CreationError::Negative);
    /// } else if value == 0 {
    ///     return Err(CreationError::Zero);
    /// }
    /// Ok(Self(value as u64))
    /// ```
    ///
    /// Having done this I then lifted the below from the solutions and I prefer
    /// it.
    fn new(value: i64) -> Result<Self, CreationError> {
        match value.cmp(&0) {
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Equal => Err(CreationError::Zero),
            _ => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
