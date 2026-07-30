use alloc::vec;
use alloc::vec::Vec;
use core::marker::ConstParamTy_;
use core::marker::Destruct;
use core::marker::Freeze;

use crate::const_helpers as ch;
use crate::spec::try_fn_once;
use crate::sure_eq::SureEq;

pub(crate) const LENGTH<T: ConstParamTy_ + 'static, const SET: &'static [T]>: usize =
    const { SET.len() };

pub(crate) const CARTESIAN_LENGTH<
    T: ConstParamTy_ + 'static,
    U: ConstParamTy_ + 'static,
    const A: &'static [T],
    const B: &'static [U],
>: usize = const { A.len() * B.len() };

/// Returns the input slice but sorted.
#[expect(clippy::ok_expect)]
pub const SORT<
    T: const Ord + ConstParamTy_ + Copy + const Destruct + Freeze + 'static,
    const SET: &'static [T]
>: &[T] = const {
    let arr: [T; LENGTH::<T, SET>] = SET.try_into().ok().expect("this is infallible");
    &ch::sort(arr)
};

/// Returns the input slice but normalized(sorted + deduplicated).
pub const NORMALIZE<
    T: SureEq + const Ord + Copy + const Destruct + 'static,
    const SET: &'static [T],
>: &[T] = const {
    normalize::<T, {LENGTH::<T, SET>}>(SET).const_make_global()
};

/// Returns the input slices concatenated with each other.
pub const UNION<T: ConstParamTy_ + Copy + Freeze + 'static , const SETS: &'static [&'static [T]]>:
    &[T] = const {
    union_(SETS).const_make_global()
};

/// Returns the intersection of all input slices.
pub const INTERSECTION<
    T: SureEq + Copy + const Destruct + 'static ,
    const SETS: &'static [&'static [T]],
>: &[T] = const {
    intersection(SETS).const_make_global()
};

const fn deduped<T: SureEq + Copy>(slice: &[T]) -> Vec<T> {
    let [first, ..] = slice else { return vec![] };

    // FIXME: make this `vec![first]` once possible in const
    let mut deduped: Vec<T> = vec![];
    deduped.push(*first);

    let mut i = 1; // starting at the 2nd element, since the first one is always unique
    while i < slice.len() {
        let (previous, current) = (slice[i - 1], slice[i]);
        if previous != current {
            deduped.push(current);
        }
        i += 1;
    }
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

#[expect(clippy::ok_expect)]
const fn normalize<
    T: SureEq + [const] Ord + Copy + [const] Destruct + 'static,
    const LEN: usize,
>(
    slice: &'static [T],
) -> Vec<T> {
    let slice: &[T] = match try_fn_once::<&[T], Vec<T>, &[u8], Vec<u8>, _>(slice, normalize_u8) {
        Ok(normalized) => return normalized,
        Err(slice) => slice,
    };

    let slice: &[T] = match try_fn_once::<&[T], Vec<T>, &[u16], Vec<u16>, _>(slice, normalize_u16) {
        Ok(normalized) => return normalized,
        Err(slice) => slice,
    };

    let arr: [T; LEN] = slice.try_into().ok().expect("this is infallible");
    let sorted = ch::sort(arr);
    deduped(&sorted)
}

// FIXME: this would be way less ugly with const Range Iterators
const fn normalize_u8(slice: &[u8]) -> Vec<u8> {
    const LEN: usize = u8::MAX as usize + 1;
    let mut set: [bool; LEN] = [false; LEN];

    // for elem in slice: set[usize::from(elem)] = true
    let mut i: usize = 0;
    while i < slice.len() {
        set[usize::from(slice[i])] = true;
        i += 1;
    }

    let mut normalized: Vec<u8> = Vec::with_capacity(LEN);

    // for i in 0..=u8::MAX: if set[usize::from(i)]: normalized.push(i)
    let mut i: u8 = 0;
    loop {
        if set[i as usize] {
            normalized.push(i);
        }

        i = i.wrapping_add(1);
        if i == 0 {
            break;
        }
    }

    normalized
}

#[expect(clippy::large_stack_arrays)]
const fn normalize_u16(slice: &[u16]) -> Vec<u16> {
    const LEN: usize = u16::MAX as usize + 1;
    let mut set: [bool; LEN] = [false; LEN];

    // for elem in slice: set[usize::from(elem)] = trues
    let mut i: usize = 0;
    while i < slice.len() {
        set[usize::from(slice[i])] = true;
        i += 1;
    }

    let mut normalized: Vec<u16> = Vec::with_capacity(LEN);

    // for i in 0..=u16::MAX: if set[usize::from(i)]: normalized.push(i)
    let mut i: u16 = 0;
    loop {
        if set[i as usize] {
            normalized.push(i);
        }
        i = i.wrapping_add(1);
        if i == 0 {
            break;
        }
    }

    normalized
}
