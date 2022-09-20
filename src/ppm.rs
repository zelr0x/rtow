use std::fmt;

use crate::color::TranslatedColor;

#[derive(Debug, Clone, PartialEq)]
pub struct PpmColor<'a>(pub &'a TranslatedColor);

impl fmt::Display for PpmColor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.0.r, self.0.g, self.0.b
        )
    }
}
