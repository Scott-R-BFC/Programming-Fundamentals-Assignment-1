use std::error::Error;
const RATING_RANGE: std::ops::RangeInclusive<f32> = 0.0..=10.0;

/// Rating 'inherits' these 'traits from f32
/// - Debug: defines printing with println!("{:?}", rating_var);
/// - Clone: defines creation of a copy with rating_var.clone();
/// - Copy: gives copy semantics instead of move semantics
/// - PartialEq and PartialOrd: define equality and comparison operators
///
/// Rating is a subset of f32 limited to the range [RATING_RANGE]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rating(f32);

/// Specifies that the comparison operator is
/// - **reflexive**: a == a is true
/// - **symmetric**: a == b => b == a
/// - **transitive**: a == b && b == c => a == c
impl Eq for Rating {}

// Defines format string functionality as f32 has
impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Rating methods
impl Rating {
    /// Returns a `Result` which contains a `Rating` or an `Error` based on if the provided `f32`
    /// is within the range of the rating
    pub fn from(value: f32) -> Result<Rating, Box<dyn Error>> {
        if !RATING_RANGE.contains(&value) {
            return Err("Cannot create new Rating with invalid value. Value must be in range 0.0 ..= 10.0".into());
        }

        Ok(Rating(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_from_below_range() {
        assert!(Rating::from(RATING_RANGE.start() - 1.0).is_err());
    }

    #[test]
    fn rating_from_above_range() {
        assert!(Rating::from(RATING_RANGE.end() + 1.0).is_err());
    }

    #[test]
    fn rating_from_lower_bound() {
        let _ = Rating::from(*RATING_RANGE.start());
    }

    #[test]
    fn rating_from_upper_bound() {
        let _ = Rating::from(*RATING_RANGE.end());
    }

    fn rating_from_within_range() {
        let _ = Rating::from((RATING_RANGE.start() + RATING_RANGE.end()) / 2.0);
    }
}
