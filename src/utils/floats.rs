#[cfg(feature = "float64")]
mod float_types {
    pub type Float = f64;
    pub type Positive = typed_floats::tf64::Positive;
    pub type PositiveFinite = typed_floats::tf64::PositiveFinite;
}

#[cfg(not(feature = "float64"))]
mod float_types {
    pub type Float = f32;
    pub type Positive = typed_floats::tf32::Positive;
    pub type PositiveFinite = typed_floats::tf32::PositiveFinite;
}

pub use float_types::*;
