use alloc::vec;
use alloc::vec::Vec;
use core::marker::Destruct;

use crate::base::Set;
use crate::const_helpers as ch;
use crate::normalize;
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
        ($($ty:ty => $normalize_fn:ident),+ ) => {$(
            match try_fn_once::<[T; LEN], Vec<T>, [$ty; LEN], Vec<$ty>>(
                arr, normalize::$normalize_fn::<LEN>,
            ) {
                Ok(normalized) => return normalized.const_make_global(),
                Err(_arr) => {},
            };
        )+};
    }
    specialize! {
        u32    => normalize_u32,
        u64    => normalize_u64,
        u128   => normalize_u128,
        usize  => normalize_usize,
        i32    => normalize_i32,
        i64    => normalize_i64,
        i128   => normalize_i128,
        isize  => normalize_isize
    }

    let sorted = ch::sort(arr);
    deduped(&sorted).const_make_global()
}

macro_rules! define_normalize_narrow {
    ($name:ident, $t:ty, $uns_t:ty, [$($slots:expr),+]) => {
        #[allow(clippy::large_stack_arrays, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::too_many_lines)]
        const fn $name<const LEN: usize>(arr: [$t; LEN]) -> Vec<$t> {
            const ELEMENT_COUNT: usize = (<$uns_t>::MAX as usize) + 1;

            let mut element_bitset: [bool; ELEMENT_COUNT] = [false; ELEMENT_COUNT];

            let (arr_chunks, arr_tail) = arr.as_chunks::<64>();
            let (arr_chunks_len, arr_tail_len) = (arr_chunks.len(), arr_tail.len());

            let mut arr_chunks_index: usize = 0;

            while arr_chunks_index < arr_chunks_len {
                let arr_chunk = arr_chunks[arr_chunks_index];
                arr_chunks_index += 1;
                let [
                    a01, a02, a03, a04, a05, a06, a07, a08,
                    a09, a10, a11, a12, a13, a14, a15, a16,
                    a17, a18, a19, a20, a21, a22, a23, a24,
                    a25, a26, a27, a28, a29, a30, a31, a32,
                    a33, a34, a35, a36, a37, a38, a39, a40,
                    a41, a42, a43, a44, a45, a46, a47, a48,
                    a49, a50, a51, a52, a53, a54, a55, a56,
                    a57, a58, a59, a60, a61, a62, a63, a64,
                ] = arr_chunk;
                element_bitset[a01 as $uns_t as usize] = true;
                element_bitset[a02 as $uns_t as usize] = true;
                element_bitset[a03 as $uns_t as usize] = true;
                element_bitset[a04 as $uns_t as usize] = true;
                element_bitset[a05 as $uns_t as usize] = true;
                element_bitset[a06 as $uns_t as usize] = true;
                element_bitset[a07 as $uns_t as usize] = true;
                element_bitset[a08 as $uns_t as usize] = true;
                element_bitset[a09 as $uns_t as usize] = true;
                element_bitset[a10 as $uns_t as usize] = true;
                element_bitset[a11 as $uns_t as usize] = true;
                element_bitset[a12 as $uns_t as usize] = true;
                element_bitset[a13 as $uns_t as usize] = true;
                element_bitset[a14 as $uns_t as usize] = true;
                element_bitset[a15 as $uns_t as usize] = true;
                element_bitset[a16 as $uns_t as usize] = true;
                element_bitset[a17 as $uns_t as usize] = true;
                element_bitset[a18 as $uns_t as usize] = true;
                element_bitset[a19 as $uns_t as usize] = true;
                element_bitset[a20 as $uns_t as usize] = true;
                element_bitset[a21 as $uns_t as usize] = true;
                element_bitset[a22 as $uns_t as usize] = true;
                element_bitset[a23 as $uns_t as usize] = true;
                element_bitset[a24 as $uns_t as usize] = true;
                element_bitset[a25 as $uns_t as usize] = true;
                element_bitset[a26 as $uns_t as usize] = true;
                element_bitset[a27 as $uns_t as usize] = true;
                element_bitset[a28 as $uns_t as usize] = true;
                element_bitset[a29 as $uns_t as usize] = true;
                element_bitset[a30 as $uns_t as usize] = true;
                element_bitset[a31 as $uns_t as usize] = true;
                element_bitset[a32 as $uns_t as usize] = true;
                element_bitset[a33 as $uns_t as usize] = true;
                element_bitset[a34 as $uns_t as usize] = true;
                element_bitset[a35 as $uns_t as usize] = true;
                element_bitset[a36 as $uns_t as usize] = true;
                element_bitset[a37 as $uns_t as usize] = true;
                element_bitset[a38 as $uns_t as usize] = true;
                element_bitset[a39 as $uns_t as usize] = true;
                element_bitset[a40 as $uns_t as usize] = true;
                element_bitset[a41 as $uns_t as usize] = true;
                element_bitset[a42 as $uns_t as usize] = true;
                element_bitset[a43 as $uns_t as usize] = true;
                element_bitset[a44 as $uns_t as usize] = true;
                element_bitset[a45 as $uns_t as usize] = true;
                element_bitset[a46 as $uns_t as usize] = true;
                element_bitset[a47 as $uns_t as usize] = true;
                element_bitset[a48 as $uns_t as usize] = true;
                element_bitset[a49 as $uns_t as usize] = true;
                element_bitset[a50 as $uns_t as usize] = true;
                element_bitset[a51 as $uns_t as usize] = true;
                element_bitset[a52 as $uns_t as usize] = true;
                element_bitset[a53 as $uns_t as usize] = true;
                element_bitset[a54 as $uns_t as usize] = true;
                element_bitset[a55 as $uns_t as usize] = true;
                element_bitset[a56 as $uns_t as usize] = true;
                element_bitset[a57 as $uns_t as usize] = true;
                element_bitset[a58 as $uns_t as usize] = true;
                element_bitset[a59 as $uns_t as usize] = true;
                element_bitset[a60 as $uns_t as usize] = true;
                element_bitset[a61 as $uns_t as usize] = true;
                element_bitset[a62 as $uns_t as usize] = true;
                element_bitset[a63 as $uns_t as usize] = true;
                element_bitset[a64 as $uns_t as usize] = true;
            }


            let mut arr_tail_index: usize = 0;

            while arr_tail_index < arr_tail_len {
                element_bitset[arr_tail[arr_tail_index] as $uns_t as usize] = true;
                arr_tail_index += 1;
            }

            let mut normalized: Vec<$t> = Vec::with_capacity(ELEMENT_COUNT);

            let mut next_write_offset: usize = 0;
            let normalized_start: *mut $t = normalized.as_mut_ptr();
            $(
                let core::ops::Range { start, end } = $slots;
                let mut slot: usize = start;
                while slot < end {
                    if element_bitset[slot] {
                        // SAFETY: TODO
                        unsafe { *normalized_start.add(next_write_offset) = slot as $t };
                        next_write_offset += 1;
                    }
                    slot += 1;
                }
            )+

            // SAFETY: TODO
            unsafe { normalized.set_len(next_write_offset) }
            normalized
        }
    };
}

define_normalize_narrow!(normalize_u8, u8, u8, [0..256]);
define_normalize_narrow!(normalize_u16, u16, u16, [0..65536]);
define_normalize_narrow!(normalize_i8, i8, u8, [128..256, 0..128]);
define_normalize_narrow!(normalize_i16, i16, u16, [32768..65536, 0..32768]);
