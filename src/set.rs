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
    ($name:ident, $t:ty, $uns_t:ty, $flip:expr) => {
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
            // SAFETY: Arrays are guaranteed to be contiguous.
            let bitset_chunks: [[bool; 64]; ELEMENT_COUNT / 64] = unsafe { core::mem::transmute(element_bitset) };
            let mut chunk_start: usize = 0;
            while chunk_start < ELEMENT_COUNT {
                let slot: usize = chunk_start ^ $flip;
                let [
                    b01, b02, b03, b04, b05, b06, b07, b08,
                    b09, b10, b11, b12, b13, b14, b15, b16,
                    b17, b18, b19, b20, b21, b22, b23, b24,
                    b25, b26, b27, b28, b29, b30, b31, b32,
                    b33, b34, b35, b36, b37, b38, b39, b40,
                    b41, b42, b43, b44, b45, b46, b47, b48,
                    b49, b50, b51, b52, b53, b54, b55, b56,
                    b57, b58, b59, b60, b61, b62, b63, b64,
                ] = bitset_chunks[slot / 64];
                // SAFETY: TODO
                #[rustfmt::skip]
                unsafe {
                    if b01 { *normalized_start.add(next_write_offset) =  slot       as $t; next_write_offset += 1; }
                    if b02 { *normalized_start.add(next_write_offset) = (slot |  1) as $t; next_write_offset += 1; }
                    if b03 { *normalized_start.add(next_write_offset) = (slot |  2) as $t; next_write_offset += 1; }
                    if b04 { *normalized_start.add(next_write_offset) = (slot |  3) as $t; next_write_offset += 1; }
                    if b05 { *normalized_start.add(next_write_offset) = (slot |  4) as $t; next_write_offset += 1; }
                    if b06 { *normalized_start.add(next_write_offset) = (slot |  5) as $t; next_write_offset += 1; }
                    if b07 { *normalized_start.add(next_write_offset) = (slot |  6) as $t; next_write_offset += 1; }
                    if b08 { *normalized_start.add(next_write_offset) = (slot |  7) as $t; next_write_offset += 1; }
                    if b09 { *normalized_start.add(next_write_offset) = (slot |  8) as $t; next_write_offset += 1; }
                    if b10 { *normalized_start.add(next_write_offset) = (slot |  9) as $t; next_write_offset += 1; }
                    if b11 { *normalized_start.add(next_write_offset) = (slot | 10) as $t; next_write_offset += 1; }
                    if b12 { *normalized_start.add(next_write_offset) = (slot | 11) as $t; next_write_offset += 1; }
                    if b13 { *normalized_start.add(next_write_offset) = (slot | 12) as $t; next_write_offset += 1; }
                    if b14 { *normalized_start.add(next_write_offset) = (slot | 13) as $t; next_write_offset += 1; }
                    if b15 { *normalized_start.add(next_write_offset) = (slot | 14) as $t; next_write_offset += 1; }
                    if b16 { *normalized_start.add(next_write_offset) = (slot | 15) as $t; next_write_offset += 1; }
                    if b17 { *normalized_start.add(next_write_offset) = (slot | 16) as $t; next_write_offset += 1; }
                    if b18 { *normalized_start.add(next_write_offset) = (slot | 17) as $t; next_write_offset += 1; }
                    if b19 { *normalized_start.add(next_write_offset) = (slot | 18) as $t; next_write_offset += 1; }
                    if b20 { *normalized_start.add(next_write_offset) = (slot | 19) as $t; next_write_offset += 1; }
                    if b21 { *normalized_start.add(next_write_offset) = (slot | 20) as $t; next_write_offset += 1; }
                    if b22 { *normalized_start.add(next_write_offset) = (slot | 21) as $t; next_write_offset += 1; }
                    if b23 { *normalized_start.add(next_write_offset) = (slot | 22) as $t; next_write_offset += 1; }
                    if b24 { *normalized_start.add(next_write_offset) = (slot | 23) as $t; next_write_offset += 1; }
                    if b25 { *normalized_start.add(next_write_offset) = (slot | 24) as $t; next_write_offset += 1; }
                    if b26 { *normalized_start.add(next_write_offset) = (slot | 25) as $t; next_write_offset += 1; }
                    if b27 { *normalized_start.add(next_write_offset) = (slot | 26) as $t; next_write_offset += 1; }
                    if b28 { *normalized_start.add(next_write_offset) = (slot | 27) as $t; next_write_offset += 1; }
                    if b29 { *normalized_start.add(next_write_offset) = (slot | 28) as $t; next_write_offset += 1; }
                    if b30 { *normalized_start.add(next_write_offset) = (slot | 29) as $t; next_write_offset += 1; }
                    if b31 { *normalized_start.add(next_write_offset) = (slot | 30) as $t; next_write_offset += 1; }
                    if b32 { *normalized_start.add(next_write_offset) = (slot | 31) as $t; next_write_offset += 1; }
                    if b33 { *normalized_start.add(next_write_offset) = (slot | 32) as $t; next_write_offset += 1; }
                    if b34 { *normalized_start.add(next_write_offset) = (slot | 33) as $t; next_write_offset += 1; }
                    if b35 { *normalized_start.add(next_write_offset) = (slot | 34) as $t; next_write_offset += 1; }
                    if b36 { *normalized_start.add(next_write_offset) = (slot | 35) as $t; next_write_offset += 1; }
                    if b37 { *normalized_start.add(next_write_offset) = (slot | 36) as $t; next_write_offset += 1; }
                    if b38 { *normalized_start.add(next_write_offset) = (slot | 37) as $t; next_write_offset += 1; }
                    if b39 { *normalized_start.add(next_write_offset) = (slot | 38) as $t; next_write_offset += 1; }
                    if b40 { *normalized_start.add(next_write_offset) = (slot | 39) as $t; next_write_offset += 1; }
                    if b41 { *normalized_start.add(next_write_offset) = (slot | 40) as $t; next_write_offset += 1; }
                    if b42 { *normalized_start.add(next_write_offset) = (slot | 41) as $t; next_write_offset += 1; }
                    if b43 { *normalized_start.add(next_write_offset) = (slot | 42) as $t; next_write_offset += 1; }
                    if b44 { *normalized_start.add(next_write_offset) = (slot | 43) as $t; next_write_offset += 1; }
                    if b45 { *normalized_start.add(next_write_offset) = (slot | 44) as $t; next_write_offset += 1; }
                    if b46 { *normalized_start.add(next_write_offset) = (slot | 45) as $t; next_write_offset += 1; }
                    if b47 { *normalized_start.add(next_write_offset) = (slot | 46) as $t; next_write_offset += 1; }
                    if b48 { *normalized_start.add(next_write_offset) = (slot | 47) as $t; next_write_offset += 1; }
                    if b49 { *normalized_start.add(next_write_offset) = (slot | 48) as $t; next_write_offset += 1; }
                    if b50 { *normalized_start.add(next_write_offset) = (slot | 49) as $t; next_write_offset += 1; }
                    if b51 { *normalized_start.add(next_write_offset) = (slot | 50) as $t; next_write_offset += 1; }
                    if b52 { *normalized_start.add(next_write_offset) = (slot | 51) as $t; next_write_offset += 1; }
                    if b53 { *normalized_start.add(next_write_offset) = (slot | 52) as $t; next_write_offset += 1; }
                    if b54 { *normalized_start.add(next_write_offset) = (slot | 53) as $t; next_write_offset += 1; }
                    if b55 { *normalized_start.add(next_write_offset) = (slot | 54) as $t; next_write_offset += 1; }
                    if b56 { *normalized_start.add(next_write_offset) = (slot | 55) as $t; next_write_offset += 1; }
                    if b57 { *normalized_start.add(next_write_offset) = (slot | 56) as $t; next_write_offset += 1; }
                    if b58 { *normalized_start.add(next_write_offset) = (slot | 57) as $t; next_write_offset += 1; }
                    if b59 { *normalized_start.add(next_write_offset) = (slot | 58) as $t; next_write_offset += 1; }
                    if b60 { *normalized_start.add(next_write_offset) = (slot | 59) as $t; next_write_offset += 1; }
                    if b61 { *normalized_start.add(next_write_offset) = (slot | 60) as $t; next_write_offset += 1; }
                    if b62 { *normalized_start.add(next_write_offset) = (slot | 61) as $t; next_write_offset += 1; }
                    if b63 { *normalized_start.add(next_write_offset) = (slot | 62) as $t; next_write_offset += 1; }
                    if b64 { *normalized_start.add(next_write_offset) = (slot | 63) as $t; next_write_offset += 1; }
                }
                chunk_start += 64;
            }

            // SAFETY: TODO
            unsafe { normalized.set_len(next_write_offset) }
            normalized
        }
    };
}

define_normalize_narrow!(normalize_u8, u8, u8, 0);
define_normalize_narrow!(normalize_u16, u16, u16, 0);
define_normalize_narrow!(normalize_i8, i8, u8, 128);
define_normalize_narrow!(normalize_i16, i16, u16, 32768);
