//! Branding based on the uniqueness of closure types
//!
//! Simple, doesn't require additional nightly features, but produces bad error messages

use std::marker::PhantomData;

/// An unique type generated by the [`brand!`] macro.
/// Guaranteed to be zero-sized.
///
/// This wouldn't compile:
/// ```compile_fail
/// # use nolife::brand::closure::*;
/// fn assert_same_type<T>(_: T, _: T) {}
/// assert_same_type(brand!(), brand!())
/// ```
#[repr(transparent)]
pub struct Brand<F>(PhantomData<F>);

impl<F> Brand<F> {
    /// An implementation detail used by the [`brand!`] macro. Don't use it unless you new exactly
    /// what you're doing.
    ///
    /// # Safety
    /// This can be used to create a duplicate brand, which has same safety implications as
    /// [`IsBrand::duplicate`](super::IsBrand::duplicate)
    #[must_use]
    pub unsafe fn new(_: F) -> Self {
        Self(PhantomData)
    }
}

impl<F> super::sealed::Seal for Brand<F> {}
impl<F> super::IsBrand for Brand<F> {
    unsafe fn duplicate(self) -> (Self, Self) {
        (self, Self(PhantomData))
    }
}

/// Generate a new unique brand. This is safe since macro will yield a new brand every time
#[macro_export]
// Unfortunately, macros are not scoped properly
#[allow(clippy::module_name_repetitions)]
macro_rules! _closure_brand {
    () => {
        unsafe { $crate::brand::closure::Brand::new(|| ()) }
    };
}

pub use crate::_closure_brand as brand;
