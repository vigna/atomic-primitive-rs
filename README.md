# atomic-primitive

Traits for primitive atomic types in Rust.

These traits provide a simple hierarchy for generic programming with Rust's
primitive atomic types:

* [`PrimitiveAtomic`]: [`AtomicBool`] and all atomic integer types;
  * [`PrimitiveAtomicInteger`]: all atomic integer types;
    * [`PrimitiveAtomicSigned`]: [`AtomicI8`], [`AtomicI16`], [`AtomicI32`],
      [`AtomicI64`], [`AtomicIsize`];
    * [`PrimitiveAtomicUnsigned`]: [`AtomicU8`], [`AtomicU16`], [`AtomicU32`],
      [`AtomicU64`], [`AtomicUsize`].

Additionally, [`AtomicPrimitive`] maps non-atomic primitive types to their
atomic counterparts, mirroring the unstable
[`std::sync::atomic::AtomicPrimitive`] trait.

It is not a goal of this crate to *add* any functionality to the atomic
types, only to expose what is already available in the standard library in a
more generic way. The traits are also [sealed] against downstream
implementations to allow expansion in a non-breaking way.

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
[`std::sync::atomic::AtomicPrimitive`]: https://doc.rust-lang.org/std/sync/atomic/trait.AtomicPrimitive.html
