use core::marker::ConstParamTy_;
use core::marker::Freeze;

/// # Safety
///
/// You may only implement this trait for a type if it meets all of the following conditions:
/// 1. This type must not be a [`union`](https://doc.rust-lang.org/std/keyword.union.html), [`pointer`](https://doc.rust-lang.org/std/primitive.pointer.html), [`reference`](https://doc.rust-lang.org/std/primitive.reference.html), [`UnsafeCell`](core::cell::UnsafeCell), or [`Atomic`](https://doc.rust-lang.org/std/sync/atomic/struct.Atomic.html)
/// 2. Condition #1 must hold recursively for this type's fields, variants, or elements (if it has any).
/// 3. This type's [`PartialEq`] implementation must be equivalent to comparing all non-padding bytes of the data bytewise.
///
/// If the type fails to meet any one of these, implementing this trait is considered to be UB.
///
/// # How to implement this trait
///
/// The recommended way to justify the safety of an implementation of this trait is to ensure the following:
///
/// 1. All of the type's fields or variants (if it has any) implement [`SureEq`].
/// 2. [`PartialEq`] and [`Eq`] are derived using the `std` derive macros, namely `#[derive(PartialEq, Eq)]`.
///
/// (FIXME: The above doesn't account for the fact that this wouldn't make the `Eq` implementation `const`.)
///
/// If these conditions are fulfilled, this is sufficient to argue that the three required preconditions are satisfied.
pub unsafe trait SureEq: const Eq + Freeze + ConstParamTy_ + 'static {}

// SAFETY:
// For the primitives below, these are the reasons why each required condition is satisfied.
// #1 Because this is a primitive type that is not a union, pointer, reference, UnsafeCell, or Atomic.
// #2 Because this is a primitive type without any fields or variants.
// #3 Because we trust `std`'s `PartialEq` impl to be equivalent to this requirement.

// SAFETY: see above
unsafe impl SureEq for () {}
// SAFETY: see above
unsafe impl SureEq for bool {}
// SAFETY: see above
unsafe impl SureEq for char {}
// SAFETY: see above
unsafe impl SureEq for i8 {}
// SAFETY: see above
unsafe impl SureEq for i16 {}
// SAFETY: see above
unsafe impl SureEq for i32 {}
// SAFETY: see above
unsafe impl SureEq for i64 {}
// SAFETY: see above
unsafe impl SureEq for i128 {}
// SAFETY: see above
unsafe impl SureEq for isize {}
// SAFETY: see above
unsafe impl SureEq for u8 {}
// SAFETY: see above
unsafe impl SureEq for u16 {}
// SAFETY: see above
unsafe impl SureEq for u32 {}
// SAFETY: see above
unsafe impl SureEq for u64 {}
// SAFETY: see above
unsafe impl SureEq for u128 {}
// SAFETY: see above
unsafe impl SureEq for usize {}

// SAFETY:
// #1 Because this is an array, not a union, pointer, reference, UnsafeCell, or Atomic.
// #2 Because the elements of this array implement `SureEq`.
// #3 Because when the elements implement `SureEq`, we trust `std`'s blanket `PartialEq` impl to be equivalent to this requirement.
unsafe impl<T: SureEq, const N: usize> SureEq for [T; N] {}

macro_rules! unsafe_impl_sure_eq_for_tuples {
    ($(($($T:ident),+)),+,) => {
        // SAFETY:
        // #1 Because this is a tuple, not a union, pointer, reference, UnsafeCell, or Atomic.
        // #2 Because the fields of this tuple implement `SureEq`.
        // #3 Because when the fields implement `SureEq`, we trust `std`'s blanket `PartialEq` impl to be equivalent to this requirement.
        $(unsafe impl<$($T: SureEq),+> SureEq for ($($T),+,) {})+
    };
}

// implementations for tuples up to length 12, because that is as far as `core` goes for `ConstParamTy_`
unsafe_impl_sure_eq_for_tuples! {
    (A),
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H),
    (A, B, C, D, E, F, G, H, I),
    (A, B, C, D, E, F, G, H, I, J),
    (A, B, C, D, E, F, G, H, I, J, K),
    (A, B, C, D, E, F, G, H, I, J, K, L),
}
