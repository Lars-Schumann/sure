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

macro_rules! bench {
    ($([type: $type:ident, count: $count:literal, feature: $feature:literal]),+ $(,)?) => {$(
        #[cfg(feature = $feature)]
        {
            let a: Sure<$type, { YOINK::<$type, $count> }> = Sure::new(0).unwrap();
            let b: Sure<$type, _> = a.normalize();
            assert_eq!(b.inner(), 0);
        }
    )+};
}

fn main() {
    #[allow(unused)]
    use sure::base::Sure;

    bench!(
        [type: u8,      count: 1_000,   feature: "u8-1_000"   ],
        [type: u8,      count: 2_000,   feature: "u8-2_000"   ],
        [type: u8,      count: 5_000,   feature: "u8-5_000"   ],
        [type: u8,      count: 10_000,  feature: "u8-10_000"  ],
        [type: u8,      count: 15_000,  feature: "u8-15_000"  ],
        [type: u8,      count: 20_000,  feature: "u8-20_000"  ],
        [type: u8,      count: 25_000,  feature: "u8-25_000"  ],
        [type: u8,      count: 30_000,  feature: "u8-30_000"  ],
        [type: u8,      count: 35_000,  feature: "u8-35_000"  ],
        [type: u8,      count: 40_000,  feature: "u8-40_000"  ],
        [type: u8,      count: 45_000,  feature: "u8-45_000"  ],
        [type: u8,      count: 50_000,  feature: "u8-50_000"  ],
        [type: u8,      count: 55_000,  feature: "u8-55_000"  ],
        [type: u8,      count: 60_000,  feature: "u8-60_000"  ],
        [type: u8,      count: 65_000,  feature: "u8-65_000"  ],
        [type: u8,      count: 70_000,  feature: "u8-70_000"  ],
        [type: u8,      count: 75_000,  feature: "u8-75_000"  ],
        [type: u8,      count: 80_000,  feature: "u8-80_000"  ],
        [type: u8,      count: 85_000,  feature: "u8-85_000"  ],
        [type: u8,      count: 90_000,  feature: "u8-90_000"  ],
        [type: u8,      count: 95_000,  feature: "u8-95_000"  ],

        [type: u16,     count: 1_000,   feature: "u16-1_000"  ],
        [type: u16,     count: 2_000,   feature: "u16-2_000"  ],
        [type: u16,     count: 5_000,   feature: "u16-5_000"  ],
        [type: u16,     count: 10_000,  feature: "u16-10_000" ],
        [type: u16,     count: 15_000,  feature: "u16-15_000" ],
        [type: u16,     count: 20_000,  feature: "u16-20_000" ],
        [type: u16,     count: 25_000,  feature: "u16-25_000" ],
        [type: u16,     count: 30_000,  feature: "u16-30_000" ],
        [type: u16,     count: 35_000,  feature: "u16-35_000" ],
        [type: u16,     count: 40_000,  feature: "u16-40_000" ],
        [type: u16,     count: 45_000,  feature: "u16-45_000" ],
        [type: u16,     count: 50_000,  feature: "u16-50_000" ],
        [type: u16,     count: 55_000,  feature: "u16-55_000" ],
        [type: u16,     count: 60_000,  feature: "u16-60_000" ],
        [type: u16,     count: 65_000,  feature: "u16-65_000" ],
        [type: u16,     count: 70_000,  feature: "u16-70_000" ],
        [type: u16,     count: 75_000,  feature: "u16-75_000" ],
        [type: u16,     count: 80_000,  feature: "u16-80_000" ],
        [type: u16,     count: 85_000,  feature: "u16-85_000" ],
        [type: u16,     count: 90_000,  feature: "u16-90_000" ],
        [type: u16,     count: 95_000,  feature: "u16-95_000" ],

        [type: u32,     count: 1_000,   feature: "u32-1_000"  ],
        [type: u32,     count: 2_000,   feature: "u32-2_000"  ],
     // [type: u32,     count: 5_000,   feature: "u32-5_000"  ],
     // [type: u32,     count: 10_000,  feature: "u32-10_000" ],
     // [type: u32,     count: 15_000,  feature: "u32-15_000" ],
     // [type: u32,     count: 20_000,  feature: "u32-20_000" ],
     // [type: u32,     count: 25_000,  feature: "u32-25_000" ],
     // [type: u32,     count: 30_000,  feature: "u32-30_000" ],
     // [type: u32,     count: 35_000,  feature: "u32-35_000" ],
     // [type: u32,     count: 40_000,  feature: "u32-40_000" ],
     // [type: u32,     count: 45_000,  feature: "u32-45_000" ],
     // [type: u32,     count: 50_000,  feature: "u32-50_000" ],
     // [type: u32,     count: 55_000,  feature: "u32-55_000" ],
     // [type: u32,     count: 60_000,  feature: "u32-60_000" ],
     // [type: u32,     count: 65_000,  feature: "u32-65_000" ],
     // [type: u32,     count: 70_000,  feature: "u32-70_000" ],
     // [type: u32,     count: 75_000,  feature: "u32-75_000" ],
     // [type: u32,     count: 80_000,  feature: "u32-80_000" ],
     // [type: u32,     count: 85_000,  feature: "u32-85_000" ],
     // [type: u32,     count: 90_000,  feature: "u32-90_000" ],
     // [type: u32,     count: 95_000,  feature: "u32-95_000" ],
    );

    println!("bench done!");
}
