#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(generic_const_args)]
#![feature(generic_const_items)]
#![feature(generic_const_parameter_types)]
#![feature(min_generic_const_args)]
#![feature(macroless_generic_const_args)]
#![allow(long_running_const_eval)]
#![allow(incomplete_features)]

#[allow(unused)]
const RANDOM: &[u8; 2_000_000] = include_bytes!("../random.bin");

#[allow(unused)]
const YOINK<T: 'static, const COUNT: usize>: &[T] = const {
    let byte_count: usize = size_of::<T>() * COUNT;
    unsafe { core::mem::transmute(&RANDOM[0..byte_count])}
};

macro_rules! bench {
    ($([type: $type:ident, count: $count:literal, feature: $feature:literal]),+) => {$(
        #[cfg(feature = $feature)]
        {
            let a: Sure<$type, { YOINK::<$type, $count> }> = Sure::new(0).unwrap();
            let b: Sure<$type, _> = a.normalize();
            assert_eq!(b.inner(), 0);
        }
    )+};
}

fn main() {
    use sure::base::Sure;

    bench!(
        [type: u8, count: 1_000, feature: "u8-1_000"],
        [type: u8, count: 2_000, feature: "u8-2_000"],
        [type: u8, count: 5_000, feature: "u8-5_000"],
        [type: u8, count: 10_000, feature: "u8-10_000"],
        [type: u8, count: 15_000, feature: "u8-15_000"],
        [type: u8, count: 20_000, feature: "u8-20_000"],
        [type: u8, count: 25_000, feature: "u8-25_000"],
        [type: u8, count: 30_000, feature: "u8-30_000"],
        [type: u8, count: 35_000, feature: "u8-35_000"],
        [type: u8, count: 40_000, feature: "u8-40_000"],
        [type: u8, count: 45_000, feature: "u8-45_000"],
        [type: u8, count: 50_000, feature: "u8-50_000"],
        [type: u8, count: 55_000, feature: "u8-55_000"],
        [type: u8, count: 60_000, feature: "u8-60_000"],
        [type: u8, count: 65_000, feature: "u8-65_000"],
        [type: u8, count: 70_000, feature: "u8-70_000"],
        [type: u8, count: 75_000, feature: "u8-75_000"],
        [type: u8, count: 80_000, feature: "u8-80_000"],
        [type: u8, count: 85_000, feature: "u8-85_000"],
        [type: u8, count: 90_000, feature: "u8-90_000"],
        [type: u8, count: 95_000, feature: "u8-95_000"]
    );

    println!("bench done!");
}

// 29.5s
// 25.0s
// 26.7s
// 40.2s
// 49.4s
// 17.2s
