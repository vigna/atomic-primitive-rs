use crate::PrimitiveAtomicInteger;

/// Marker trait for primitive atomic [unsigned integer types].
///
/// Implemented by [`AtomicU8`], [`AtomicU16`], [`AtomicU32`], [`AtomicU64`],
/// and [`AtomicUsize`].
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

use core::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize};

impl PrimitiveAtomicUnsigned for AtomicU8 {}
impl PrimitiveAtomicUnsigned for AtomicU16 {}
impl PrimitiveAtomicUnsigned for AtomicU32 {}
impl PrimitiveAtomicUnsigned for AtomicU64 {}
impl PrimitiveAtomicUnsigned for AtomicUsize {}
