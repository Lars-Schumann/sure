use paste::paste;
use seq_macro::seq;

fn std_codomain(a_set: &[i32], b_set: &[i32], f: impl Fn(i32, i32) -> i32) -> Vec<i32> {
    let mut codomain = Vec::with_capacity(a_set.len() * b_set.len());
    for &a in a_set {
        for &b in b_set {
            codomain.push(f(a, b));
        }
    }
    codomain.sort();
    codomain
}

seq!(A_LEN in 0..=20 {
    seq!(B_LEN in 0..=20 {
        paste! {
            #[test]
            #[allow(clippy::identity_op, clippy::erasing_op)]
            fn [<codomain_ops_bitor_ A_LEN _ B_LEN>]() {
                for _ in 0..100 {
                    let a: [i32; A_LEN] = core::array::from_fn(|_| fastrand::i32(..));
                    let b: [i32; B_LEN] = core::array::from_fn(|_| fastrand::i32(..));
                    let mut actual = sure::i32_bitor::codomain::<{ A_LEN * B_LEN }>(&a, &b);
                    actual.sort();
                    assert_eq!(actual.as_slice(), std_codomain(&a, &b, core::ops::BitOr::bitor));
                }
            }

            #[test]
            #[allow(clippy::identity_op, clippy::erasing_op)]
            fn [<codomain_fns_wrapping_add_ A_LEN _ B_LEN>]() {
                for _ in 0..100 {
                    let a: [i32; A_LEN] = core::array::from_fn(|_| fastrand::i32(..));
                    let b: [i32; B_LEN] = core::array::from_fn(|_| fastrand::i32(..));
                    let mut actual = sure::i32_i32_wrapping_add::codomain::<{ A_LEN * B_LEN }>(&a, &b);
                    actual.sort();
                    assert_eq!(actual.as_slice(), std_codomain(&a, &b, i32::wrapping_add));
                }
            }
        }
    });
});
