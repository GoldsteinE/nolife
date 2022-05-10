//! Internal machinery for tracking reference origins

mod sealed {
    pub trait Seal {}
}

/// Implemented for all `Brand<_>` types
// Name `brand::Is` would be kinda dumb
#[allow(clippy::module_name_repetitions)]
pub trait IsBrand: sealed::Seal + Sized {
    /// Create a new brand of the same type.
    ///
    /// # Safety
    /// Don't use this. It's only allowed to:
    /// 1. Split a reference into two references with greater level
    /// 2. Obtain a husk and a reference from an owned object
    unsafe fn duplicate(self) -> (Self, Self);
}

pub mod closure;
#[cfg(feature = "const_string_brands")]
pub mod const_string;

#[cfg(not(feature = "const_string_brands"))]
pub use closure::{brand, Brand};

#[cfg(feature = "const_string_brands")]
pub use const_string::{brand, Brand};
