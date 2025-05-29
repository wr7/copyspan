use core::{hash::Hash, ops::Add};

#[allow(unused)]
pub trait Dummy {}
impl<T> Dummy for T {}

#[cfg(feature = "alloc")]
use alloc::fmt::Debug as AllocRequirements;

#[cfg(not(feature = "alloc"))]
use Dummy as AllocRequirements;

#[doc(hidden)]
pub trait RustNumber:
    Default
    + Clone
    + Copy
    + Add<Self, Output = Self>
    + PartialOrd
    + Ord
    + PartialEq
    + Eq
    + AllocRequirements
    + Hash
{
}

macro_rules! impl_rustnumber {
    ($ty:ty) => {
        impl RustNumber for $ty {}
    };
}

impl_rustnumber!(u8);
impl_rustnumber!(u16);
impl_rustnumber!(u32);
impl_rustnumber!(u64);
impl_rustnumber!(usize);
impl_rustnumber!(i8);
impl_rustnumber!(i16);
impl_rustnumber!(i32);
impl_rustnumber!(i64);
impl_rustnumber!(isize);
