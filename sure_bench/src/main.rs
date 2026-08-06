#![feature(generic_const_args)]
#![feature(generic_const_items)]
#![feature(generic_const_parameter_types)]
#![feature(min_generic_const_args)]
#![feature(macroless_generic_const_args)]
#![allow(long_running_const_eval)]
#![allow(incomplete_features)]

#[repr(C, align(16))]
struct Aligned<T>(pub T);

#[allow(unused)]
static RANDOM: Aligned<[u8; 2_000_000]> = Aligned(*include_bytes!("../random.bin"));

#[allow(unused)]
const YOINK<T: 'static, const COUNT: usize>: &[T] = const {
    unsafe { core::slice::from_raw_parts(RANDOM.0.as_ptr().cast::<T>(), COUNT) }
};

macro_rules! bench_normalize {
    ($([type: $type:ident, count: $count:literal, feature: $feature:literal]),+ $(,)?) => {$(
        #[cfg(feature = $feature)]
        {
            let a: Sure<$type, { YOINK::<$type, $count> }> = Sure::new(0).unwrap();
            let b: Sure<$type, _> = a.normalize();
            assert_eq!(b.inner(), 0);
        }
    )+};
}

macro_rules! bench_cartesian_product {
    ($([count: $count:literal, feature: $feature:literal]),+ $(,)?) => {$(
        #[cfg(feature = $feature)]
        {
            use sure::SureI32;
            let a: SureI32<{ YOINK::<i32, $count> }> = SureI32::new(0).unwrap();
            let n: SureI32![2] = Sure::new(2).unwrap();
            let b: SureI32<_> = a / n;
            assert_eq!(b.inner(), 0);
        }
    )+};
}

fn main() {
    #[allow(unused)]
    use sure::base::Sure;

    bench_normalize!(
        [ type: u8,    count: 1_000,   feature: "u8-1_000"    ],
        [ type: u8,    count: 2_000,   feature: "u8-2_000"    ],
        [ type: u8,    count: 5_000,   feature: "u8-5_000"    ],
        [ type: u8,    count: 10_000,  feature: "u8-10_000"   ],
        [ type: u8,    count: 15_000,  feature: "u8-15_000"   ],
        [ type: u8,    count: 20_000,  feature: "u8-20_000"   ],
        [ type: u8,    count: 25_000,  feature: "u8-25_000"   ],
        [ type: u8,    count: 30_000,  feature: "u8-30_000"   ],
        [ type: u8,    count: 35_000,  feature: "u8-35_000"   ],
        [ type: u8,    count: 40_000,  feature: "u8-40_000"   ],
        [ type: u8,    count: 45_000,  feature: "u8-45_000"   ],
        [ type: u8,    count: 50_000,  feature: "u8-50_000"   ],
        [ type: u8,    count: 55_000,  feature: "u8-55_000"   ],
        [ type: u8,    count: 60_000,  feature: "u8-60_000"   ],
        [ type: u8,    count: 65_000,  feature: "u8-65_000"   ],
        [ type: u8,    count: 70_000,  feature: "u8-70_000"   ],
        [ type: u8,    count: 75_000,  feature: "u8-75_000"   ],
        [ type: u8,    count: 80_000,  feature: "u8-80_000"   ],
        [ type: u8,    count: 85_000,  feature: "u8-85_000"   ],
        [ type: u8,    count: 90_000,  feature: "u8-90_000"   ],
        [ type: u8,    count: 95_000,  feature: "u8-95_000"   ],

        [ type: u16,   count: 1_000,   feature: "u16-1_000"   ],
        [ type: u16,   count: 2_000,   feature: "u16-2_000"   ],
        [ type: u16,   count: 5_000,   feature: "u16-5_000"   ],
        [ type: u16,   count: 10_000,  feature: "u16-10_000"  ],
        [ type: u16,   count: 15_000,  feature: "u16-15_000"  ],
        [ type: u16,   count: 20_000,  feature: "u16-20_000"  ],
        [ type: u16,   count: 25_000,  feature: "u16-25_000"  ],
        [ type: u16,   count: 30_000,  feature: "u16-30_000"  ],
        [ type: u16,   count: 35_000,  feature: "u16-35_000"  ],
        [ type: u16,   count: 40_000,  feature: "u16-40_000"  ],
        [ type: u16,   count: 45_000,  feature: "u16-45_000"  ],
        [ type: u16,   count: 50_000,  feature: "u16-50_000"  ],
        [ type: u16,   count: 55_000,  feature: "u16-55_000"  ],
        [ type: u16,   count: 60_000,  feature: "u16-60_000"  ],
        [ type: u16,   count: 65_000,  feature: "u16-65_000"  ],
        [ type: u16,   count: 70_000,  feature: "u16-70_000"  ],
        [ type: u16,   count: 75_000,  feature: "u16-75_000"  ],
        [ type: u16,   count: 80_000,  feature: "u16-80_000"  ],
        [ type: u16,   count: 85_000,  feature: "u16-85_000"  ],
        [ type: u16,   count: 90_000,  feature: "u16-90_000"  ],
        [ type: u16,   count: 95_000,  feature: "u16-95_000"  ],

        [ type: u32,   count: 1_000,   feature: "u32-1_000"   ],
        [ type: u32,   count: 2_000,   feature: "u32-2_000"   ],
        [ type: u32,   count: 5_000,   feature: "u32-5_000"   ],
        [ type: u32,   count: 10_000,  feature: "u32-10_000"  ],
        [ type: u32,   count: 15_000,  feature: "u32-15_000"  ],
        [ type: u32,   count: 20_000,  feature: "u32-20_000"  ],
        [ type: u32,   count: 25_000,  feature: "u32-25_000"  ],
        [ type: u32,   count: 30_000,  feature: "u32-30_000"  ],
        [ type: u32,   count: 35_000,  feature: "u32-35_000"  ],
        [ type: u32,   count: 40_000,  feature: "u32-40_000"  ],
        [ type: u32,   count: 45_000,  feature: "u32-45_000"  ],
        [ type: u32,   count: 50_000,  feature: "u32-50_000"  ],
        [ type: u32,   count: 55_000,  feature: "u32-55_000"  ],
        [ type: u32,   count: 60_000,  feature: "u32-60_000"  ],
        [ type: u32,   count: 65_000,  feature: "u32-65_000"  ],
        [ type: u32,   count: 70_000,  feature: "u32-70_000"  ],
        [ type: u32,   count: 75_000,  feature: "u32-75_000"  ],
        [ type: u32,   count: 80_000,  feature: "u32-80_000"  ],
        [ type: u32,   count: 85_000,  feature: "u32-85_000"  ],
        [ type: u32,   count: 90_000,  feature: "u32-90_000"  ],
        [ type: u32,   count: 95_000,  feature: "u32-95_000"  ],

        [ type: u64,   count: 1_000,   feature: "u64-1_000"   ],
        [ type: u64,   count: 2_000,   feature: "u64-2_000"   ],
        [ type: u64,   count: 5_000,   feature: "u64-5_000"   ],
        [ type: u64,   count: 10_000,  feature: "u64-10_000"  ],
        [ type: u64,   count: 15_000,  feature: "u64-15_000"  ],
        [ type: u64,   count: 20_000,  feature: "u64-20_000"  ],
        [ type: u64,   count: 25_000,  feature: "u64-25_000"  ],
        [ type: u64,   count: 30_000,  feature: "u64-30_000"  ],
        [ type: u64,   count: 35_000,  feature: "u64-35_000"  ],
        [ type: u64,   count: 40_000,  feature: "u64-40_000"  ],
        [ type: u64,   count: 45_000,  feature: "u64-45_000"  ],
        [ type: u64,   count: 50_000,  feature: "u64-50_000"  ],
        [ type: u64,   count: 55_000,  feature: "u64-55_000"  ],
        [ type: u64,   count: 60_000,  feature: "u64-60_000"  ],
        [ type: u64,   count: 65_000,  feature: "u64-65_000"  ],
        [ type: u64,   count: 70_000,  feature: "u64-70_000"  ],
        [ type: u64,   count: 75_000,  feature: "u64-75_000"  ],
        [ type: u64,   count: 80_000,  feature: "u64-80_000"  ],
        [ type: u64,   count: 85_000,  feature: "u64-85_000"  ],
        [ type: u64,   count: 90_000,  feature: "u64-90_000"  ],
        [ type: u64,   count: 95_000,  feature: "u64-95_000"  ],

        [ type: u128,  count: 1_000,   feature: "u128-1_000"  ],
        [ type: u128,  count: 2_000,   feature: "u128-2_000"  ],
        [ type: u128,  count: 5_000,   feature: "u128-5_000"  ],
        [ type: u128,  count: 10_000,  feature: "u128-10_000" ],
        [ type: u128,  count: 15_000,  feature: "u128-15_000" ],
        [ type: u128,  count: 20_000,  feature: "u128-20_000" ],
        [ type: u128,  count: 25_000,  feature: "u128-25_000" ],
        [ type: u128,  count: 30_000,  feature: "u128-30_000" ],
        [ type: u128,  count: 35_000,  feature: "u128-35_000" ],
        [ type: u128,  count: 40_000,  feature: "u128-40_000" ],
        [ type: u128,  count: 45_000,  feature: "u128-45_000" ],
        [ type: u128,  count: 50_000,  feature: "u128-50_000" ],
        [ type: u128,  count: 55_000,  feature: "u128-55_000" ],
        [ type: u128,  count: 60_000,  feature: "u128-60_000" ],
        [ type: u128,  count: 65_000,  feature: "u128-65_000" ],
        [ type: u128,  count: 70_000,  feature: "u128-70_000" ],
        [ type: u128,  count: 75_000,  feature: "u128-75_000" ],
        [ type: u128,  count: 80_000,  feature: "u128-80_000" ],
        [ type: u128,  count: 85_000,  feature: "u128-85_000" ],
        [ type: u128,  count: 90_000,  feature: "u128-90_000" ],
        [ type: u128,  count: 95_000,  feature: "u128-95_000" ],
    );

    bench_cartesian_product!(
        [ count: 1_000,   feature: "i32-cartesian-1_000"  ],
        [ count: 2_000,   feature: "i32-cartesian-2_000"  ],
        [ count: 5_000,   feature: "i32-cartesian-5_000"  ],
        [ count: 10_000,  feature: "i32-cartesian-10_000" ],
        [ count: 15_000,  feature: "i32-cartesian-15_000" ],
        [ count: 20_000,  feature: "i32-cartesian-20_000" ],
        [ count: 25_000,  feature: "i32-cartesian-25_000" ],
        [ count: 30_000,  feature: "i32-cartesian-30_000" ],
        [ count: 35_000,  feature: "i32-cartesian-35_000" ],
        [ count: 40_000,  feature: "i32-cartesian-40_000" ],
        [ count: 45_000,  feature: "i32-cartesian-45_000" ],
        [ count: 50_000,  feature: "i32-cartesian-50_000" ],
        [ count: 55_000,  feature: "i32-cartesian-55_000" ],
        [ count: 60_000,  feature: "i32-cartesian-60_000" ],
        [ count: 65_000,  feature: "i32-cartesian-65_000" ],
        [ count: 70_000,  feature: "i32-cartesian-70_000" ],
        [ count: 75_000,  feature: "i32-cartesian-75_000" ],
        [ count: 80_000,  feature: "i32-cartesian-80_000" ],
        [ count: 85_000,  feature: "i32-cartesian-85_000" ],
        [ count: 90_000,  feature: "i32-cartesian-90_000" ],
        [ count: 95_000,  feature: "i32-cartesian-95_000" ],
    );

    println!("bench done!");
}
