//! Part model.

/// Represents a valid part in the Advent of Code challenge.
/// The part must be either 1 or 2.
pub struct Part(i32);

/// Represents an error that can occur while creating a `Part`.
#[derive(Debug, PartialEq)]
pub enum PartError {
    /// The part is less than 1.
    BelowOne,
    /// The part is greater than 2.
    AboveTwo,
}

impl Part {
    /// Creates a new `Part` if the given part is valid. The part must be either 1 or 2.
    ///
    /// # Arguments
    /// * `part` - The part of the challenge.
    ///
    /// # Returns
    /// A `Result` which is `Ok` containing the part if successful, or an `Err` containing a `PartError::BelowOne` if the
    /// part is less than 1, or a `PartError::AboveTwo` if the part is greater than 2.
    pub fn new(part: i32) -> Result<Part, PartError> {
        if part < 1 {
            Err(PartError::BelowOne)
        } else if part > 2 {
            Err(PartError::AboveTwo)
        } else {
            Ok(Part(part))
        }
    }

    /// Returns the inner part value.
    ///
    /// # Returns
    /// The inner part value.
    pub fn value(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_new_valid() {
        let valid_parts = vec![1, 2];

        for part in valid_parts {
            let part_ = Part::new(part);
            assert!(part_.is_ok());

            if let Ok(part_) = part_ {
                assert_eq!(part_.value(), part);
            }
        }
    }

    #[test]
    fn test_part_new_invalid() {
        let invalid_parts_below_one = (vec![0, -1, -10], PartError::BelowOne);
        let invalid_parts_above_two = (vec![3, 4, 10], PartError::AboveTwo);

        for (invalid_parts, part_error) in vec![invalid_parts_below_one, invalid_parts_above_two] {
            for part in invalid_parts {
                let part = Part::new(part);
                assert!(part.is_err());

                if let Err(part_err) = part {
                    assert_eq!(part_err, part_error);
                }
            }
        }
    }
}