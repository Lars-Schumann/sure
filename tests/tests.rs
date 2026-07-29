#![feature(const_result_trait_fn, const_trait_impl)]

use sure::*;

#[test]
fn one() {
    let foo: SureU8![1, 2, 3] = SureU8::new(2).unwrap();
    let bar: SureU8![10, 20] = SureU8::new(10).unwrap();
    let baz: SureU8![11, 21, 12, 22, 13, 23] = foo + bar;

    let _qux: SureU8![11, 12, 13, 21, 22, 23] = baz.sort();
}

#[test]
fn two() {
    let foo: SureU8![1, 1, 1] = SureU8::new(1).unwrap();
    let bar: SureU8![10, 20] = SureU8::new(10).unwrap();
    let baz: SureU8![11, 21, 11, 21, 11, 21] = foo + bar;

    let _qux: SureU8![11, 11, 11, 21, 21, 21] = baz.sort();
    let _qox: SureU8![11, 21] = baz.normalize();
}

#[test]
fn three() {
    let foo: SureU8![1, 1, 1, 2, 2] = SureU8::new(2).unwrap();
    let bar: SureU8![1, 2, 3] = SureU8::new(2).unwrap();
    let baz: SureU8![2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5] = (foo + bar).sort();

    let _qox: SureU8![2, 3, 4, 5] = baz.normalize();
}

#[test]
fn four() {
    let foo: SureU8![2, 4] = SureU8::new(2).unwrap();
    let bar: SureU8![1, 2, 3] = SureU8::new(3).unwrap();
    let baz: SureU8![2, 4, 4, 6, 8, 12] = (foo * bar).sort();

    let _qox: SureU8![2, 4, 6, 8, 12] = baz.normalize();
}

#[test]
fn five() {
    let a: SureI8![2, 4] = SureI8::new(4).unwrap();
    let b: SureI8![1, 3] = SureI8::new(1).unwrap();

    let _c: SureI8![3, 5, 7] = (a + b).normalize();
}

#[test]
fn huge() {
    use sure_u16::*;
    let r1: SureU16![Range![0..=4]] = SureU16::new(1).unwrap();
    let r2: SureU16![Range![10..=12]] = SureU16::new(10).unwrap();

    let _q: SureU16![0, 10, 11, 12, 20, 22, 24, 30, 33, 36, 40, 44, 48] = (r1 * r2).normalize();
}

#[test]
fn bleh() {
    let _r1: SureU8![4] = SureU8::new(4).unwrap();
}

#[test]
fn onion() {
    use sure_u32::{Range, Union};

    let _r1: SureU32![Union![Range![0..=2], Range![4..=5]]] =
        <SureU32![0, 1, 2, 4, 5]>::new(2).unwrap();
}

#[test]
fn onion2() {
    use sure_u32::{Range, Union};

    let _r1: SureU32![Union![Range![0..=2], Range![4..=5]]] =
        <SureU32![0, 1, 2, 4, 5]>::new(2).unwrap();
}

#[test]
fn ranges() {
    use sure_i8::Range;

    assert_eq!(Range![-3..2], &[-3, -2, -1, 0, 1]);
    assert_eq!(Range![125..], &[125, 126, 127]);
    assert_eq!(Range![-3..=2], &[-3, -2, -1, 0, 1, 2]);
    assert_eq!(Range![..-125], &[-128, -127, -126]);
    assert_eq!(Range![..=-125], &[-128, -127, -126, -125]);
}

#[test]
fn intersections() {
    use set::SORT;
    use sure_i8::Intersection;
    use sure_i8::Range;

    // Intersection on one set should be the identity
    assert_eq!(Intersection![Range![-3..27]], Range![-3..27]);

    // Intersection of one set with itself should be the identity
    assert_eq!(
        Intersection![Range![-3..27], Range![-3..27],],
        Range![-3..27]
    );

    // Intersection with the "full" set should be the identity
    assert_eq!(
        Intersection![Range![-3..27], Range![-128..=127]],
        Range![-3..27]
    );

    assert_eq!(
        SORT::<i8, { Intersection![Range![1..=20], Range![10..=30]] }>,
        Range![10..=20]
    );

    assert_eq!(
        SORT::<i8, { Intersection![Range![10..=50], Range![20..=100], Range![30..=40]] }>,
        Range![30..=40]
    );
}

#[test]
fn widen() {
    // {5} is a subset of 5
    let _: SureU32<{ &[5] }> = <SureU32![5]>::new(5).unwrap().widen();

    // {4} is a subset of {1, 2, 3}
    let _: SureU32![1, 2, 3] = <SureU32![3]>::new(3).unwrap().widen();

    // {6, 4} is a subset of {3, 4, 5, 6}
    let _: SureU32![3, 4, 5, 6] = <SureU32![6, 4]>::new(6).unwrap().widen();

    // {4, 4, 4} is a subset of {4}, for now? subject to change?
    let _: SureU32<{ &[4] }> = <SureU32![4, 4, 4]>::new(4).unwrap().widen();
}

#[test]
fn cast() {
    let _: SureU32<{ &[5] }> = <SureU32![1, 5]>::new(5).unwrap().cast().unwrap();

    let _: SureU32![1, 2, 3] = <SureU32![1, 2, 5]>::new(2).unwrap().cast().unwrap();

    let _: SureU32![1, 2] = <SureU32![1, 2, 3]>::new(2).unwrap().cast().unwrap();

    // let _: SureU32![1, 2] = <SureU32![3, 4]>::new(4).unwrap().cast().unwrap();
}

#[test]
fn generic() {
    use sure::base::Sure;

    let a: Sure<u8, { &[5, 4] }> = Sure::new(5).unwrap();
    let b: Sure<u8, { &[1, 2] }> = Sure::new(2).unwrap();

    let _c: Sure<u8, { &[5, 6, 7] }> = (b + a).normalize();

    let x: Sure<isize, { &[5, 4] }> = Sure::new(5).unwrap();
    let y: Sure<isize, { &[1, 2] }> = Sure::new(2).unwrap();

    let _z: Sure<isize, { &[4, 5, 8, 10] }> = (x * y).normalize();
}

#[test]
fn generic2() {
    use sure::base::Sure;
    use sure::sure_u16::Range;

    const {
        let a: Sure<u16, { Range![1..=4] }> = Sure::new(4).ok().unwrap();
        let b: Sure<u16, { Range![2..=6] }> = Sure::new(2).ok().unwrap();

        let _c: Sure<u16, { Range![3..=10] }> = a.wrapping_add(b).normalize();
        let _d = a.isqrt();
    }
}

#[test]
fn to_as() {
    use sure::base::Sure;
    use sure::sure_i8::Range as RangeI8;
    use sure::sure_i8::Union;
    use sure::sure_u16::Range as RangeU16;

    let a: SureU16![RangeU16![1..=130]] = Sure::new(4).unwrap();
    let _b: SureI8![Union![&[-128, -127, -126], RangeI8![1..=127]]] = a.as_i8().normalize();

    let c: SureU16![RangeU16![1..=5]] = Sure::new(2).unwrap();
    let _d: SureI8![RangeI8![1..=5]] = c.to_i8();
}

#[test]
fn to_nonzero() {
    use sure::base::Sure;
    let set_not_zero: SureI32![1, 2, 3] = Sure::new(1).unwrap();
    let _real_non_zero: core::num::NonZeroI32 = set_not_zero.to_non_zero();
}
