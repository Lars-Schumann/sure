use paste::paste;
use seq_macro::seq;

fn std_image(a_set: &[i32], b_set: &[i32], f: impl Fn(i32, i32) -> i32) -> Vec<i32> {
    let mut image = Vec::with_capacity(a_set.len() * b_set.len());
    for &a in a_set {
        for &b in b_set {
            image.push(f(a, b));
        }
    }
    image.sort();
    image
}

seq!(A_LEN in 0..=20 {
    seq!(B_LEN in 0..=20 {
        paste! {
            #[test]
            #[allow(clippy::identity_op, clippy::erasing_op)]
            fn [<image_ops_bitor_ A_LEN _ B_LEN>]() {
                for _ in 0..100 {
                    let a: [i32; A_LEN] = core::array::from_fn(|_| fastrand::i32(..));
                    let b: [i32; B_LEN] = core::array::from_fn(|_| fastrand::i32(..));
                    let mut actual = sure::i32_bitor::image::<{ A_LEN * B_LEN }>(&a, &b);
                    actual.sort();
                    assert_eq!(actual.as_slice(), std_image(&a, &b, core::ops::BitOr::bitor));
                }
            }

            #[test]
            #[allow(clippy::identity_op, clippy::erasing_op)]
            fn [<image_fns_wrapping_add_ A_LEN _ B_LEN>]() {
                for _ in 0..100 {
                    let a: [i32; A_LEN] = core::array::from_fn(|_| fastrand::i32(..));
                    let b: [i32; B_LEN] = core::array::from_fn(|_| fastrand::i32(..));
                    let mut actual = sure::i32_i32_wrapping_add::image::<{ A_LEN * B_LEN }>(&a, &b);
                    actual.sort();
                    assert_eq!(actual.as_slice(), std_image(&a, &b, i32::wrapping_add));
                }
            }
        }
    });
});
