use crate::PrimitiveAtomicInteger;

/// Marker trait for primitive atomic [signed integer types].
///
/// Implemented by [`AtomicI8`], [`AtomicI16`], [`AtomicI32`], [`AtomicI64`],
/// and [`AtomicIsize`].
///
/// This trait is sealed to prevent downstream implementations.
///
/// [`AtomicI8`]: core::sync::atomic::AtomicI8
/// [`AtomicI16`]: core::sync::atomic::AtomicI16
/// [`AtomicI32`]: core::sync::atomic::AtomicI32
/// [`AtomicI64`]: core::sync::atomic::AtomicI64
/// [`AtomicIsize`]: core::sync::atomic::AtomicIsize
/// [signed integer types]: core::sync::atomic
pub trait PrimitiveAtomicSigned: PrimitiveAtomicInteger {}

use core::sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize};

impl PrimitiveAtomicSigned for AtomicI8 {}
impl PrimitiveAtomicSigned for AtomicI16 {}
impl PrimitiveAtomicSigned for AtomicI32 {}
impl PrimitiveAtomicSigned for AtomicI64 {}
impl PrimitiveAtomicSigned for AtomicIsize {}
