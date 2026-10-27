use alloc::vec::Vec;

const fn bucket_bits(len: usize) -> u32 {
    len.saturating_sub(1).bit_width().clamp(1, 17)
}
const BUCKETS<const N: usize>: usize = const { 1 << bucket_bits(N) };

// sort + dedup
macro_rules! normalize {
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
                let mut i = first;
                if dense {
                    len = N;
                    diff = v ^ a[N - 1];
                    let top = (1 as $ty) << (<$ty>::BITS - 1);
                    i = 1;
                    while i < N && diff & top == 0 {
                        diff |= v ^ a[i];
                        i += 1;
                    }
                } else {
                    while i != N {
                        diff |= v ^ a[i];
                        len += 1;
                        i = next[i];
                    }
                }
                let bits = <$ty>::BITS - diff.leading_zeros();
                let mut digit = bucket_bits(len).min(bits);
                if digit < bits {
                    digit = digit.saturating_sub(1).max(1);
                }
                let shift = bits - digit;
                let buckets = 1usize << digit;
                let mask = buckets - 1;
                let mut heads = [N; BUCKETS::<N>];
                i = first;
                while i != N {
                    let following = if dense { i + 1 } else { next[i] };
                    let b = (((a[i] ^ <$ty>::MIN) >> shift) as usize) & mask;
                    next[i] = heads[b];
                    heads[b] = i;
                    i = following;
                }
                let mut b = 0;
                while b < buckets {
                    i = heads[b];
                    let start = dest;
                    if i != N {
                        out[dest] = a[i];
                        dest += 1;
                        i = if shift == 0 { N } else { next[i] };
                    }
                    while i != N {
                        let v = a[i];
                        let mut j = start;
                        while j < dest && out[j] < v {
                            j += 1;
                        }
                        if j == dest || out[j] != v {
                            if dest - start == 8 {
                                dest = sort(a, next, heads[b], out, start, false);
                                break;
                            }
                            let mut k = dest;
                            while k > j {
                                out[k] = out[k - 1];
                                k -= 1;
                            }
                            out[j] = v;
                            dest += 1;
                        }
                        i = next[i];
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

normalize!(normalize_u32, u32);
normalize!(normalize_u64, u64);
normalize!(normalize_u128, u128);
normalize!(normalize_usize, usize);
normalize!(normalize_i32, i32);
normalize!(normalize_i64, i64);
normalize!(normalize_i128, i128);
normalize!(normalize_isize, isize);
