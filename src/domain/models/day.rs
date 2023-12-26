//! Day model.

/// Represents a valid day in the Advent of Code challenge.
/// The day must be a positive integer.
pub struct Day(i32);

/// Represents an error that can occur while creating a `Day`.
#[derive(Debug, PartialEq)]
pub enum DayError {
    /// The day is less than 1.
    BelowOne,
}

impl Day {
    /// Creates a new `Day` if the given day is valid.
    ///
    /// # Arguments
    /// * `day` - The day of the challenge.
    ///
    /// # Returns
    /// A `Result` which is `Ok` containing the day if successful, or an `Err` containing a `DayError::BelowOne` if the
    /// day is less than 1.
    pub fn new(day: i32) -> Result<Day, DayError> {
        if day < 1 {
            Err(DayError::BelowOne)
        } else {
            Ok(Day(day))
        }
    }

    /// Returns the inner day value.
    ///
    /// # Returns
    /// The inner day value.
    pub fn value(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_new_valid() {
        let valid_days = vec![1, 10];

        for day in valid_days {
            let day_ = Day::new(day);
            assert!(day_.is_ok());

            if let Ok(day_) = day_ {
                assert_eq!(day_.value(), day);
            }
        }
    }

    #[test]
    fn test_day_new_invalid() {
        let invalid_days = vec![0, -1, -10];

        for day in invalid_days {
            let day = Day::new(day);
            assert!(day.is_err());

            if let Err(day) = day {
                assert_eq!(day, DayError::BelowOne);
            }
        }
    }
}