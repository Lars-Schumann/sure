const fn bucket_bits(len: usize) -> u32 {
    len.saturating_sub(1)
        .bit_width()
        .saturating_sub(1)
        .clamp(1, 17)
}

const BUCKETS<const N: usize>: usize = const { 1 << bucket_bits(N) };

macro_rules! sort {
    ($name:ident, $ty:ty) => {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss,
            clippy::manual_bit_width,
            clippy::too_many_lines
        )]
        pub(crate) const fn $name<const N: usize>(array: [$ty; N]) -> [$ty; N] {
            const fn sort<const N: usize>(
                array: &[$ty; N],
                next: &mut [usize; N],
                out: &mut [$ty; N],
                mut dest: usize,
                first: usize,
                dense: bool,
            ) -> usize {
                let mut len = 0;
                let mut differences: $ty = 0;
                let first_value = array[first];
                if dense {
                    len = N;
                    let mut i = 1;
                    let top = (1 as $ty) << (<$ty>::BITS - 1);
                    differences = first_value ^ array[N - 1];
                    while i < N && differences & top == 0 {
                        differences |= first_value ^ array[i];
                        i += 1;
                    }
                } else {
                    let mut i = first;
                    while i != N {
                        differences |= first_value ^ array[i];
                        len += 1;
                        i = next[i];
                    }
                }
                let bits = <$ty>::BITS - differences.leading_zeros();
                let digit_bits = bits.min(bucket_bits(len));
                let shift = bits - digit_bits;
                let buckets = 1usize << digit_bits;
                let mask = buckets - 1;
                let mut heads = [if shift == 0 { 0 } else { N }; BUCKETS::<N>];
                if shift == 0 {
                    let mut i = first;
                    while i != N {
                        heads[(array[i] ^ <$ty>::MIN) as usize & mask] += 1;
                        i = if dense { i + 1 } else { next[i] };
                    }
                    let prefix = (first_value ^ <$ty>::MIN) & !(mask as $ty);
                    let mut b = 0;
                    while b < buckets {
                        let value = (prefix | b as $ty) ^ <$ty>::MIN;
                        let end = dest + heads[b];
                        while dest < end {
                            out[dest] = value;
                            dest += 1;
                        }
                        b += 1;
                    }
                    return dest;
                }
                let mut unsorted = [false; BUCKETS::<N>];
                let mut i = first;
                while i != N {
                    let v = array[i];
                    let following = if dense { i + 1 } else { next[i] };
                    let b = ((v ^ <$ty>::MIN) >> shift) as usize & mask;
                    let head = heads[b];
                    if head == N || v <= array[head] || unsorted[b] {
                        next[i] = head;
                        heads[b] = i;
                    } else {
                        let mut p = head;
                        let mut j = next[p];
                        let mut remaining = 8;
                        while j != N && array[j] < v {
                            if remaining == 0 {
                                unsorted[b] = true;
                                break;
                            }
                            p = j;
                            j = next[p];
                            remaining -= 1;
                        }
                        next[i] = j;
                        next[p] = i;
                    }
                    i = following;
                }
                let mut b = 0;
                while b < buckets {
                    let mut j = heads[b];
                    if unsorted[b] {
                        dest = sort(array, next, out, dest, j, false);
                    } else {
                        while j != N {
                            out[dest] = array[j];
                            dest += 1;
                            j = next[j];
                        }
                    }
                    b += 1;
                }
                dest
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
            let mut next = [N; N];
            let mut out = array;
            sort(&array, &mut next, &mut out, 0, 0, true);
            out
        }
    };
}

sort!(sort_u32, u32);
sort!(sort_u64, u64);
sort!(sort_u128, u128);
sort!(sort_usize, usize);
sort!(sort_i32, i32);
sort!(sort_i64, i64);
sort!(sort_i128, i128);
sort!(sort_isize, isize);
