// Rating 'inherits' these 'traits from f32
// Debug defines printing with println!("{:?}", rating_var);
// Clone defines creation of a copy with rating_var.clone();
// Copy gives copy semantics instead of move semantics
// PartialEq and PartialOrd define equality and comparison operators
/// f32 limited between 0.0 and 10.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rating(f32);

// Specifies that the comparison operator is
// reflexive: a == a is true
// symmetric: a == b => b == a
// transitive: a == b && b == c => a == c
impl Eq for Rating {}

// Defines format string functionality as f32 has
impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Rating methods
impl Rating {
    pub fn from(value: f32) -> Result<Rating, Box<dyn std::error::Error>> {
        if value < 0.0 || value > 10.0 {
            return Err("Cannot create new Rating with invalid value. Value must be in range 0.0 ..= 10.0".into());
        }

        Ok(Rating(value))
    }
}
