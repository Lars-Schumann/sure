#![feature(macro_metavar_expr_concat)]

use sure::set::normalize;

fn std_normalized<T: Clone + Ord, const N: usize>(mut arr: [T; N]) -> Vec<T> {
    arr.sort();
    let mut arr = arr.to_vec();
    arr.dedup();
    arr
}

macro_rules! stuff {
    ($ty:ident, [$($len:literal),+]) => {
        $(
            #[test]
            fn ${ concat($ty, "_", $len) }() {
                for _ in 0..100 {
                    let arr: [$ty; $len] = core::array::from_fn(|_i| fastrand::$ty(..));
                    assert_eq!(normalize(arr), std_normalized(arr));
                }
            }
        )+
    };
}

macro_rules! stuffs {
    ($($ty:ident),+) => {
        $(stuff!($ty, [1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000]);)+
    };
}

stuffs!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
