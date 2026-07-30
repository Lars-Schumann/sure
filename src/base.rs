use core::marker::Destruct;
use core::marker::Freeze;

use crate::const_helpers as ch;
use crate::set;
use crate::sure_eq::SureEq;

/// A wrapper type with the invariant:\
/// The contained value will always be contained in `SET`.
///
/// This invariant may be relied upon for the purposes of `unsafe` code and any safe mechanism to break this invariant are considered to be UB.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Sure<T: SureEq + 'static, const SET: &'static [T]>(T);

impl<T, const SET: &'static [T]> Sure<T, SET>
where
    T: Copy + const Destruct + Freeze + SureEq + const Ord + 'static,
{
    /// The exhaustive set of values that could be stored in this type.
    pub const SET: &'static [T] = SET;

    /// Returns the SET of this type.
    ///
    /// This is the same as `Self::SET`, but can be used where the type of a variable is not easily available.
    #[must_use]
    pub const fn set(&self) -> &'static [T] {
        SET
    }

    /// Creates a `Sure` if the given value is contained in `SET`.
    ///
    /// # Errors
    ///
    /// Will return `Err(value)` if the given value is not contained in `SET`.
    pub const fn new(value: T) -> Result<Self, T> {
        match Self::set_contains(&value) {
            true => Ok(
                // SAFETY: we just checked precondition #1: `Self::set_contains(value)`
                unsafe { Self::new_unchecked(value) },
            ),
            false => Err(value),
        }
    }

    /// Creates a `Sure` if the given value is found in `SET` using the [`Self::set_contains_via_binary_search`] method.
    ///
    /// # Errors
    ///
    /// Will return `Err(value)` if the given value is not found in `SET` as explained above.
    pub const fn new_via_binary_search(value: T) -> Result<Self, T> {
        match Self::set_contains_via_binary_search(&value) {
            true => Ok(
                // SAFETY: we just checked precondition #1: `Self::set_contains(value)`, since the binary search is strictly more conservative.
                unsafe { Self::new_unchecked(value) },
            ),
            false => Err(value),
        }
    }

    /// Creates a `Sure` without checking whether the value is contained in `SET`. This results in undefined behavior if the value is not contained in `SET`.
    ///
    /// # Safety
    ///
    /// One of the following conditions must hold, they are all logically equivalent:\
    /// 1. `Self::set_contains(value)`\
    /// 2. `Self::new(value).is_some()`\
    /// 3. `Self::SET` contains `value`
    #[must_use]
    pub const unsafe fn new_unchecked(value: T) -> Self {
        debug_assert!(
            Self::set_contains(&value),
            "Tried to create a Sure with a value thats not contained in its SET, this is UB."
        );
        Self(value)
    }

    /// Checks if the given value is contained in `SET`.
    #[must_use]
    pub const fn set_contains(value: &T) -> bool {
        ch::slice_contains(SET, value)
    }

    /// Checks if the given value is contained in `SET` using a binary search.
    ///
    /// If the `SET` is not sorted in ascending order this may return `false`, even when the value is actually contained.
    ///
    /// In that sense this function is more conservative than `set_contains`, only returning `true` when `set_contains` would also return `true`.
    #[must_use]
    pub const fn set_contains_via_binary_search(value: &T) -> bool {
        SET.binary_search(value).is_ok()
    }

    /// Returns the value stored in this `Sure`.
    #[must_use]
    pub const fn inner(self) -> T {
        self.0
    }

    /// This is a no-op on the contained value.
    ///
    /// It only sorts the values stored in `SET`.
    #[must_use]
    pub const fn sort(self) -> Sure<T, { set::SORT::<T, SET> }> {
        // SAFETY: `SORT` only sorts the elements in `SET`, so it's output will have identical elements.
        unsafe { self.cast_unchecked() }
    }

    /// This is a no-op on the contained value.
    ///
    /// It only sorts and deduplicates the values stored in `SET`.
    #[must_use]
    pub const fn normalize(self) -> Sure<T, { set::NORMALIZE::<T, SET> }> {
        // SAFETY: `NORMALIZE` only sorts and deduplicates the elements in `SET`, so it's output will have identical elements.
        // We rely on the unsafe `SureEq` trait to guarantee that we don't delete any unique elements.
        unsafe { self.cast_unchecked() }
    }

    /// This is a no-op on the contained value.
    ///
    /// It only changes `SET` to `SUPER_SET`, or fails to compile if `SET` isn't a subset of `SUPER_SET`.
    #[must_use]
    pub const fn widen<const SUPER_SET: &'static [T]>(self) -> Sure<T, SUPER_SET> {
        ch::const_assert!(
            ch::slice_is_subset(SET, SUPER_SET),
            "Tried to widen a Sure which failed because the target's SET isn't a superset of the original."
        );

        // SAFETY: We just asserted that `SET` is a subset of `SUPER_SET`
        unsafe { self.cast_unchecked() }
    }

    /// Creates a `Sure<T, NEW_SET>` if `self` currently stores a value that is contained in `NEW_SET`.
    ///
    /// # Errors
    ///
    /// Will return `Err(self)` if the given value is not contained in `SET`.
    pub const fn cast<const NEW_SET: &'static [T]>(self) -> Result<Sure<T, NEW_SET>, Self> {
        match Sure::<T, NEW_SET>::set_contains(&self.inner()) {
            true => Ok(
                // SAFETY: we just checked precondition #1: `Self::set_contains(value)`
                unsafe { self.cast_unchecked() },
            ),
            false => Err(self),
        }
    }

    /// # Safety
    ///
    /// This inherits the preconditions from `Sure<T, NEW_SET>::new_unchecked(self.inner())`.\
    /// The most common way to argue this is by making sure that `SET` has identical elements to `NEW_SET`,\
    /// or that it's elements are a subset of `NEW_SET`.
    #[must_use]
    pub const unsafe fn cast_unchecked<const NEW_SET: &'static [T]>(self) -> Sure<T, NEW_SET> {
        // SAFETY: we pass the preconditions to the caller
        unsafe { Sure::new_unchecked(self.inner()) }
    }
}
