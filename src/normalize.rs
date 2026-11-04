use alloc::vec::Vec;

const BUCKETS<const N: usize>: usize = const { 1 << N.bit_width() };

macro_rules! each {
    ($slice:expr, $cond:expr, $body:ident $(, $i:ident)?) => {
        let mut s: &[_] = $slice;
        each!(@ s, $cond, $body, 8 $($i)?, v0 0, v1 1, v2 2, v3 3, v4 4, v5 5, v6 6, v7 7);
        each!(@ s, $cond, $body, 1 $($i)?, v0 0);
    };
    (@ $s:ident, $cond:expr, $body:ident, $n:literal $($i:ident)?, $($v:ident $k:literal),+) => {
        while let &[$($v,)+ ref rest @ ..] = $s
            && $cond
        {
            $s = rest;
            $($body!($v, $k);)+
            $($i += $n;)?
        }
    };
}

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
                a: &[$ty],
                out: &mut [$ty; N],
                mut dest: usize,
            ) -> usize {
                let first = a[0];
                let mut diff = 0;
                macro_rules! mix {
                    ($v:expr, $k:expr) => {
                        diff |= $v ^ first;
                    };
                }
                each!(a, diff & const { 1 << (<$ty>::BITS - 1) } == 0, mix);

                let bits = <$ty>::BITS - diff.leading_zeros();
                let width = bits.min(a.len().bit_width()).min(const { B.ilog2() });
                let shift = bits - width;
                let key = <$ty>::MIN ^ ((first ^ <$ty>::MIN) >> (bits - 1) >> 1 << (bits - 1) << 1);

                let mut heads = [N; B];
                let mut next = [N; N];
                let mut i = 0;
                macro_rules! push {
                    ($v:expr, $k:expr) => {
                        let h = &mut heads[(($v ^ key) as $uty >> shift) as usize];
                        if *h != N && shift != 0 {
                            next[i | $k] = *h;
                        }
                        *h = i | $k;
                    };
                }
                each!(a, true, push, i);

                macro_rules! bucket {
                    ($h:expr, $k:expr) => {
                        if $h != N {
                            let start = dest;
                            let mut last = a[$h];
                            out[dest] = last;
                            dest += 1;
                            i = next[$h];
                            'list: while i != N {
                                let mut v = a[i];
                                i = next[i];
                                if v <= last {
                                    if v == last {
                                        continue;
                                    }
                                    if dest - start >= 16 {
                                        let mut bucket = *out;
                                        let mut len = 0;
                                        i = $h;
                                        while i != N {
                                            bucket[len] = a[i];
                                            len += 1;
                                            i = next[i];
                                        }
                                        dest = sort::<N, 4096>(bucket.split_at(len).0, out, start);
                                        break;
                                    }
                                    let mut j = start;
                                    while j != dest {
                                        let o = out[j];
                                        if v <= o {
                                            if v == o {
                                                continue 'list;
                                            }
                                            out[j] = v;
                                            v = o;
                                        }
                                        j += 1;
                                    }
                                }
                                out[dest] = v;
                                last = v;
                                dest += 1;
                            }
                        }
                    };
                }
                // SAFETY: yes.
                let used = unsafe { core::slice::from_raw_parts(heads.as_ptr(), 1 << width) };
                each!(used, true, bucket);
                dest
            }

            let mut out = a;
            let &[mut last, ref tail @ ..] = a.as_slice() else {
                return Vec::new();
            };
            let mut s = tail;
            let mut len = 1;
            while let &[v, ref rest @ ..] = s
                && v >= last
            {
                s = rest;
                if v != last {
                    out[len] = v;
                    len += 1;
                    last = v;
                }
            }
            if !s.is_empty() {
                len = sort::<N, { BUCKETS::<N> }>(&a, &mut out, 0);
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
