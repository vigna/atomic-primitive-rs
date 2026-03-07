# atomic-primitive

[![crate](https://img.shields.io/crates/v/atomic-primitive.svg)](https://crates.io/crates/atomic-primitive)
[![documentation](https://docs.rs/atomic-primitive/badge.svg)](https://docs.rs/atomic-primitive)
[![minimum rustc 1.85](https://img.shields.io/badge/rustc-1.85+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![build status](https://github.com/vigna/atomic-primitive-rs/workflows/master/badge.svg)](https://github.com/vigna/atomic-primitive-rs/actions)

Traits for primitive atomic types in Rust.

These traits provide a simple hierarchy for generic programming with Rust's
primitive atomic types:

- [`PrimitiveAtomic`]: [`AtomicBool`] and all atomic integer types;
  - [`PrimitiveAtomicInteger`]: all atomic integer types;
    - [`PrimitiveAtomicSigned`]: [`AtomicI8`], [`AtomicI16`], [`AtomicI32`],
      [`AtomicI64`], [`AtomicIsize`];
    - [`PrimitiveAtomicUnsigned`]: [`AtomicU8`], [`AtomicU16`], [`AtomicU32`],
      [`AtomicU64`], [`AtomicUsize`].

Additionally, [`AtomicPrimitive`] maps non-atomic primitive types to their
atomic counterparts, mirroring the unstable
[`core::sync::atomic::AtomicPrimitive`] trait.

It is not a goal of this crate to _add_ any functionality to the atomic types,
only to expose what is already available in the standard library in a more
generic way, and make a few experimental traits and types available on stable.
The only addition is an [`AtomicPrimitive`] bound on [`PrimitiveAtomic::Value`],
which enables bidirectional navigation between atomic and non-atomic types.
The traits are also [sealed] against downstream implementations to allow
expansion in a non-breaking way.

## Compatibility

This crate is currently tested for Rust 1.85 and greater.

[`AtomicBool`]: core::sync::atomic::AtomicBool
[`AtomicU8`]: core::sync::atomic::AtomicU8
[`AtomicU16`]: core::sync::atomic::AtomicU16
[`AtomicU32`]: core::sync::atomic::AtomicU32
[`AtomicU64`]: core::sync::atomic::AtomicU64
[`AtomicUsize`]: core::sync::atomic::AtomicUsize
[`AtomicI8`]: core::sync::atomic::AtomicI8
[`AtomicI16`]: core::sync::atomic::AtomicI16
[`AtomicI32`]: core::sync::atomic::AtomicI32
[`AtomicI64`]: core::sync::atomic::AtomicI64
[`AtomicIsize`]: core::sync::atomic::AtomicIsize
[sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
[`core::sync::atomic::AtomicPrimitive`]: https://doc.rust-lang.org/core/sync/atomic/trait.AtomicPrimitive.html
