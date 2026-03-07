# atomic-primitive

[![crate](https://img.shields.io/crates/v/atomic-primitive.svg)](https://crates.io/crates/atomic-primitive)
[![documentation](https://docs.rs/atomic-primitive/badge.svg)](https://docs.rs/atomic-primitive)
[![minimum rustc 1.85](https://img.shields.io/badge/rustc-1.85+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![build status](https://github.com/vigna/atomic-primitive-rs/workflows/ci/badge.svg)](https://github.com/vigna/atomic-primitive-rs/actions)

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

[`PrimitiveAtomic`]: https://docs.rs/atomic-primitive/latest/atomic_primitive/trait.PrimitiveAtomic.html
[`PrimitiveAtomic::Value`]: https://docs.rs/atomic-primitive/latest/atomic_primitive/trait.PrimitiveAtomic.html#associatedtype.Value
[`PrimitiveAtomicInteger`]: https://docs.rs/atomic-primitive/latest/atomic_primitive/trait.PrimitiveAtomicInteger.html
[`PrimitiveAtomicSigned`]: https://docs.rs/atomic-primitive/latest/atomic_primitive/trait.PrimitiveAtomicSigned.html
[`PrimitiveAtomicUnsigned`]: https://docs.rs/atomic-primitive/latest/atomic_primitive/trait.PrimitiveAtomicUnsigned.html
[`AtomicPrimitive`]: https://docs.rs/atomic-primitive/latest/atomic_primitive/trait.AtomicPrimitive.html
[`AtomicBool`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html
[`AtomicU8`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU8.html
[`AtomicU16`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU16.html
[`AtomicU32`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU32.html
[`AtomicU64`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU64.html
[`AtomicUsize`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html
[`AtomicI8`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI8.html
[`AtomicI16`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI16.html
[`AtomicI32`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI32.html
[`AtomicI64`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI64.html
[`AtomicIsize`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicIsize.html
[sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
[`core::sync::atomic::AtomicPrimitive`]: https://doc.rust-lang.org/core/sync/atomic/trait.AtomicPrimitive.html
