use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{
    brand::IsBrand,
    owned::{Husk, OwnershipKind},
    Owned,
};

/// Borrowed reference. References of `LEVEL` 0 are mutable.
pub struct Ref<T, B, const LEVEL: usize>
where
    B: IsBrand,
{
    ptr: NonNull<T>,
    brand: B,
}

/// Mutable borrowed reference
pub type RefMut<T, B> = Ref<T, B, 0>;

impl<T, B, const LEVEL: usize> Ref<T, B, LEVEL>
where
    B: IsBrand,
{
    /// Create a new `Ref` with given `ptr` and `brand`. This is extremely unsafe and probably will
    /// backfire if used outside of this crate
    ///
    /// # Safety
    /// 1. `ptr` must be owned by an [`Owned`](crate::Owned) value and obtained by calling
    ///    [`Owned::split`](crate::Owned::split) with the same `brand`
    /// 2. There could be only one `Ref` of level 0. Any further `Ref`s are obtained by splitting
    ///    or joining other `Ref`s. `Ref` obtained by splitting must have greater level. `Ref`
    ///    obtained by joining may have its level decreased by one.
    pub unsafe fn new(ptr: NonNull<T>, brand: B) -> Self {
        Self { ptr, brand }
    }

    /// Split this reference into two immutable references with incremented LEVEL
    // Note: this doesn't use tuple since it seems to make typechecker unreasonably angry
    pub fn split(self) -> [Ref<T, B, { LEVEL + 1 }>; 2] {
        // SAFETY: we're using `.duplicate()` to split a reference
        let (brand1, brand2) = unsafe { self.brand.duplicate() };
        // SAFETY: if this `Ref` was created safely, calling `::new()` with the same parameters is
        // safe, since we're splitting `Ref` while increasing level
        unsafe { [Ref::new(self.ptr, brand1), Ref::new(self.ptr, brand2)] }
    }

    /// Join this reference with other reference of same level, decrementing level
    pub fn join(self, _: Self) -> Ref<T, B, { LEVEL - 1 }> {
        // SAFETY: if these `Ref`s were created safely, calling `::new()` with the same parameters is
        // safe, since we're joining two `Ref`s of the same type while decreasing level by one
        unsafe { Ref::new(self.ptr, self.brand) }
    }
}

impl<T, B> RefMut<T, B>
where
    B: IsBrand,
{
    /// Join this reference with [`Husk`], reconstructing the owned value
    pub fn reconstruct<Kind>(self, husk: Husk<T, B, Kind>) -> Owned<T, Kind>
    where
        Kind: OwnershipKind<T>,
    {
        // We destroyed the last reference...
        let ptr = self.ptr;
        // SAFETY: ...so we're now allowed to reconstruct the owned value
        unsafe { Owned::from_inner(Kind::join(husk.into_inner(), ptr)) }
    }
}

impl<T, B, const LEVEL: usize> Deref for Ref<T, B, LEVEL>
where
    B: IsBrand,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: only references pointing to `.ptr` currently are non-zero-LEVEL `Ref`s which do
        // not allow obtaining mutable references (or we are the only zero-LEVEL `Ref` which is
        // also OK)
        unsafe { self.ptr.as_ref() }
    }
}

impl<T, B> DerefMut for RefMut<T, B>
where
    B: IsBrand,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: we're the only reference pointing to `.ptr`
        unsafe { self.ptr.as_mut() }
    }
}

/// Split an [`Owned`] value into [`Husk`] and [`Ref`]
#[macro_export]
macro_rules! borrow {
    ($owned:expr) => {{
        let brand = $crate::brand::brand!();
        // SAFETY: we're using `.duplicate()` to obtain husk and ref from the owned object
        let (husk_brand, ref_brand) = unsafe { $crate::brand::IsBrand::duplicate(brand) };
        // SAFETY: we will use the same brand to construct reference
        let (husk, ptr) = unsafe { $crate::Owned::split($owned, husk_brand) };
        // SAFETY: `ptr` is owned by a provided `Owned` value and is obtained by calling
        // `Owned::split` with the same `brand`
        let reference = unsafe { $crate::Ref::<_, _, 0>::new(ptr, ref_brand) };
        (husk, reference)
    }};
}
