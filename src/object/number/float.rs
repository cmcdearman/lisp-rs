#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Float {
    Float32(f32),
    Float64(f64),
    BigFloat,
}