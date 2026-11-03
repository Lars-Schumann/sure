use alloc::vec::Vec;

const fn bucket_bits(len: usize) -> u32 {
    len.saturating_sub(1).bit_width().clamp(3, 17)
}
const BUCKETS<const N: usize>: usize = const { 1 << bucket_bits(N) };

macro_rules! normalize {
    ($name:ident, $ty:ty, $uty:ty) => {
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
                head: usize,
                out: &mut [$ty; N],
                mut dest: usize,
            ) -> usize {
                const TOP: $ty = 1 << (<$ty>::BITS - 1);
                let dense = head == N;
                let first = a[if dense { 0 } else { head }];
                let (mut diff, mut i, mut s): (_, _, &[$ty]) = (0, head, a);
                let mut len = N;
                if dense {
                    while let &[v0, v1, v2, v3, v4, v5, v6, v7, ref rest @ ..] = s
                        && diff & TOP == 0
                    {
                        s = rest;
                        diff |= v0 ^ v1
                            | v1 ^ v2
                            | v2 ^ v3
                            | v3 ^ v4
                            | v4 ^ v5
                            | v5 ^ v6
                            | v6 ^ v7
                            | v7 ^ first;
                    }
                    while let &[v, ref rest @ ..] = s
                        && diff & TOP == 0
                    {
                        s = rest;
                        diff |= v ^ first;
                    }
                } else {
                    len = 0;
                    while i != N {
                        diff |= a[i] ^ first;
                        i = next[i];
                        len += 1;
                    }
                }
                if diff == 0 {
                    out[dest] = first;
                    return dest + 1;
                }
                let bits = <$ty>::BITS - diff.leading_zeros();
                let log_b = const { B.ilog2() };
                let digit = bits.min(bucket_bits(len)).min(log_b);
                let shift = bits - digit;
                let key = <$ty>::MIN ^ ((first ^ <$ty>::MIN) >> (bits - 1) >> 1 << (bits - 1) << 1);
                let mut heads = [N; B];
                macro_rules! push {
                    ($v:expr, $i:expr) => {
                        let h = &mut heads[(($v ^ key) as $uty >> shift) as usize];
                        if *h != N && shift != 0 {
                            next[$i] = *h;
                        }
                        *h = $i;
                    };
                }
                if dense {
                    s = a;
                    i = 0;
                    while let &[v0, v1, v2, v3, v4, v5, v6, v7, ref rest @ ..] = s {
                        s = rest;
                        push!(v0, i);
                        push!(v1, i | 1);
                        push!(v2, i | 2);
                        push!(v3, i | 3);
                        push!(v4, i | 4);
                        push!(v5, i | 5);
                        push!(v6, i | 6);
                        push!(v7, i | 7);
                        i += 8;
                    }
                    while let &[v, ref rest @ ..] = s {
                        s = rest;
                        push!(v, i);
                        i += 1;
                    }
                } else {
                    i = head;
                    while i != N {
                        let n = next[i];
                        next[i] = N;
                        push!(a[i], i);
                        i = n;
                    }
                }
                macro_rules! bucket {
                    ($h:expr) => {
                        if $h != N {
                            let mut last = a[$h];
                            out[dest] = last;
                            dest += 1;
                            i = next[$h];
                            if i == N {
                            } else if next[i] == N {
                                let v = a[i];
                                if v > last {
                                    out[dest] = v;
                                    dest += 1;
                                } else if v < last {
                                    out[dest - 1] = v;
                                    out[dest] = last;
                                    dest += 1;
                                }
                            } else {
                                let start = dest - 1;
                                while i != N {
                                    let v = a[i];
                                    i = next[i];
                                    if v > last {
                                        out[dest] = v;
                                        dest += 1;
                                        last = v;
                                        continue;
                                    }
                                    let mut j = dest - 1;
                                    while j > start && v < out[j - 1] {
                                        j -= 1;
                                    }
                                    if v == out[j] || (j > start && v == out[j - 1]) {
                                        continue;
                                    }
                                    if dest - start >= 8 {
                                        dest = sort::<N, 4096>(a, next, $h, out, start);
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
                            }
                        }
                    };
                }
                let mut hs: &[usize] = &heads;
                if digit < log_b {
                    hs = heads.split_at(1 << if digit < 3 { 3 } else { digit }).0;
                }
                while let &[h0, h1, h2, h3, h4, h5, h6, h7, ref rest @ ..] = hs {
                    hs = rest;
                    bucket!(h0);
                    bucket!(h1);
                    bucket!(h2);
                    bucket!(h3);
                    bucket!(h4);
                    bucket!(h5);
                    bucket!(h6);
                    bucket!(h7);
                }
                dest
            }

            let mut out = a;
            let (mut len, mut i) = (0, 0);
            if N != 0 {
                (len, i) = (1, 1);
                while i < N && a[i] >= out[len - 1] {
                    if a[i] != out[len - 1] {
                        out[len] = a[i];
                        len += 1;
                    }
                    i += 1;
                }
            }
            if i != N {
                len = sort::<N, { BUCKETS::<N> }>(&a, &mut [N; N], N, &mut out, 0);
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

normalize!(normalize_u32, u32, u32);
normalize!(normalize_u64, u64, u64);
normalize!(normalize_u128, u128, u128);
normalize!(normalize_usize, usize, usize);
normalize!(normalize_i32, i32, u32);
normalize!(normalize_i64, i64, u64);
normalize!(normalize_i128, i128, u128);
normalize!(normalize_isize, isize, usize);
