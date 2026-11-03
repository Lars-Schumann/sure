use alloc::vec::Vec;

const fn bucket_bits(len: usize) -> u32 {
    len.saturating_sub(1).bit_width().clamp(1, 17)
}
const BUCKETS<const N: usize>: usize = const { 1 << bucket_bits(N) };

macro_rules! normalize {
    ($name:ident, $ty:ty) => {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::manual_bit_width,
            clippy::too_many_lines
        )]
        pub(crate) const fn $name<const N: usize>(a: [$ty; N]) -> Vec<$ty> {
            const fn sort<const N: usize, const B: usize>(
                a: &[$ty; N],
                next: &mut [usize; N],
                first: usize,
                out: &mut [$ty; N],
                mut dest: usize,
                dense: bool,
            ) -> usize {
                let v = a[first];
                let (mut diff, mut len, mut i) = (0, 0, first);
                if dense {
                    let top = (1 as $ty) << (<$ty>::BITS - 1);
                    (len, diff, i) = (N, v ^ a[N - 1], 1);
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
                let mut digit = bucket_bits(len).min(bits).min(B.trailing_zeros());
                if digit < bits {
                    digit = digit.saturating_sub(1).max(1);
                }
                let shift = bits - digit;
                let buckets = 1usize << digit;
                let mask = buckets - 1;
                let sign = ((<$ty>::MIN >> shift) as usize) & mask;
                let mut heads = [N; B];
                if shift == 0 {
                    if dense {
                        i = 0;
                        while i < N {
                            heads[(a[i] as usize) & mask] = i;
                            i += 1;
                        }
                    } else {
                        i = first;
                        while i != N {
                            heads[(a[i] as usize) & mask] = i;
                            i = next[i];
                        }
                    }
                    let mut b = 0;
                    while b < buckets {
                        i = heads[b ^ sign];
                        if i != N {
                            out[dest] = a[i];
                            dest += 1;
                        }
                        b += 1;
                    }
                    return dest;
                }
                if dense {
                    i = 0;
                    while i < N {
                        let h = &mut heads[((a[i] >> shift) as usize) & mask];
                        next[i] = *h;
                        *h = i;
                        i += 1;
                    }
                } else {
                    i = first;
                    while i != N {
                        let h = &mut heads[((a[i] >> shift) as usize) & mask];
                        let following = next[i];
                        next[i] = *h;
                        *h = i;
                        i = following;
                    }
                }
                let mut b = 0;
                while b < buckets {
                    i = heads[b ^ sign];
                    b += 1;
                    if i == N {
                        continue;
                    }
                    let head = i;
                    let mut lo = a[i];
                    let second = next[i];
                    if second == N {
                        out[dest] = lo;
                        dest += 1;
                        continue;
                    }
                    let mut hi = a[second];
                    if lo > hi {
                        (lo, hi) = (hi, lo);
                    }
                    let start = dest;
                    i = next[second];
                    if i == N {
                        out[dest] = lo;
                        dest += 1;
                        if hi != lo {
                            out[dest] = hi;
                            dest += 1;
                        }
                        continue;
                    }
                    let mut mid = a[i];
                    if mid < lo {
                        (lo, mid) = (mid, lo);
                    } else if mid > hi {
                        (hi, mid) = (mid, hi);
                    }
                    out[dest] = lo;
                    dest += 1;
                    if mid != lo {
                        out[dest] = mid;
                        dest += 1;
                    }
                    if hi != mid {
                        out[dest] = hi;
                        dest += 1;
                    }
                    i = next[i];
                    if lo == hi {
                        while i != N && a[i] == lo {
                            i = next[i];
                        }
                    }
                    while i != N {
                        let v = a[i];
                        let mut j = start;
                        while j < dest && out[j] < v {
                            j += 1;
                        }
                        if j == dest || out[j] != v {
                            if dest - start == 8 {
                                dest = sort::<N, 4096>(a, next, head, out, start, false);
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
                }
                dest
            }
            let mut out = a;
            let (mut len, mut i) = (0, 0);
            if N != 0 {
                let mut previous = a[0];
                (len, i) = (1, 1);
                while i < N {
                    let v = a[i];
                    if v < previous {
                        break;
                    }
                    if v != previous {
                        out[len] = v;
                        len += 1;
                        previous = v;
                    }
                    i += 1;
                }
            }
            if i != N {
                len = sort::<N, { BUCKETS::<N> }>(&a, &mut [N; N], 0, &mut out, 0, true);
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
