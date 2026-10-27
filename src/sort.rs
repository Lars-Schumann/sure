use alloc::vec::Vec;

const fn bucket_bits(len: usize) -> u32 {
    len.saturating_sub(1).bit_width().clamp(1, 17)
}
const BUCKETS<const N: usize>: usize = const { 1 << bucket_bits(N) };

macro_rules! sort {
    ($name:ident, $ty:ty) => {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::manual_bit_width,
            clippy::too_many_lines
        )]
        pub(crate) const fn $name<const N: usize>(a: [$ty; N]) -> Vec<$ty> {
            const fn sort<const N: usize>(
                a: &[$ty; N],
                next: &mut [usize; N],
                first: usize,
                out: &mut [$ty; N],
                mut dest: usize,
                dense: bool,
            ) -> usize {
                let v = a[first];
                let mut diff = 0;
                let mut len = 0;
                if dense {
                    len = N;
                    diff = v ^ a[N - 1];
                    let top = (1 as $ty) << (<$ty>::BITS - 1);
                    let mut i = 1;
                    while i < N && diff & top == 0 {
                        diff |= v ^ a[i];
                        i += 1;
                    }
                } else {
                    let mut i = first;
                    while i != N {
                        diff |= v ^ a[i];
                        len += 1;
                        i = next[i];
                    }
                }
                if diff == 0 {
                    out[dest] = v;
                    return dest + 1;
                }
                if len <= 8 {
                    let start = dest;
                    let mut i = first;
                    while i != N {
                        let v = a[i];
                        let mut j = start;
                        while j < dest && out[j] < v {
                            j += 1;
                        }
                        if j == dest || out[j] != v {
                            let mut k = dest;
                            while k > j {
                                out[k] = out[k - 1];
                                k -= 1;
                            }
                            out[j] = v;
                            dest += 1;
                        }
                        i = if dense { i + 1 } else { next[i] };
                    }
                    return dest;
                }
                let bits = <$ty>::BITS - diff.leading_zeros();
                let digit = bits.min(bucket_bits(len));
                let shift = bits - digit;
                let buckets = 1usize << digit;
                let mask = buckets - 1;
                let mut heads = [N; BUCKETS::<N>];
                let mut i = first;
                while i != N {
                    let following = if dense { i + 1 } else { next[i] };
                    let b = (((a[i] ^ <$ty>::MIN) >> shift) as usize) & mask;
                    next[i] = heads[b];
                    heads[b] = i;
                    i = following;
                }
                let mut b = 0;
                while b < buckets {
                    let head = heads[b];
                    if head != N {
                        if shift == 0 || next[head] == N {
                            out[dest] = a[head];
                            dest += 1;
                        } else {
                            dest = sort(a, next, head, out, dest, false);
                        }
                    }
                    b += 1;
                }
                dest
            }
            let mut out = a;
            let mut len = 0;
            let mut i = 0;
            while i < N && (i == 0 || a[i - 1] <= a[i]) {
                if i == 0 || a[i - 1] != a[i] {
                    out[len] = a[i];
                    len += 1;
                }
                i += 1;
            }
            if i != N {
                len = sort(&a, &mut [N; N], 0, &mut out, 0, true);
            }
            let mut result: Vec<$ty> = Vec::with_capacity(len);
            // SAFETY: yes.
            unsafe {
                core::ptr::copy_nonoverlapping(out.as_ptr(), result.as_mut_ptr(), len);
                result.set_len(len);
            }
            result
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
