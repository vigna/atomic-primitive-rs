use core::sync::atomic::Ordering;

use crate::Sealed;

/// Trait for all primitive [atomic types], including [`AtomicBool`] and all
/// atomic integer types.
///
/// This encapsulates trait implementations and inherent methods that are common
/// among all of the primitive atomic types: [`AtomicBool`], [`AtomicU8`],
/// [`AtomicU16`], [`AtomicU32`], [`AtomicU64`], [`AtomicUsize`], [`AtomicI8`],
/// [`AtomicI16`], [`AtomicI32`], [`AtomicI64`], and [`AtomicIsize`].
///
/// See the corresponding items on the individual types for more documentation
/// and examples.
///
/// This trait is sealed to prevent downstream implementations.
///
/// [`AtomicBool`]: core::sync::atomic::AtomicBool
/// [`AtomicU8`]: core::sync::atomic::AtomicU8
/// [`AtomicU16`]: core::sync::atomic::AtomicU16
/// [`AtomicU32`]: core::sync::atomic::AtomicU32
/// [`AtomicU64`]: core::sync::atomic::AtomicU64
/// [`AtomicUsize`]: core::sync::atomic::AtomicUsize
/// [`AtomicI8`]: core::sync::atomic::AtomicI8
/// [`AtomicI16`]: core::sync::atomic::AtomicI16
/// [`AtomicI32`]: core::sync::atomic::AtomicI32
/// [`AtomicI64`]: core::sync::atomic::AtomicI64
/// [`AtomicIsize`]: core::sync::atomic::AtomicIsize
/// [atomic types]: core::sync::atomic
#[expect(private_bounds)]
pub trait PrimitiveAtomic: Sealed + Sized + Send + Sync {
    /// The non-atomic type corresponding to this atomic type.
    type Value: Copy + Send + Sync;

    /// Creates a new atomic value.
    fn new(value: Self::Value) -> Self;

    /// Returns a mutable reference to the underlying value.
    fn get_mut(&mut self) -> &mut Self::Value;

    /// Consumes the atomic and returns the contained value.
    fn into_inner(self) -> Self::Value;

    /// Loads a value from the atomic.
    fn load(&self, order: Ordering) -> Self::Value;

    /// Stores a value into the atomic.
    fn store(&self, value: Self::Value, order: Ordering);

    /// Stores a value into the atomic, returning the previous value.
    fn swap(&self, value: Self::Value, order: Ordering) -> Self::Value;

    /// Stores a value into the atomic if the current value is the same as
    /// `current`.
    fn compare_exchange(
        &self,
        current: Self::Value,
        new: Self::Value,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::Value, Self::Value>;

    /// Stores a value into the atomic if the current value is the same as
    /// `current`. Unlike [`compare_exchange`](PrimitiveAtomic::compare_exchange),
    /// this function is allowed to spuriously fail.
    fn compare_exchange_weak(
        &self,
        current: Self::Value,
        new: Self::Value,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::Value, Self::Value>;

    /// Fetches the value, and applies a function to it that returns an optional
    /// new value. Returns a `Result` of `Ok(previous_value)` if the function
    /// returned `Some(_)`, else `Err(previous_value)`.
    fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::Value, Self::Value>
    where
        F: FnMut(Self::Value) -> Option<Self::Value>;

    /// Bitwise "and" with the current value. Returns the previous value.
    fn fetch_and(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Bitwise "nand" with the current value. Returns the previous value.
    fn fetch_nand(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Bitwise "or" with the current value. Returns the previous value.
    fn fetch_or(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Bitwise "xor" with the current value. Returns the previous value.
    fn fetch_xor(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Returns a raw pointer to the underlying value.
    fn as_ptr(&self) -> *mut Self::Value;
}

macro_rules! impl_primitive_atomic {
    ($Atomic:ty, $Value:ty) => {
        impl Sealed for $Atomic {}

        impl PrimitiveAtomic for $Atomic {
            type Value = $Value;

            #[inline]
            fn new(value: $Value) -> Self {
                <$Atomic>::new(value)
            }

            #[inline]
            fn get_mut(&mut self) -> &mut $Value {
                <$Atomic>::get_mut(self)
            }

            forward! {
                fn into_inner(self) -> $Value;
            }

            forward! {
                fn load(&self, order: Ordering) -> $Value;
                fn swap(&self, value: $Value, order: Ordering) -> $Value;
                fn as_ptr(&self) -> *mut $Value;
            }

            #[inline]
            fn store(&self, value: $Value, order: Ordering) {
                <$Atomic>::store(self, value, order)
            }

            #[inline]
            fn compare_exchange(
                &self,
                current: $Value,
                new: $Value,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$Value, $Value> {
                <$Atomic>::compare_exchange(self, current, new, success, failure)
            }

            #[inline]
            fn compare_exchange_weak(
                &self,
                current: $Value,
                new: $Value,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$Value, $Value> {
                <$Atomic>::compare_exchange_weak(self, current, new, success, failure)
            }

            #[inline]
            fn fetch_update<F>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                f: F,
            ) -> Result<$Value, $Value>
            where
                F: FnMut($Value) -> Option<$Value>,
            {
                <$Atomic>::fetch_update(self, set_order, fetch_order, f)
            }

            forward! {
                fn fetch_and(&self, val: $Value, order: Ordering) -> $Value;
                fn fetch_nand(&self, val: $Value, order: Ordering) -> $Value;
                fn fetch_or(&self, val: $Value, order: Ordering) -> $Value;
                fn fetch_xor(&self, val: $Value, order: Ordering) -> $Value;
            }
        }
    };
}

use core::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
    AtomicU32, AtomicU64, AtomicUsize,
};

impl_primitive_atomic!(AtomicBool, bool);
impl_primitive_atomic!(AtomicU8, u8);
impl_primitive_atomic!(AtomicU16, u16);
impl_primitive_atomic!(AtomicU32, u32);
impl_primitive_atomic!(AtomicU64, u64);
impl_primitive_atomic!(AtomicUsize, usize);
impl_primitive_atomic!(AtomicI8, i8);
impl_primitive_atomic!(AtomicI16, i16);
impl_primitive_atomic!(AtomicI32, i32);
impl_primitive_atomic!(AtomicI64, i64);
impl_primitive_atomic!(AtomicIsize, isize);
