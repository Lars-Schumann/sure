macro_rules! f {
    ($fn_path:path, $a:expr, $b:expr) => {
        $fn_path($a, $b)
    };
    ($op:tt, $a:expr, $b:expr) => {
        $a $op $b
    };
}
pub(crate) use f;

macro_rules! codomain_fn {
    (a_t: $a_t:ty, b_t: $b_t:ty, codomain_t: $codomain_t:ty, op: {$op:tt}) => {
        #[doc(hidden)] // only pub for easier testing
        pub const fn codomain<const LEN: usize>(a_set: &[$a_t], b_set: &[$b_t]) -> [$codomain_t; LEN] {
            use crate::codomain::f;
            use crate::for_macro::for_;

            let mut codomain: [$codomain_t; LEN] = [<$codomain_t>::default(); _];

            let (a_chunks, a_tail) = a_set.as_chunks::<8>();
            let (b_chunks, b_tail) = b_set.as_chunks::<8>();

            let (a_chunks_len, a_tail_len) = (a_chunks.len(), a_tail.len());
            let (b_chunks_len, b_tail_len) = (b_chunks.len(), b_tail.len());

            if b_chunks_len > 0 {
                let (codomain_chunks, _tail) = codomain.as_chunks_mut::<64>();
                let mut codomain_chunks_index: usize = 0;

                for_!(a_chunks_index in 0..a_chunks_len {
                    let [a1, a2, a3, a4, a5, a6, a7, a8] = a_chunks[a_chunks_index];
                    for_!(b_chunks_index in 0..b_chunks_len {
                        let [b1, b2, b3, b4, b5, b6, b7, b8] = b_chunks[b_chunks_index];

                        codomain_chunks[codomain_chunks_index] = [
                            f!($op, a1, b1), f!($op, a1, b2), f!($op, a1, b3), f!($op, a1, b4), f!($op, a1, b5), f!($op, a1, b6), f!($op, a1, b7), f!($op, a1, b8),
                            f!($op, a2, b1), f!($op, a2, b2), f!($op, a2, b3), f!($op, a2, b4), f!($op, a2, b5), f!($op, a2, b6), f!($op, a2, b7), f!($op, a2, b8),
                            f!($op, a3, b1), f!($op, a3, b2), f!($op, a3, b3), f!($op, a3, b4), f!($op, a3, b5), f!($op, a3, b6), f!($op, a3, b7), f!($op, a3, b8),
                            f!($op, a4, b1), f!($op, a4, b2), f!($op, a4, b3), f!($op, a4, b4), f!($op, a4, b5), f!($op, a4, b6), f!($op, a4, b7), f!($op, a4, b8),
                            f!($op, a5, b1), f!($op, a5, b2), f!($op, a5, b3), f!($op, a5, b4), f!($op, a5, b5), f!($op, a5, b6), f!($op, a5, b7), f!($op, a5, b8),
                            f!($op, a6, b1), f!($op, a6, b2), f!($op, a6, b3), f!($op, a6, b4), f!($op, a6, b5), f!($op, a6, b6), f!($op, a6, b7), f!($op, a6, b8),
                            f!($op, a7, b1), f!($op, a7, b2), f!($op, a7, b3), f!($op, a7, b4), f!($op, a7, b5), f!($op, a7, b6), f!($op, a7, b7), f!($op, a7, b8),
                            f!($op, a8, b1), f!($op, a8, b2), f!($op, a8, b3), f!($op, a8, b4), f!($op, a8, b5), f!($op, a8, b6), f!($op, a8, b7), f!($op, a8, b8),
                        ];
                        codomain_chunks_index += 1;
                    });
                });
            }

            if b_tail_len > 0 {
                let codomain_filled_count: usize = (a_chunks_len * b_chunks_len * 64);
                let (codomain_rest, _tail) = codomain[codomain_filled_count..].as_chunks_mut::<8>();
                let mut codomain_rest_index: usize = 0;

                for_!(a_chunks_index in 0..a_chunks_len {
                    let [a1, a2, a3, a4, a5, a6, a7, a8] = a_chunks[a_chunks_index];

                    for_!(b_tail_index in 0..b_tail_len {
                        let b = b_tail[b_tail_index];

                        codomain_rest[codomain_rest_index] = [
                            f!($op, a1, b),
                            f!($op, a2, b),
                            f!($op, a3, b),
                            f!($op, a4, b),
                            f!($op, a5, b),
                            f!($op, a6, b),
                            f!($op, a7, b),
                            f!($op, a8, b),
                        ];
                        codomain_rest_index += 1;
                    });
                });
            }

            if a_tail_len > 0 {
                let codomain_filled_count: usize = (a_chunks_len * b_chunks_len * 64) + (a_chunks_len * b_tail_len * 8);
                let (codomain_rest, _tail) = codomain[codomain_filled_count..].as_chunks_mut::<8>();
                let mut codomain_rest_index: usize = 0;

                for_!(b_chunks_index in 0..b_chunks_len {
                    let [b1, b2, b3, b4, b5, b6, b7, b8] = b_chunks[b_chunks_index];

                        for_!(a_tail_index in 0..a_tail_len {
                        let a = a_tail[a_tail_index];

                        codomain_rest[codomain_rest_index] = [
                            f!($op, a, b1),
                            f!($op, a, b2),
                            f!($op, a, b3),
                            f!($op, a, b4),
                            f!($op, a, b5),
                            f!($op, a, b6),
                            f!($op, a, b7),
                            f!($op, a, b8),
                        ];
                        codomain_rest_index += 1;
                    });
                });
            }

            if b_tail_len > 0 {
                let codomain_filled_count: usize = (a_chunks_len * b_chunks_len * 64) + (a_chunks_len * b_tail_len * 8) + (b_chunks_len * a_tail_len * 8);
                let codomain_rest = &mut codomain[codomain_filled_count..];
                let mut codomain_rest_index: usize = 0;

                for_!(a_tail_index in 0..a_tail_len {
                    let a = a_tail[a_tail_index];

                    for_!(b_tail_index in 0..b_tail_len {
                        let b = b_tail[b_tail_index];

                        codomain_rest[codomain_rest_index] = f!($op, a, b);
                        codomain_rest_index += 1;
                    });
                });
            }

            codomain
        }
    };
}
pub(crate) use codomain_fn;
