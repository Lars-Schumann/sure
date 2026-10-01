macro_rules! radix_sort {
    ($name:ident, $ty:ty, $unsigned:ty, $sign_bit:expr) => {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        pub(crate) const fn $name<const N: usize>(mut array: [$ty; N]) -> [$ty; N] {
            if N < 2 {
                return array;
            }

            let mut differences: $unsigned = 0;
            let first = array[0] as $unsigned;
            let mut i = 1;
            while i < N {
                differences |= first ^ array[i] as $unsigned;
                i += 1;
            }
            if differences == 0 {
                return array;
            }

            let mut scratch = array;
            let mut source = &mut array;
            let mut output = &mut scratch;
            let mut in_scratch = false;
            let mut shift = 0;
            while differences != 0 {
                if differences & 0xff != 0 {
                    let mut counts = [0usize; 256];
                    let mut i = 0;
                    while i < N {
                        let bucket =
                            (((source[i] as $unsigned ^ $sign_bit) >> shift) & 0xff) as usize;
                        counts[bucket] += 1;
                        i += 1;
                    }

                    let mut total = 0;
                    let mut bucket = 0;
                    while bucket < counts.len() {
                        let count = counts[bucket];
                        counts[bucket] = total;
                        total += count;
                        bucket += 1;
                    }

                    let mut i = 0;
                    while i < N {
                        let value = source[i];
                        let bucket = (((value as $unsigned ^ $sign_bit) >> shift) & 0xff) as usize;
                        output[counts[bucket]] = value;
                        counts[bucket] += 1;
                        i += 1;
                    }
                    (source, output) = (output, source);
                    in_scratch = !in_scratch;
                }
                differences = (differences as u128 >> 8) as $unsigned;
                shift += 8;
            }

            if in_scratch { scratch } else { array }
        }
    };
}

radix_sort!(radix_sort_u8, u8, u8, 0);
radix_sort!(radix_sort_u16, u16, u16, 0);
radix_sort!(radix_sort_u32, u32, u32, 0);
radix_sort!(radix_sort_u64, u64, u64, 0);
radix_sort!(radix_sort_u128, u128, u128, 0);
radix_sort!(radix_sort_usize, usize, usize, 0);
radix_sort!(radix_sort_i8, i8, u8, 1 << 7);
radix_sort!(radix_sort_i16, i16, u16, 1 << 15);
radix_sort!(radix_sort_i32, i32, u32, 1 << 31);
radix_sort!(radix_sort_i64, i64, u64, 1 << 63);
radix_sort!(radix_sort_i128, i128, u128, 1 << 127);
radix_sort!(radix_sort_isize, isize, usize, 1 << (usize::BITS - 1));
