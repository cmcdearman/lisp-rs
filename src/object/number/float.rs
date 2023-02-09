use std::fmt::Display;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum Float {
    Float32(f32),
    Float64(f64),
    BigFloat,
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}