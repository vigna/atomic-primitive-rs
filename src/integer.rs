use core::sync::atomic::Ordering;

use crate::PrimitiveAtomic;

/// Trait for all primitive atomic [integer types], extending [`PrimitiveAtomic`].
///
/// This adds integer-specific atomic operations: [`fetch_add`], [`fetch_sub`],
/// [`fetch_max`], [`fetch_min`], and a [`BITS`] constant.
///
/// This trait is sealed to prevent downstream implementations.
///
/// [`fetch_add`]: PrimitiveAtomicInteger::fetch_add
/// [`fetch_sub`]: PrimitiveAtomicInteger::fetch_sub
/// [`fetch_max`]: PrimitiveAtomicInteger::fetch_max
/// [`fetch_min`]: PrimitiveAtomicInteger::fetch_min
/// [`BITS`]: PrimitiveAtomicInteger::BITS
/// [integer types]: core::sync::atomic
pub trait PrimitiveAtomicInteger: PrimitiveAtomic {
    /// The size of this atomic integer type in bits.
    const BITS: u32;

    /// Adds to the current value, returning the previous value.
    fn fetch_add(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Subtracts from the current value, returning the previous value.
    fn fetch_sub(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Stores the maximum of the current value and `val`. Returns the previous value.
    fn fetch_max(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Stores the minimum of the current value and `val`. Returns the previous value.
    fn fetch_min(&self, val: Self::Value, order: Ordering) -> Self::Value;
}

macro_rules! impl_primitive_atomic_integer {
    ($Atomic:ty, $bits:expr) => {
        impl PrimitiveAtomicInteger for $Atomic {
            const BITS: u32 = $bits;

            forward! {
                fn fetch_add(&self, val: Self::Value, order: Ordering) -> Self::Value;
                fn fetch_sub(&self, val: Self::Value, order: Ordering) -> Self::Value;
                fn fetch_max(&self, val: Self::Value, order: Ordering) -> Self::Value;
                fn fetch_min(&self, val: Self::Value, order: Ordering) -> Self::Value;
            }
        }
    };
}

use core::sync::atomic::{
    AtomicI8, AtomicI16, AtomicI32, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicUsize,
};
#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{AtomicI64, AtomicU64};

impl_primitive_atomic_integer!(AtomicU8, 8);
impl_primitive_atomic_integer!(AtomicU16, 16);
impl_primitive_atomic_integer!(AtomicU32, 32);
impl_primitive_atomic_integer!(AtomicUsize, usize::BITS);
impl_primitive_atomic_integer!(AtomicI8, 8);
impl_primitive_atomic_integer!(AtomicI16, 16);
impl_primitive_atomic_integer!(AtomicI32, 32);
impl_primitive_atomic_integer!(AtomicIsize, isize::BITS);
#[cfg(target_has_atomic = "64")]
impl_primitive_atomic_integer!(AtomicU64, 64);
#[cfg(target_has_atomic = "64")]
impl_primitive_atomic_integer!(AtomicI64, 64);
