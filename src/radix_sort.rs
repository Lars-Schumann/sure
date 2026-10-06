const fn radix_bits(len: usize) -> u32 {
    len.saturating_sub(1).bit_width().saturating_sub(2).min(14)
}

const BUCKETS<const LEN: usize>: usize = const { 1 << radix_bits(LEN) };

macro_rules! radix_sort {
    ($name:ident, $ty:ty, $unsigned:ty, $sign_bit:expr) => {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss,
            clippy::manual_bit_width
        )]
        pub(crate) const fn $name<const N: usize>(mut array: [$ty; N]) -> [$ty; N] {
            const fn sort<const N: usize>(
                array: &mut [$ty; N],
                scratch: &mut [$ty; N],
                start: usize,
                end: usize,
            ) {
                let len = end - start;
                if len <= 24 {
                    let mut i = start + 1;
                    while i < end {
                        let value = array[i];
                        let mut j = i;
                        while j > start && array[j - 1] > value {
                            array[j] = array[j - 1];
                            j -= 1;
                        }
                        array[j] = value;
                        i += 1;
                    }

                    return;
                }
                let mut differences: $unsigned = 0;
                let first = array[start] as $unsigned;
                let mut i = start + 1;
                while i < end {
                    differences |= first ^ array[i] as $unsigned;
                    i += 1;
                }
                if differences == 0 {
                    return;
                }
                let significant_bits = differences.bit_width();
                let digit_bits = significant_bits.min(radix_bits(len));
                let shift = significant_bits - digit_bits;
                let bucket_count = 1usize << digit_bits;
                let mask = (bucket_count - 1) as $unsigned;
                let mut counts = [0usize; BUCKETS::<N>];
                let mut i = start;
                while i < end {
                    let bucket = (((array[i] as $unsigned ^ $sign_bit) >> shift) & mask) as usize;
                    counts[bucket] += 1;
                    i += 1;
                }
                if shift == 0 {
                    let prefix = (first ^ $sign_bit) & !mask;
                    let mut i = start;
                    let mut bucket = 0;
                    while bucket < bucket_count {
                        let value = ((prefix | bucket as $unsigned) ^ $sign_bit) as $ty;
                        let end = i + counts[bucket];
                        while i < end {
                            array[i] = value;
                            i += 1;
                        }
                        bucket += 1;
                    }
                    return;
                }
                let mut total = start;
                let mut bucket = 0;
                while bucket < bucket_count {
                    let count = counts[bucket];
                    counts[bucket] = total;
                    total += count;
                    bucket += 1;
                }
                let mut i = start;
                while i < end {
                    let value = array[i];
                    let bucket = (((value as $unsigned ^ $sign_bit) >> shift) & mask) as usize;
                    let offset = &mut counts[bucket];
                    scratch[*offset] = value;
                    *offset += 1;
                    i += 1;
                }
                array[start..end].copy_from_slice(&scratch[start..end]);
                let mut start = start;
                let mut bucket = 0;
                while bucket < bucket_count {
                    let end = counts[bucket];
                    if end - start > 1 {
                        sort(array, scratch, start, end);
                    }
                    start = end;
                    bucket += 1;
                }
            }
            if N < 2 {
                return array;
            }
            let mut i = 1;
            while i < N && array[i - 1] <= array[i] {
                i += 1;
            }
            if i == N {
                return array;
            }
            let mut scratch = array;
            sort(&mut array, &mut scratch, 0, N);
            array
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
