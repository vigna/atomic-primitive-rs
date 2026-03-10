#![doc = include_str!("../README.md")]
#![no_std]

#[macro_use]
mod macros;

mod atomic;
mod integer;
mod signed;
mod unsigned;

#[cfg(test)]
mod tests;

pub use self::atomic::PrimitiveAtomic;
pub use self::integer::PrimitiveAtomicInteger;
pub use self::signed::PrimitiveAtomicSigned;
pub use self::unsigned::PrimitiveAtomicUnsigned;

trait Sealed {}

/// Maps a non-atomic primitive type to its corresponding atomic type.
///
/// This mirrors the unstable [`core::sync::atomic::AtomicPrimitive`] trait.
/// Once that trait is stabilized, this can be deprecated in favor of the std
/// version.
///
/// # Examples
///
/// ```
/// use atomic_primitive::{AtomicPrimitive, PrimitiveAtomic};
///
/// fn make_atomic<T: AtomicPrimitive>(val: T) -> T::Atomic {
///     T::Atomic::new(val)
/// }
/// ```
///
/// [`core::sync::atomic::AtomicPrimitive`]: https://doc.rust-lang.org/core/sync/atomic/trait.AtomicPrimitive.html
pub trait AtomicPrimitive: Sized + Copy + Send + Sync {
    /// The atomic type corresponding to this primitive type.
    ///
    /// Note that the bound on `Value` is not imposed in the unstable
    /// `core::sync::atomic::AtomicPrimitive` trait, but it is useful
    /// to round-trip between the primitive and atomic types.
    type Atomic: PrimitiveAtomic<Value = Self>;

    /// Converts this value to its atomic counterpart.
    #[inline]
    fn to_atomic(self) -> Self::Atomic {
        Self::Atomic::new(self)
    }
}

/// Type alias for the atomic version of a primitive type.
///
/// This mirrors the unstable [`core::sync::atomic::Atomic`] type alias.
///
/// [`core::sync::atomic::Atomic`]: https://doc.rust-lang.org/core/sync/atomic/type.Atomic.html
pub type Atomic<T> = <T as AtomicPrimitive>::Atomic;

macro_rules! impl_atomic_primitive {
    ($($Primitive:ty => $AtomicType:ty),+ $(,)?) => {$(
        impl AtomicPrimitive for $Primitive {
            type Atomic = $AtomicType;
        }
    )+};
}

use core::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
    AtomicU32, AtomicU64, AtomicUsize,
};

impl_atomic_primitive! {
    bool => AtomicBool,
    u8 => AtomicU8,
    u16 => AtomicU16,
    u32 => AtomicU32,
    u64 => AtomicU64,
    usize => AtomicUsize,
    i8 => AtomicI8,
    i16 => AtomicI16,
    i32 => AtomicI32,
    i64 => AtomicI64,
    isize => AtomicIsize,
}
