use alloc::vec;
use alloc::vec::Vec;
use core::marker::Destruct;

use crate::base::Set;
use crate::const_helpers as ch;
use crate::sort;
use crate::spec::try_fn_once;
use crate::sure_eq::SureEq;

pub(crate) const LENGTH<T: SureEq, const SET: Set<T>>: usize = const { SET.len() };

pub(crate) const CARTESIAN_LENGTH<T: SureEq, U: SureEq, const A: Set<T>, const B: Set<U>>: usize =
    const { A.len() * B.len() };

/// Returns the input slice, normalized (sorted and deduplicated).
pub const NORMALIZE<T: SureEq + const Ord + Copy + const Destruct, const SET: Set<T>>: Set<T> = const {
    let arr: [T; LENGTH::<T, SET>] = SET.try_into().ok().unwrap();
    normalize(arr)
};

/// Returns the input slices concatenated with each other.
pub const UNION<T: SureEq + Copy, const SETS: Set<Set<T>>>: Set<T> =
    const { union_(SETS).const_make_global() };

/// Returns the intersection of all input slices.
pub const INTERSECTION<T: SureEq + Copy + const Destruct, const SETS: Set<Set<T>>>: Set<T> =
    const { intersection(SETS).const_make_global() };

const fn deduped<T: SureEq + Copy>(slice: &[T]) -> Vec<T> {
    let [first, ..] = slice else { return vec![] };

    let slice_len = slice.len();

    let mut deduped: Vec<T> = Vec::with_capacity(slice_len);

    deduped.push(*first);

    let deduped_start: *mut T = deduped.as_mut_ptr();

    let mut i: usize = 1; // starting at the 2nd element, since the first one is always unique
    let mut next_write_offset: usize = 1;
    while i < slice_len {
        let (previous, current) = (slice[i - 1], slice[i]);
        if previous != current {
            // SAFETY: TODO, do we need to ptr::write here for drop reasons?
            unsafe { *deduped_start.add(next_write_offset) = current }
            next_write_offset += 1;
        }
        i += 1;
    }

    // SAFETY: TODO
    unsafe { deduped.set_len(next_write_offset) }
    deduped
}

const fn union_<T: Copy>(sets: &[&[T]]) -> Vec<T> {
    let mut union_: Vec<T> = vec![];
    let mut i: usize = 0;

    while i < sets.len() {
        let mut j: usize = 0;
        while j < sets[i].len() {
            union_.push(sets[i][j]);
            j += 1;
        }
        i += 1;
    }

    union_
}

const fn intersection<T: SureEq + Copy + [const] Destruct>(sets: &[&[T]]) -> Vec<T> {
    let [first_set, ..] = sets else {
        return vec![];
    };
    let mut intersection: Vec<T> = ch::slice_to_vec(first_set);

    let mut i: usize = 1; // starting at the 2nd element, since the first is already part of the intersection
    while i < sets.len() {
        ch::vec_reduce_to_intersection_with(&mut intersection, sets[i]);
        i += 1;
    }

    intersection
}

#[doc(hidden)]
pub const fn normalize<T: SureEq + [const] Ord + Copy + [const] Destruct, const LEN: usize>(
    arr: [T; LEN],
) -> Set<T> {
    match try_fn_once::<[T; LEN], Vec<T>, [u8; LEN], Vec<u8>>(arr, normalize_u8) {
        Ok(normalized) => return normalized.const_make_global(),
        Err(_slice) => {}
    }

    match try_fn_once::<[T; LEN], Vec<T>, [u16; LEN], Vec<u16>>(arr, normalize_u16) {
        Ok(normalized) => return normalized.const_make_global(),
        Err(_slice) => {}
    }

    match try_fn_once::<[T; LEN], Vec<T>, [i8; LEN], Vec<i8>>(arr, normalize_i8) {
        Ok(normalized) => return normalized.const_make_global(),
        Err(_slice) => {}
    }

    match try_fn_once::<[T; LEN], Vec<T>, [i16; LEN], Vec<i16>>(arr, normalize_i16) {
        Ok(normalized) => return normalized.const_make_global(),
        Err(_slice) => {}
    }

    macro_rules! specialize {
        ($($ty:ty => $sort_fn:ident),+ ) => {$(
            match try_fn_once::<[T; LEN], [T; LEN], [$ty; LEN], [$ty; LEN]>(
                arr, sort::$sort_fn::<LEN>,
            ) {
                Ok(sorted) => return deduped(&sorted).const_make_global(),
                Err(_arr) => {},
            };
        )+};
    }
    specialize! {
        u32    => sort_u32,
        u64    => sort_u64,
        u128   => sort_u128,
        usize  => sort_usize,
        i32    => sort_i32,
        i64    => sort_i64,
        i128   => sort_i128,
        isize  => sort_isize
    }

    let sorted = ch::sort(arr);
    deduped(&sorted).const_make_global()
}

macro_rules! define_normalize_narrow {
    ($uns_t:ty, $snd_t:ty, $uns_fn:ident, $snd_fn:ident) => {
        #[allow(clippy::large_stack_arrays)]
        const fn $uns_fn<const LEN: usize>(arr: [$uns_t; LEN]) -> Vec<$uns_t> {
            const ELEMENT_COUNT: usize = (<$uns_t>::MAX as usize) + 1;

            let mut element_bitset: [bool; ELEMENT_COUNT] = [false; ELEMENT_COUNT];

            let mut i: usize = 0;
            while i < LEN {
                element_bitset[arr[i] as usize] = true;
                i += 1;
            }

            let mut normalized: Vec<$uns_t> = Vec::with_capacity(ELEMENT_COUNT);

            let mut i: $uns_t = 0;
            let mut next_write_offset: usize = 0;
            let normalized_start: *mut $uns_t = normalized.as_mut_ptr();
            loop {
                if element_bitset[i as usize] {
                    // SAFETY: TODO
                    unsafe { *normalized_start.add(next_write_offset) = i };
                    next_write_offset += 1;
                }

                if i == <$uns_t>::MAX {
                    break;
                }
                i += 1;
            }

            // SAFETY: TODO
            unsafe { normalized.set_len(next_write_offset) }
            normalized
        }

        #[allow(clippy::similar_names, clippy::undocumented_unsafe_blocks)]
        const fn $snd_fn<const LEN: usize>(mut arr: [$snd_t; LEN]) -> Vec<$snd_t> {
            // flip sign bit so it sorts the same way as the unsigned type
            let mut i: usize = 0;
            while i < LEN {
                arr[i] ^= <$snd_t>::MIN;
                i += 1;
            }

            let arr_uns: [$uns_t; LEN] = unsafe { core::intrinsics::transmute_unchecked(arr) };
            let vec_uns: Vec<$uns_t> = $uns_fn(arr_uns);
            // SAFETY: obviously unsound but urgh
            let mut vec_snd: Vec<$snd_t> = unsafe { core::mem::transmute(vec_uns) };

            // unflip sign bit
            let vec_len = vec_snd.len();
            let slice = vec_snd.as_mut_slice();
            let mut i: usize = 0;
            while i < vec_len {
                slice[i] ^= <$snd_t>::MIN;
                i += 1;
            }

            vec_snd
        }
    };
}

define_normalize_narrow!(u8, i8, normalize_u8, normalize_i8);
define_normalize_narrow!(u16, i16, normalize_u16, normalize_i16);
