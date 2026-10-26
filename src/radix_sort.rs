const fn radix_bits(len: usize) -> u32 {
    len.saturating_sub(1).bit_width().saturating_sub(2).min(14)
}

const BUCKETS<const LEN: usize>: usize = const { 1 << radix_bits(LEN) };

macro_rules! insert {
    ($array:ident, $start:ident, $i:ident, $value:ident) => {{
        let mut j = $i;
        while j > $start {
            let previous = $array[j - 1];
            if previous <= $value {
                break;
            }
            $array[j] = previous;
            j -= 1;
        }
        $array[j] = $value;
    }};
}

macro_rules! radix_sort {
    ($name:ident, $ty:ty) => {
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
                let mut i = start + 1;
                while i < end && array[i - 1] <= array[i] {
                    i += 1;
                }
                if i == end {
                    return;
                }
                if len <= 24 {
                    let mut i = start + 1;
                    while i < end {
                        let value = array[i];
                        insert!(array, start, i, value);
                        i += 1;
                    }

                    return;
                }
                let first = array[start];
                let mut sampled: $ty = 0;
                let mut i = start + len / 8;
                while i < end {
                    sampled |= first ^ array[i];
                    i += len / 8;
                }
                let mut bits = (<$ty>::BITS - sampled.leading_zeros()).max(1);
                loop {
                    let digit_bits = bits.min(radix_bits(len));
                    let shift = bits - digit_bits;
                    let bucket_count = 1usize << digit_bits;
                    let mask = bucket_count - 1;
                    let mut heads = [N; BUCKETS::<N>];
                    let mut next = [N; N];
                    let mut differences: $ty = 0;
                    let mut i = start;
                    while i < end {
                        let value = array[i];
                        differences |= first ^ value;
                        let bucket = ((value ^ <$ty>::MIN) >> shift) as usize & mask;
                        next[i] = heads[bucket];
                        heads[bucket] = i;
                        i += 1;
                    }
                    let actual_bits = <$ty>::BITS - differences.leading_zeros();
                    if actual_bits > bits {
                        bits = actual_bits;
                        continue;
                    }
                    let mut i = start;
                    let mut bucket = 0;
                    while bucket < bucket_count {
                        let bucket_start = i;
                        let mut index = heads[bucket];
                        while index != N {
                            let value = array[index];
                            if shift != 0 && i - bucket_start < 24 {
                                insert!(scratch, bucket_start, i, value);
                            } else {
                                scratch[i] = value;
                            }
                            index = next[index];
                            i += 1;
                        }
                        heads[bucket] = i;
                        bucket += 1;
                    }
                    array[start..end].copy_from_slice(&scratch[start..end]);
                    if shift == 0 {
                        return;
                    }
                    let mut start = start;
                    let mut bucket = 0;
                    while bucket < bucket_count {
                        let end = heads[bucket];
                        if end - start > 24 {
                            sort(array, scratch, start, end);
                        }
                        start = end;
                        bucket += 1;
                    }
                    return;
                }
            }
            if N < 2 {
                return array;
            }
            let mut scratch = array;
            sort(&mut array, &mut scratch, 0, N);
            array
        }
    };
}

radix_sort!(radix_sort_u32, u32);
radix_sort!(radix_sort_u64, u64);
radix_sort!(radix_sort_u128, u128);
radix_sort!(radix_sort_usize, usize);
radix_sort!(radix_sort_i8, i8);
radix_sort!(radix_sort_i16, i16);
radix_sort!(radix_sort_i32, i32);
radix_sort!(radix_sort_i64, i64);
radix_sort!(radix_sort_i128, i128);
radix_sort!(radix_sort_isize, isize);
