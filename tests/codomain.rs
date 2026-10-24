#![feature(macro_metavar_expr_concat)]

fn std_codomain(a_set: &[i32], b_set: &[i32]) -> Vec<i32> {
    let mut codomain = Vec::with_capacity(a_set.len() * b_set.len());
    for &a in a_set {
        for &b in b_set {
            codomain.push(a | b);
        }
    }
    codomain.sort();
    codomain
}

macro_rules! stuff {
    ($a_len:literal, [$($b_len:literal),+]) => {
        $(
            #[test]
            fn ${concat(codomain_, $a_len, "_", $b_len)}() {
                for _ in 0..100 {
                    let a: [i32; $a_len] = core::array::from_fn(|_| fastrand::i32(..));
                    let b: [i32; $b_len] = core::array::from_fn(|_| fastrand::i32(..));
                    let mut actual = sure::i32_bitor::codomain::<{ $a_len * $b_len }>(&a, &b);
                    actual.sort();
                    assert_eq!(actual.as_slice(), std_codomain(&a, &b));
                }
            }
        )+
    };
}

macro_rules! stuffs {
    ([$($a_len:literal),+], $b_lens:tt) => {
        $(stuff!($a_len, $b_lens);)+
    };
}

stuffs!(
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
);
