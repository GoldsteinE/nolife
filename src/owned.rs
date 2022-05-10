use std::ptr::NonNull;

use crate::brand::IsBrand;

mod seal {
    pub trait Sealed {}
}

/// Only implemented for [`Heap`] for now. I'm still searching for a nice enough hack to support
/// stack ownership
pub trait OwnershipKind<T>: seal::Sealed {
    type Husk;
    type Inner;

    fn split(val: Self::Inner) -> (Self::Husk, NonNull<T>);
    /// # Safety
    /// `ptr` must be obtained from [`.split()`](OwnershipKind::split)
    /// and you must transfer full ownership to this method.
    ///
    /// No references are allowed to exist at this point and until next `.split()`.
    unsafe fn join(husk: Self::Husk, ptr: NonNull<T>) -> Self::Inner;

    fn move_out(val: Self::Inner) -> T;
}

/// Heap-allocated ownership kind
pub struct Heap;

impl seal::Sealed for Heap {}
impl<T> OwnershipKind<T> for Heap {
    type Husk = ();
    type Inner = Box<T>;

    fn split(val: Self::Inner) -> (Self::Husk, NonNull<T>) {
        // SAFETY: Box<T> is guaranteed not to be null
        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(val)) };
        ((), ptr)
    }

    unsafe fn join(_husk: Self::Husk, ptr: NonNull<T>) -> Self::Inner {
        // SAFETY: pointer was obtained from `.split()` and we are the only owner
        unsafe { Box::from_raw(ptr.as_ptr()) }
    }

    fn move_out(val: Self::Inner) -> T {
        *val
    }
}

/// Struct representing ownership and the only reference of a value
pub struct Owned<T, Kind>
where
    Kind: OwnershipKind<T>,
{
    inner: Kind::Inner,
}

/// Struct representing ownership of a value which is currently being borrowed
pub struct Husk<T, B, Kind>
where
    Kind: OwnershipKind<T>,
    B: IsBrand,
{
    inner: Kind::Husk,
    brand: B,
}

impl<T, Kind> Owned<T, Kind>
where
    Kind: OwnershipKind<T>,
{
    pub fn into_inner(self) -> T {
        Kind::move_out(self.inner)
    }

    /// Reconstruct [`Owned`] object from its `inner` pointer.
    ///
    /// # Safety
    /// `inner` must be obtained by calling [`OwnershipKind::join`] with correctly branded [`Husk`].
    /// No other references are allowed to exist at this point.
    pub unsafe fn from_inner(inner: Kind::Inner) -> Self {
        Self { inner }
    }

    /// Split [`Owned`] object into [`Husk`] and pointer which can be used to construct references.
    ///
    /// # Safety
    /// Same `brand` must be used when calling this method and when constructing references.
    pub unsafe fn split<B>(self, brand: B) -> (Husk<T, B, Kind>, NonNull<T>)
    where
        B: IsBrand,
    {
        let (inner, ptr) = Kind::split(self.inner);
        (Husk { inner, brand }, ptr)
    }
}

impl<T, B, Kind> Husk<T, B, Kind>
where
    B: IsBrand,
    Kind: OwnershipKind<T>,
{
    /// Forget brand information, leaving just unbranded husk
    pub fn into_inner(self) -> Kind::Husk {
        self.inner
    }
}

/// Create a new [`Owned`] value on the heap
#[macro_export]
macro_rules! heap {
    ($val:expr) => {
        // SAFETY: it's always safe to create a heap-allocated owned value
        unsafe { $crate::Owned::<_, $crate::Heap>::from_inner(::std::boxed::Box::new($val)) }
    };
}
