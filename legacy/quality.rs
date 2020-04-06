//! Quality setting implementation.

use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Image quality enumeration.
#[json]
#[derive(Clone)]
pub enum Quality {
    /// Super-low quality.
    Potato,
    /// Low quality.
    Low,
    /// Medium quality.
    Medium,
    /// High quality.
    High,
    /// Super quality.
    Super,
}

impl Display for Quality {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Potato => "Potato",
                Self::Low => "Low",
                Self::Medium => "Medium",
                Self::High => "High",
                Self::Super => "Super",
            }
        )
    }
}
