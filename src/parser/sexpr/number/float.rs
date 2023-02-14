use std::{fmt::Display, str::FromStr};

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum Float {
    Float32(f32),
    Float64(f64),
    // BigFloat,
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Float::Float32(n) => write!(f, "{}", n),
            Float::Float64(n) => write!(f, "{}", n),
        }
    }
}

pub struct ParseFloatError(pub String);

impl FromStr for Float {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("f32") {
            return Ok(Float::Float32(
                s.strip_suffix("f32")
                    .expect("expected float literal suffix")
                    .parse()
                    .map_err(|_| ParseFloatError("invalid f32 literal".to_string()))?,
            ));
        } else if s.ends_with("f64") {
            return Ok(Float::Float64(
                s.strip_suffix("f64")
                    .expect("expected float literal suffix")
                    .parse()
                    .map_err(|_| ParseFloatError("invalid f64 literal".to_string()))?,
            ));
        }
        Ok(Float::Float32(s.parse::<f32>().map_err(|_| {
            ParseFloatError("invalid floating point literal".to_string())
        })?))
    }
}
