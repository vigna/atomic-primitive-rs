use crate::{
    Atomic, AtomicPrimitive, PrimitiveAtomic, PrimitiveAtomicInteger, PrimitiveAtomicSigned,
    PrimitiveAtomicUnsigned,
};
use core::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32,
    AtomicU64, AtomicUsize, Ordering,
};

#[test]
fn test_atomic_primitive_mapping() {
    let a: Atomic<u8> = 42u8.to_atomic();
    assert_eq!(a.load(Ordering::Relaxed), 42);

    let a: Atomic<u64> = 123u64.to_atomic();
    assert_eq!(a.load(Ordering::Relaxed), 123);

    let a: Atomic<bool> = true.to_atomic();
    assert_eq!(a.load(Ordering::Relaxed), true);

    let a: Atomic<usize> = 999usize.to_atomic();
    assert_eq!(a.load(Ordering::Relaxed), 999);
}

#[test]
fn test_primitive_atomic_basic() {
    let a = AtomicU64::new(10);
    assert_eq!(PrimitiveAtomic::load(&a, Ordering::Relaxed), 10);
    PrimitiveAtomic::store(&a, 20, Ordering::Relaxed);
    assert_eq!(PrimitiveAtomic::load(&a, Ordering::Relaxed), 20);
    assert_eq!(PrimitiveAtomic::swap(&a, 30, Ordering::Relaxed), 20);
    assert_eq!(PrimitiveAtomic::into_inner(a), 30);
}

#[test]
fn test_primitive_atomic_cas() {
    let a = AtomicU32::new(5);
    assert_eq!(
        PrimitiveAtomic::compare_exchange(&a, 5, 10, Ordering::SeqCst, Ordering::SeqCst),
        Ok(5)
    );
    assert_eq!(
        PrimitiveAtomic::compare_exchange(&a, 5, 20, Ordering::SeqCst, Ordering::SeqCst),
        Err(10)
    );
}

#[test]
fn test_primitive_atomic_integer() {
    let a = AtomicU64::new(10);
    assert_eq!(
        PrimitiveAtomicInteger::fetch_add(&a, 5, Ordering::Relaxed),
        10
    );
    assert_eq!(PrimitiveAtomic::load(&a, Ordering::Relaxed), 15);
    assert_eq!(
        PrimitiveAtomicInteger::fetch_sub(&a, 3, Ordering::Relaxed),
        15
    );
    assert_eq!(PrimitiveAtomic::load(&a, Ordering::Relaxed), 12);
}

#[test]
fn test_bits_constant() {
    assert_eq!(<AtomicU8 as PrimitiveAtomicInteger>::BITS, 8);
    assert_eq!(<AtomicU16 as PrimitiveAtomicInteger>::BITS, 16);
    assert_eq!(<AtomicU32 as PrimitiveAtomicInteger>::BITS, 32);
    assert_eq!(<AtomicU64 as PrimitiveAtomicInteger>::BITS, 64);
    assert_eq!(<AtomicUsize as PrimitiveAtomicInteger>::BITS, usize::BITS);
}

#[test]
fn test_fetch_bitwise() {
    let a = AtomicU8::new(0b1100);
    assert_eq!(
        PrimitiveAtomic::fetch_or(&a, 0b0011, Ordering::Relaxed),
        0b1100
    );
    assert_eq!(PrimitiveAtomic::load(&a, Ordering::Relaxed), 0b1111);
    assert_eq!(
        PrimitiveAtomic::fetch_and(&a, 0b1010, Ordering::Relaxed),
        0b1111
    );
    assert_eq!(PrimitiveAtomic::load(&a, Ordering::Relaxed), 0b1010);
    assert_eq!(
        PrimitiveAtomic::fetch_xor(&a, 0b1111, Ordering::Relaxed),
        0b1010
    );
    assert_eq!(PrimitiveAtomic::load(&a, Ordering::Relaxed), 0b0101);
}

#[test]
fn test_get_mut() {
    let mut a = AtomicU64::new(42);
    *PrimitiveAtomic::get_mut(&mut a) = 100;
    assert_eq!(PrimitiveAtomic::load(&a, Ordering::Relaxed), 100);
}

#[test]
fn test_fetch_update() {
    let a = AtomicU32::new(10);
    let result = PrimitiveAtomic::fetch_update(&a, Ordering::SeqCst, Ordering::SeqCst, |x| {
        if x < 20 { Some(x + 5) } else { None }
    });
    assert_eq!(result, Ok(10));
    assert_eq!(PrimitiveAtomic::load(&a, Ordering::Relaxed), 15);
}

fn _assert_primitive_atomic<T: PrimitiveAtomic>() {}
fn _assert_primitive_atomic_integer<T: PrimitiveAtomicInteger>() {}
fn _assert_primitive_atomic_signed<T: PrimitiveAtomicSigned>() {}
fn _assert_primitive_atomic_unsigned<T: PrimitiveAtomicUnsigned>() {}
fn _assert_atomic_primitive<T: AtomicPrimitive>() {}

#[test]
fn test_trait_bounds() {
    _assert_primitive_atomic::<AtomicBool>();
    _assert_primitive_atomic::<AtomicU8>();
    _assert_primitive_atomic::<AtomicI64>();

    _assert_primitive_atomic_integer::<AtomicU8>();
    _assert_primitive_atomic_integer::<AtomicI32>();
    _assert_primitive_atomic_integer::<AtomicUsize>();

    _assert_primitive_atomic_signed::<AtomicI8>();
    _assert_primitive_atomic_signed::<AtomicIsize>();

    _assert_primitive_atomic_unsigned::<AtomicU8>();
    _assert_primitive_atomic_unsigned::<AtomicUsize>();

    _assert_atomic_primitive::<bool>();
    _assert_atomic_primitive::<u64>();
    _assert_atomic_primitive::<usize>();
}
