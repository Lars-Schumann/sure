use alloc::vec::Vec;

const BUCKETS<const N: usize>: usize = const { 1 << N.bit_width() };

macro_rules! each {
    ($slice:expr $(, $cond:expr)? $(; $i:ident)? => $($body:tt)+) => {{
        let mut s = $slice;
        macro_rules! body { $($body)+ }
        each!(@ s, 16 $($i)? $(, $cond)? => v0 0, v1 1, v2 2, v3 3, v4 4, v5 5, v6 6, v7 7, v8 8, v9 9, v10 10, v11 11, v12 12, v13 13, v14 14, v15 15);
        each!(@ s, 1 $($i)? $(, $cond)? => v0 0);
    }};
    (@ $s:ident, $n:literal $($i:ident)? $(, $cond:expr)? => $($v:ident $k:literal),+) => {
        while let &[$($v,)+ ref rest @ ..] = $s $(&& $cond)? {
            $s = rest;
            $(body!($v, $k);)+
            $($i += $n;)?
        }
    };
}

macro_rules! normalize {
    ($name:ident, $ty:ty, $uty:ty) => {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::too_many_lines)]
        pub(crate) const fn $name<const N: usize>(a: [$ty; N]) -> Vec<$ty> {
            const fn sort<const N: usize, const B: usize>(
                a: &[$ty],
                out: &mut [$ty; N],
                mut dest: usize,
            ) -> usize {
                let first = a[0];
                let mut diff = 0;
                each!(a, diff & const { 1 << (<$ty>::BITS - 1) } == 0 => ($v:expr, $k:expr) => {
                    diff |= $v ^ first;
                });
                let bits = (diff as $uty).bit_width();
                let shift = bits.saturating_sub(a.len().bit_width());
                let key = <$ty>::MIN ^ ((first ^ <$ty>::MIN) & (!0 << (bits - 1) << 1));

                let mut heads = [usize::MAX; B];
                let mut next = [usize::MAX; B];
                let mut bucket = [first; B];
                let mut i = 0;
                if shift == 0 {
                    each!(a; i => ($v:expr, $k:expr) => {
                        heads[($v ^ key) as $uty as usize] = i | $k;
                    });
                } else {
                    each!(a; i => ($v:expr, $k:expr) => {
                        let h = &mut heads[(($v ^ key) as $uty >> shift) as usize];
                        next[i | $k] = *h;
                        *h = i | $k;
                    });
                }
                // SAFETY: yes.
                let used = unsafe { core::slice::from_raw_parts(heads.as_ptr(), 1 << (bits - shift)) };
                each!(used => ($h:expr, $k:expr) => {
                    if let usize::MAX = $h {} else {
                        let start = dest;
                        let mut last = a[$h];
                        out[dest] = last;
                        dest += 1;
                        let mut i = next[$h];
                        'list: loop {
                            if let usize::MAX = i {
                                break;
                            }
                            let mut v = a[i];
                            i = next[i];
                            if v == last {
                                continue;
                            }
                            if v < last {
                                if dest - start >= 6 {
                                    let mut n = i;
                                    let mut k = 0;
                                    while k != 10 && n != usize::MAX {
                                        n = next[n];
                                        k += 1;
                                    }
                                    if n != usize::MAX || dest - start >= 32 {
                                        let mut len = 0;
                                        i = $h;
                                        while i != usize::MAX {
                                            bucket[len] = a[i];
                                            len += 1;
                                            i = next[i];
                                        }
                                        // SAFETY: yes.
                                        let b = unsafe { core::slice::from_raw_parts(bucket.as_ptr(), len) };
                                        dest = match len {
                                            ..1024 => sort::<N, 1024>(b, out, start),
                                            _ => sort::<N, B>(b, out, start),
                                        };
                                        break;
                                    }
                                }
                                let mut j = start;
                                loop {
                                    let o = out[j];
                                    if v < o {
                                        out[j] = v;
                                        v = o;
                                        if v == last {
                                            break;
                                        }
                                    } else if v == o {
                                        continue 'list;
                                    }
                                    j += 1;
                                }
                            }
                            out[dest] = v;
                            last = v;
                            dest += 1;
                        }
                    }
                });
                dest
            }

            let &[mut last, ref tail @ ..] = a.as_slice() else {
                return Vec::new();
            };
            let mut out = a;
            let mut len = 1;
            let mut sorted = true;
            each!(tail, sorted => ($v:expr, $k:expr) => {
                if last != $v {
                    sorted &= last < $v;
                    out[len] = $v;
                    len += 1;
                    last = $v;
                }
            });
            if !sorted {
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
