use crate::PrimitiveAtomicInteger;

/// Marker trait for primitive atomic [unsigned integer types].
///
/// Implemented by [`AtomicU8`], [`AtomicU16`], [`AtomicU32`],
/// [`AtomicUsize`], and, on targets with 64-bit atomics, [`AtomicU64`].
///
/// This trait is sealed to prevent downstream implementations.
///
/// [`AtomicU8`]: core::sync::atomic::AtomicU8
/// [`AtomicU16`]: core::sync::atomic::AtomicU16
/// [`AtomicU32`]: core::sync::atomic::AtomicU32
/// [`AtomicU64`]: core::sync::atomic::AtomicU64
/// [`AtomicUsize`]: core::sync::atomic::AtomicUsize
/// [unsigned integer types]: core::sync::atomic
pub trait PrimitiveAtomicUnsigned: PrimitiveAtomicInteger {}

#[cfg(target_has_atomic = "64")]
use core::sync::atomic::AtomicU64;
use core::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicUsize};

impl PrimitiveAtomicUnsigned for AtomicU8 {}
impl PrimitiveAtomicUnsigned for AtomicU16 {}
impl PrimitiveAtomicUnsigned for AtomicU32 {}
#[cfg(target_has_atomic = "64")]
impl PrimitiveAtomicUnsigned for AtomicU64 {}
impl PrimitiveAtomicUnsigned for AtomicUsize {}
