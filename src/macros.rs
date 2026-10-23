macro_rules! if_signed {
    (u8,    {$($tt:tt)+}) => { /* nothing */};
    (u16,   {$($tt:tt)+}) => { /* nothing */};
    (u32,   {$($tt:tt)+}) => { /* nothing */};
    (u64,   {$($tt:tt)+}) => { /* nothing */};
    (u128,  {$($tt:tt)+}) => { /* nothing */};
    (usize, {$($tt:tt)+}) => { /* nothing */};
    (i8,    {$($tt:tt)+}) => {$($tt)+};
    (i16,   {$($tt:tt)+}) => {$($tt)+};
    (i32,   {$($tt:tt)+}) => {$($tt)+};
    (i64,   {$($tt:tt)+}) => {$($tt)+};
    (i128,  {$($tt:tt)+}) => {$($tt)+};
    (isize, {$($tt:tt)+}) => {$($tt)+};
}
pub(crate) use if_signed;

macro_rules! if_unsigned {
    (u8,    {$($tt:tt)+}) => {$($tt)+};
    (u16,   {$($tt:tt)+}) => {$($tt)+};
    (u32,   {$($tt:tt)+}) => {$($tt)+};
    (u64,   {$($tt:tt)+}) => {$($tt)+};
    (u128,  {$($tt:tt)+}) => {$($tt)+};
    (usize, {$($tt:tt)+}) => {$($tt)+};
    (i8,    {$($tt:tt)+}) => { /* nothing */};
    (i16,   {$($tt:tt)+}) => { /* nothing */};
    (i32,   {$($tt:tt)+}) => { /* nothing */};
    (i64,   {$($tt:tt)+}) => { /* nothing */};
    (i128,  {$($tt:tt)+}) => { /* nothing */};
    (isize, {$($tt:tt)+}) => { /* nothing */};
}
pub(crate) use if_unsigned;

#[doc(hidden)]
#[macro_export]
macro_rules! concat2 {
    ($literal:literal) => {
        $literal
    };
    (#{$($tt:tt)+}) => {
        concat!("`",stringify!($($tt)+),"`")
    };
    ({$($tt:tt)+} $(,)?) => {
        stringify!($($tt)+)
    };
    ($literal:literal $($tail:tt)*) => {
        concat!($literal, $crate::concat2!($($tail)*))
    };
    (#{$($tt:tt)+} $($tail:tt)*) => {
        concat!("`",stringify!($($tt)+),"`", $crate::concat2!($($tail)*))
    };
    ({$($tt:tt)+} $($tail:tt)*) => {
        concat!(stringify!($($tt)+), $crate::concat2!($($tail)*))
    };
}

#[rustfmt::skip]
macro_rules! _doc_unary_debug {
    (fn_name: $fn_name:ident, input_t: $input_t:ident, codomain_t: $codomain_t:ident) => {
        $crate::concat2!("fn_name: "#{$fn_name}", input_t: "#{$input_t}", codomain_t: "#{$codomain_t}".")
    };
}
// pub(crate) use _doc_unary_debug;

#[rustfmt::skip]
macro_rules! doc_unary_std {
    (fn_name: $fn_name:ident, input_t: $input_t:ident, codomain_t: $codomain_t:ident) => {
        $crate::concat2!("This method is the equivalent of [`"{$input_t}"::"{$fn_name}"`].")
    };
}
pub(crate) use doc_unary_std;

#[rustfmt::skip]
macro_rules! doc_unary_as {
    (fn_name: $fn_name:ident, input_t: $input_t:ident, codomain_t: $codomain_t:ident) => {
        $crate::concat2!(
            "This method is the equivalent of `"{$input_t}" as "{$codomain_t}"`.\n\n"
            "**I do not recommend using this method**, it exists for completeness.\n\n"
            "If at all possible, prefer [`Sure<"{$input_t}", _>::to_"{$codomain_t}"`], since it is guaranteed to be lossless."
        )
    };
}
pub(crate) use doc_unary_as;

#[rustfmt::skip]
macro_rules! doc_unary_to {
    (fn_name: $fn_name:ident, input_t: $input_t:ident, codomain_t: $codomain_t:ident) => {
        crate::concat2!(
            "This performs a lossless conversion from "#{$input_t}" to "#{$codomain_t}" or fails to compile if this isn't possible."
        )
    };
}
pub(crate) use doc_unary_to;

macro_rules! impl_simple_unary_ops {
    ($([inner_t: $inner_t:ident, trait_fn_name: $trait_fn_name:ident, op_trait: $(::$op_trait:ident)+, op: $op:tt]),+ $(,)?) => {$(

        mod ${concat($inner_t,_,$trait_fn_name)} {
            use crate::base::Set;
            use crate::base::Sure;
            use crate::const_helpers::array_from_fn;
            use crate::set::LENGTH;

            const CODOMAIN<const SET: Set<$inner_t>>: Set<$inner_t> = {
                &array_from_fn!($inner_t, LENGTH::<$inner_t, SET>, const |i| {
                    let a: $inner_t = SET[i];
                    $op a
                })
            };

            const impl<const SET: Set<$inner_t>> $(::$op_trait)+ for Sure<$inner_t, SET> {
                type Output = Sure<$inner_t, { CODOMAIN::<SET> }>;


                fn $trait_fn_name(self) -> Self::Output {
                    let self_inner: $inner_t = self.inner();
                    let res_inner: $inner_t = $op self_inner;
                    // SAFETY: TODO
                    unsafe { Sure::new_unchecked(res_inner) }
                }
            }
        }
    )+}
}
pub(crate) use impl_simple_unary_ops;

macro_rules! impl_simple_binary_ops {
    ($([inner_t: $inner_t:ident, trait_fn_name: $trait_fn_name:ident, op_trait: $(::$op_trait:ident)+, op: $op:tt]),+ $(,)?) => {$(

        mod ${concat($inner_t,_,$trait_fn_name)} {
            use crate::base::Set;
            use crate::base::Sure;
            use crate::set::CARTESIAN_LENGTH;

            #[expect(non_snake_case)]
            const fn codomain<const __A: Set<$inner_t>, const __B: Set<$inner_t>, const LEN: usize>() -> [$inner_t; LEN] {
                let A = __A;
                let B = __B;

                let mut codomain: [$inner_t; LEN] = [<$inner_t>::default(); _];

                if LEN == 0 {
                    return codomain;
                }

                let (a_chunks, a_tail) = A.as_chunks::<8>();
                let (b_chunks, b_tail) = B.as_chunks::<8>();
                let (codomain_chunks, _) = codomain.as_chunks_mut::<64>();

                let (a_chunks_len, a_tail_len) = (a_chunks.len(), a_tail.len());
                let (b_chunks_len, b_tail_len) = (b_chunks.len(), b_tail.len());

                let mut a_chunks_index: usize = 0;
                let mut codomain_chunks_index: usize = 0;

                while a_chunks_index < a_chunks_len {
                    let &[a1, a2, a3, a4, a5, a6, a7, a8] = &a_chunks[a_chunks_index]; a_chunks_index += 1;

                    let mut b_chunks_index: usize = 0;
                    while b_chunks_index < b_chunks_len {
                        let &[b1, b2, b3, b4, b5, b6, b7, b8] = &b_chunks[b_chunks_index]; b_chunks_index += 1;

                        codomain_chunks[codomain_chunks_index] = [
                            a1 $op b1, a1 $op b2, a1 $op b3, a1 $op b4, a1 $op b5, a1 $op b6, a1 $op b7, a1 $op b8,
                            a2 $op b1, a2 $op b2, a2 $op b3, a2 $op b4, a2 $op b5, a2 $op b6, a2 $op b7, a2 $op b8,
                            a3 $op b1, a3 $op b2, a3 $op b3, a3 $op b4, a3 $op b5, a3 $op b6, a3 $op b7, a3 $op b8,
                            a4 $op b1, a4 $op b2, a4 $op b3, a4 $op b4, a4 $op b5, a4 $op b6, a4 $op b7, a4 $op b8,
                            a5 $op b1, a5 $op b2, a5 $op b3, a5 $op b4, a5 $op b5, a5 $op b6, a5 $op b7, a5 $op b8,
                            a6 $op b1, a6 $op b2, a6 $op b3, a6 $op b4, a6 $op b5, a6 $op b6, a6 $op b7, a6 $op b8,
                            a7 $op b1, a7 $op b2, a7 $op b3, a7 $op b4, a7 $op b5, a7 $op b6, a7 $op b7, a7 $op b8,
                            a8 $op b1, a8 $op b2, a8 $op b3, a8 $op b4, a8 $op b5, a8 $op b6, a8 $op b7, a8 $op b8,
                        ];
                        codomain_chunks_index += 1;
                    }
                }

                let mut codomain_already_filled_count: usize = codomain_chunks_index * 64;

                if codomain_already_filled_count == LEN {
                    return codomain;
                }

                let (codomain_rest, _) = codomain[codomain_already_filled_count..].as_chunks_mut::<8>();
                let mut codomain_rest_index: usize = 0;

                let mut a_chunks_index: usize = 0;

                while a_chunks_index < a_chunks_len {
                    let &[a1, a2, a3, a4, a5, a6, a7, a8] = &a_chunks[a_chunks_index]; a_chunks_index += 1;

                    let mut b_tail_index: usize = 0;

                    while b_tail_index < b_tail_len {
                        let b_tail_elem = b_tail[b_tail_index]; b_tail_index += 1;
                        codomain_rest[codomain_rest_index] = [
                            a1 $op b_tail_elem,
                            a2 $op b_tail_elem,
                            a3 $op b_tail_elem,
                            a4 $op b_tail_elem,
                            a5 $op b_tail_elem,
                            a6 $op b_tail_elem,
                            a7 $op b_tail_elem,
                            a8 $op b_tail_elem,
                        ];
                        codomain_rest_index += 1;
                    }
                }

                let mut a_tail_index: usize = 0;

                while a_tail_index < a_tail_len {
                    let a_tail_elem = a_tail[a_tail_index]; a_tail_index += 1;
                    let mut b_chunks_index: usize = 0;

                    while b_chunks_index < b_chunks_len {
                        let &[b1, b2, b3, b4, b5, b6, b7, b8] = &b_chunks[b_chunks_index]; b_chunks_index += 1;
                        codomain_rest[codomain_rest_index] = [
                            a_tail_elem $op b1,
                            a_tail_elem $op b2,
                            a_tail_elem $op b3,
                            a_tail_elem $op b4,
                            a_tail_elem $op b5,
                            a_tail_elem $op b6,
                            a_tail_elem $op b7,
                            a_tail_elem $op b8,
                        ];
                        codomain_rest_index += 1;
                    }
                }

                codomain_already_filled_count += codomain_rest_index * 8;

                let codomain_rest = &mut codomain[codomain_already_filled_count..];
                let mut codomain_rest_index: usize = 0;
                let mut a_tail_index: usize = 0;

                while a_tail_index < a_tail_len {
                    let a_tail_elem = a_tail[a_tail_index]; a_tail_index += 1;
                    let mut b_tail_index: usize = 0;
                    while b_tail_index < b_tail_len {
                        let b_tail_elem = b_tail[b_tail_index]; b_tail_index += 1;
                        codomain_rest[codomain_rest_index] = a_tail_elem $op b_tail_elem;
                        codomain_rest_index += 1;
                    }
                }

                codomain
            }

            const CODOMAIN<const A: Set<$inner_t>, const B: Set<$inner_t>>: Set<$inner_t> = &codomain::<A, B, { CARTESIAN_LENGTH::<$inner_t, $inner_t, A, B> }>();

            const impl<const A: Set<$inner_t>, const B: Set<$inner_t>> $(::$op_trait)+<Sure<$inner_t, B> > for Sure<$inner_t, A> {
                type Output = Sure<$inner_t, { CODOMAIN::<A, B> }>;

                fn $trait_fn_name(self, b: Sure<$inner_t, B>) -> Self::Output {
                    let a_inner: $inner_t = self.inner();
                    let b_inner: $inner_t = b.inner();
                    let res_inner: $inner_t = a_inner $op b_inner;
                    // SAFETY: TODO
                    unsafe { Sure::new_unchecked(res_inner) }
                }
            }
        }
    )+}
}
pub(crate) use impl_simple_binary_ops;

macro_rules! impl_unary_fns {
    ($([fn $fn_name:ident($input_t:ident) -> $codomain_t:ident, fn_path: $fn_path:path, doc_macro_path: $doc_macro_path:ident]),+ $(,)?) => {$(

        mod ${concat($input_t,_,$fn_name)} {
            use crate::base::Set;
            use crate::base::Sure;
            use crate::const_helpers::array_from_fn;
            use crate::set::LENGTH;

            const CODOMAIN<const SET: Set<$input_t>>: Set<$codomain_t> = {
                &array_from_fn!($codomain_t, LENGTH::<$input_t, SET>, const |i| {
                    let a: $input_t = SET[i];
                    $fn_path(a)
                })
            };

            impl<const SET: Set<$input_t>> Sure<$input_t, SET> {
                #[doc = crate::macros::$doc_macro_path!(fn_name: $fn_name, input_t: $input_t, codomain_t: $codomain_t)]
                #[must_use]
                pub const fn $fn_name(self) -> Sure<$codomain_t, { CODOMAIN::<SET> }> {
                    let input_inner: $input_t = self.inner();
                    let output_inner: $codomain_t = $fn_path(input_inner);
                    // SAFETY: TODO
                    unsafe { Sure::new_unchecked(output_inner) }
                }
            }
        }

    )+}
}
pub(crate) use impl_unary_fns;

macro_rules! impl_std_binary_fns {
    ($([fn $fn_name:ident($a_t:ident, $b_t:ident) -> $codomain_t:ty, fn_path: $fn_path:path]),+ $(,)?) => {$(

        mod ${concat($a_t,_,$b_t,_,$fn_name)} {
            use crate::base::Set;
            use crate::base::Sure;
            use crate::concat2;
            use crate::set::CARTESIAN_LENGTH;

            const CODOMAIN<const A: Set<$a_t>, const B: Set<$b_t>>: Set<$codomain_t> = &{
                let a_len = A.len();
                let b_len = B.len();

                let mut codomain: [$codomain_t; CARTESIAN_LENGTH::<$a_t, $b_t, A, B>] = [<$codomain_t>::default(); _];
                let mut codomain_index: usize = 0;

                let mut a_index: usize = 0;

                while a_index < a_len {
                    let a = A[a_index];
                    let mut b_index: usize = 0;
                    while b_index < b_len {
                        let b = B[b_index];
                        codomain[codomain_index] = $fn_path(a, b);
                        b_index += 1;
                        codomain_index += 1;
                    }
                    a_index += 1;
                }
                codomain
            };

            impl<const A: Set<$a_t>> Sure<$a_t, A> {
                #[doc = concat2!("This method is the equivalent of [`"{$a_t}"::"{$fn_name}"`].")]
                #[must_use]
                pub const fn $fn_name<const B: Set<$b_t>>(self, b: Sure<$b_t, B>) -> Sure<$codomain_t, { CODOMAIN::<A, B> }> {
                    let a_inner: $a_t = self.inner();
                    let b_inner: $b_t = b.inner();
                    let output_inner: $codomain_t = $fn_path(a_inner, b_inner);
                    // SAFETY: TODO
                    unsafe { Sure::new_unchecked(output_inner) }
                }
            }
        }

    )+}
}
pub(crate) use impl_std_binary_fns;

macro_rules! impl_ints {
    (the_dolla: $d:tt, $([num_t: $num_t:ident, unsigned_num_t: $unsigned_num_t:ident, signed_num_t: $signed_num_t:ident, t_alias: $t_alias:ident, wide_num_t: $wide_num_t:ident, private_macro_prefix: $private_macro_prefix:ident, extra_mod: $extra_mod:ident],)*) => {$(

        #[doc = concat2!("A type alias for `Sure<"{$num_t}", _>`.")]
        pub type $t_alias<const SET: Set<$num_t>> = Sure<$num_t, SET>;

        #[doc = crate::concat2!("Helper consts for "#{$t_alias}".")]
        pub mod $extra_mod {
            use crate::concat2;
            use crate::const_helpers::array_from_fn;

            const RANGE_LENGTH_HELPER<const MIN: $num_t, const MAX: $num_t, const IS_INCLUSIVE: bool>: usize = const {
                let wide_min = $wide_num_t::from(MIN);
                let wide_max = $wide_num_t::from(MAX);
                let exclusive_length = wide_max.strict_sub(wide_min);
                let exclusive_length: usize = usize::try_from(exclusive_length).ok().expect("Range length could not be converted into a usize.");
                let inclusive_addition: usize = usize::from(IS_INCLUSIVE);
                exclusive_length.strict_add(inclusive_addition)
            };

            const RANGE_HELPER<const MIN: $num_t, const MAX: $num_t, const IS_INCLUSIVE: bool>: &[$num_t] = const {
                let wide_min = $wide_num_t::from(MIN);

                &array_from_fn!($num_t, RANGE_LENGTH_HELPER::<MIN, MAX, IS_INCLUSIVE>, const |i| {
                    let wide_index = $wide_num_t::try_from(i).ok().expect("");
                    $num_t::try_from(wide_min + wide_index).ok().unwrap()
                })
            };

            #[doc = concat2!("Creates a slice of "#{$num_t}" values containing the numbers in `MIN..MAX`.")]
            pub const RANGE             <const MIN: $num_t, const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<                MIN ,                 MAX   , false >;
            #[doc = concat2!("Creates a slice of "#{$num_t}" values containing the numbers in `MIN..`.")]
            pub const RANGE_FROM        <const MIN: $num_t                   >: &[$num_t] = RANGE_HELPER::<                MIN ,{const { $num_t::MAX }}, true  >;
            // RANGE_FULL omitted
            #[doc = concat2!("Creates a slice of "#{$num_t}" values containing the numbers in `MIN..=MAX`.")]
            pub const RANGE_INCLUSIVE   <const MIN: $num_t, const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<                MIN ,                 MAX   , true  >;
            #[doc = concat2!("Creates a slice of "#{$num_t}" values containing the numbers in `..MAX`.")]
            pub const RANGE_TO          <                   const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<{const{ $num_t::MIN }},               MAX   , false >;
            #[doc = concat2!("Creates a slice of "#{$num_t}" values containing the numbers in `..=MAX`.")]
            pub const RANGE_TO_INCLUSIVE<                   const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<{const{ $num_t::MIN }},               MAX   , true  >;

            #[doc = concat2!("A macro to make a slice of "#{$num_t}" values using the familiar range syntax.")]
            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, range) } {
                ( $min:literal ..  $max:literal ) => { $d crate::$extra_mod::RANGE::             <$min, $max> };
                ( $min:literal ..               ) => { $d crate::$extra_mod::RANGE_FROM::        <$min      > };
                // RANGE_FULL omitted
                ( $min:literal ..= $max:literal ) => { $d crate::$extra_mod::RANGE_INCLUSIVE::   <$min, $max> };
                (              ..  $max:literal ) => { $d crate::$extra_mod::RANGE_TO::          <      $max> };
                (              ..= $max:literal ) => { $d crate::$extra_mod::RANGE_TO_INCLUSIVE::<      $max> };
            }
            pub use ${ concat($private_macro_prefix, range) } as Range;

            #[doc = concat2!("A macro to form the union of slices of "#{$num_t}" values.")]
            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, union) } {
                ($d($set:expr),+ $d(,)?) => {
                    $d crate::set::UNION::<$num_t, { &[$d($set, )+] }>
                };
            }
            pub use ${ concat($private_macro_prefix, union) } as Union;

            #[doc = concat2!("A macro to form the intersection of slices of "#{$num_t}" values.")]
            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, intersection) } {
                ($d($set:expr),+ $d(,)?) => {
                    $d crate::set::INTERSECTION::<$num_t, { &[$d($set, )+] }>
                };
            }
            pub use ${ concat($private_macro_prefix, intersection) } as Intersection;

            macro_rules! as_fns {
                ($d ($ty:ident),+) => {$d(
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
                    #[inline]
                    pub(crate) const fn $d{ concat(as_, $d ty) }(value: $num_t) -> $ty {
                        value as $ty
                    }
                )+}
            }

            as_fns!{u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize}


            macro_rules! to_fns {
                ($d ($ty:ident),+) => {$d(
                    #[inline]
                    pub(crate) const fn $d{ concat(to_, $d ty) }(value: $num_t) -> $ty {
                        value.try_into().ok().expect(concat2!("Unable to losslessly convert to "#{$ty}))
                    }
                )+}
            }

            to_fns!{u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize}

        }

        impl<const SET: Set<$num_t>> Sure<$num_t, SET> {
            #[doc = concat2!("Creates a `NonZero<"{$num_t}">` or fails to compile if the value could be zero.")]
            #[inline]
            pub const fn to_non_zero(self) -> core::num::NonZero<$num_t> {
                const { assert!(!Self::set_contains(&0), "A Sure containing 0 cannot be converted to NonZero")}
                let self_inner = self.inner();
                // SAFETY: we just asserted that `self_inner` can never be 0
                unsafe { core::num::NonZero::new_unchecked(self_inner) }
            }
        }

        #[doc = "A convenience macro for writing the type of the same name with fewer `<{[]}>` braces."]
        #[macro_export]
        macro_rules! $t_alias {
            ($elem:literal) => {
                $d crate::Sure::<$num_t, { &[$elem] }>
            };
            ($set:expr) => {
                $d crate::Sure::<$num_t, { $set }>
            };
            ($d($elem:expr),+ $d(,)?) => {
                $d crate::Sure::<$num_t, { &[$d($elem, )+] }>
            };
        }

        //~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~STD~OPS~~~~~

        //~~~~~UNARY~~~~~~

        macros::impl_simple_unary_ops! {
            [ inner_t: $num_t, trait_fn_name: not    , op_trait: ::core::ops::Not     , op: !  ],
        }

        macros::if_signed!{ $num_t, { macros::impl_simple_unary_ops! {
            [ inner_t: $num_t, trait_fn_name: neg    , op_trait: ::core::ops::Neg     , op: -  ],
        }}}

        //~~~~~BINARY~~~~~~

        macros::impl_simple_binary_ops! {
            [ inner_t: $num_t, trait_fn_name: add    , op_trait: ::core::ops::Add     , op: +  ],
            [ inner_t: $num_t, trait_fn_name: sub    , op_trait: ::core::ops::Sub     , op: -  ],
            [ inner_t: $num_t, trait_fn_name: mul    , op_trait: ::core::ops::Mul     , op: *  ],
            [ inner_t: $num_t, trait_fn_name: div    , op_trait: ::core::ops::Div     , op: /  ],

            [ inner_t: $num_t, trait_fn_name: rem    , op_trait: ::core::ops::Rem     , op: %  ],

            [ inner_t: $num_t, trait_fn_name: bitand , op_trait: ::core::ops::BitAnd  , op: &  ],
            [ inner_t: $num_t, trait_fn_name: bitor  , op_trait: ::core::ops::BitOr   , op: |  ],
            [ inner_t: $num_t, trait_fn_name: bitxor , op_trait: ::core::ops::BitXor  , op: ^  ],

            [ inner_t: $num_t, trait_fn_name: shl    , op_trait: ::core::ops::Shl     , op: << ],
            [ inner_t: $num_t, trait_fn_name: shr    , op_trait: ::core::ops::Shr     , op: >> ],
        }

        //~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~STD~FNS~~~~~

        //~~~~~UNARY~~~~~~

        macros::if_signed!{ $num_t, {
        macros::impl_unary_fns! {
            [ fn abs($num_t) -> $num_t                                    , fn_path: ::core::primitive::$num_t::abs                   , doc_macro_path: doc_unary_std ],
            [ fn strict_abs($num_t) -> $num_t                             , fn_path: ::core::primitive::$num_t::strict_abs            , doc_macro_path: doc_unary_std ],
            [ fn unsigned_abs($num_t) -> $unsigned_num_t                  , fn_path: ::core::primitive::$num_t::unsigned_abs          , doc_macro_path: doc_unary_std ],
        }}}

        macros::if_unsigned!{ $num_t, {
        macros::impl_unary_fns! {
            [ fn bit_width($num_t) -> u32                                 , fn_path: ::core::primitive::$num_t::bit_width             , doc_macro_path: doc_unary_std ],
            [ fn cast_signed($num_t) -> $signed_num_t                     , fn_path: ::core::primitive::$num_t::cast_signed           , doc_macro_path: doc_unary_std ],

            [ fn is_power_of_two($num_t) -> bool                          , fn_path: ::core::primitive::$num_t::is_power_of_two       , doc_macro_path: doc_unary_std ],
            [ fn next_power_of_two($num_t) -> $num_t                      , fn_path: ::core::primitive::$num_t::next_power_of_two     , doc_macro_path: doc_unary_std ],
        }}}

        macros::impl_unary_fns! {
            [ fn count_ones($num_t) -> u32                                , fn_path: ::core::primitive::$num_t::count_ones            , doc_macro_path: doc_unary_std ],
            [ fn count_zeros($num_t) -> u32                               , fn_path: ::core::primitive::$num_t::count_zeros           , doc_macro_path: doc_unary_std ],

            [ fn ilog2($num_t) -> u32                                     , fn_path: ::core::primitive::$num_t::ilog2                 , doc_macro_path: doc_unary_std ],
            [ fn ilog10($num_t) -> u32                                    , fn_path: ::core::primitive::$num_t::ilog10                , doc_macro_path: doc_unary_std ],

            [ fn isolate_highest_one($num_t) -> $num_t                    , fn_path: ::core::primitive::$num_t::isolate_highest_one   , doc_macro_path: doc_unary_std ],
            [ fn isolate_lowest_one($num_t) -> $num_t                     , fn_path: ::core::primitive::$num_t::isolate_lowest_one    , doc_macro_path: doc_unary_std ],

            [ fn isqrt($num_t) -> $num_t                                  , fn_path: ::core::primitive::$num_t::isqrt                 , doc_macro_path: doc_unary_std ],

            [ fn leading_ones($num_t) -> u32                              , fn_path: ::core::primitive::$num_t::leading_ones          , doc_macro_path: doc_unary_std ],
            [ fn leading_zeros($num_t) -> u32                             , fn_path: ::core::primitive::$num_t::leading_zeros         , doc_macro_path: doc_unary_std ],

            [ fn reverse_bits($num_t) -> $num_t                           , fn_path: ::core::primitive::$num_t::reverse_bits          , doc_macro_path: doc_unary_std ],

            [ fn strict_neg($num_t) -> $num_t                             , fn_path: ::core::primitive::$num_t::strict_neg            , doc_macro_path: doc_unary_std ],

            [ fn swap_bytes($num_t) -> $num_t                             , fn_path: ::core::primitive::$num_t::swap_bytes            , doc_macro_path: doc_unary_std ],

            [ fn to_be($num_t) -> $num_t                                  , fn_path: ::core::primitive::$num_t::to_be                 , doc_macro_path: doc_unary_std ],
            [ fn to_le($num_t) -> $num_t                                  , fn_path: ::core::primitive::$num_t::to_le                 , doc_macro_path: doc_unary_std ],

            [ fn trailing_ones($num_t) -> u32                             , fn_path: ::core::primitive::$num_t::trailing_ones         , doc_macro_path: doc_unary_std ],
            [ fn trailing_zeros($num_t) -> u32                            , fn_path: ::core::primitive::$num_t::trailing_zeros        , doc_macro_path: doc_unary_std ],
        }

        //~~~~~BINARY~~~~~~

        macros::if_unsigned!{ $num_t, {
        macros::impl_std_binary_fns! {
            [ fn div_ceil($num_t, $num_t) -> $num_t                       , fn_path: ::core::primitive::$num_t::div_ceil               ],

            [ fn is_multiple_of($num_t, $num_t) -> bool                   , fn_path: ::core::primitive::$num_t::is_multiple_of         ],
            [ fn next_multiple_of($num_t, $num_t) -> $num_t               , fn_path: ::core::primitive::$num_t::next_multiple_of       ],

            [ fn saturating_add_signed($num_t, $signed_num_t) -> $num_t   , fn_path: ::core::primitive::$num_t::saturating_add_signed  ],
            [ fn saturating_sub_signed($num_t, $signed_num_t) -> $num_t   , fn_path: ::core::primitive::$num_t::saturating_sub_signed  ],

            [ fn strict_add_signed($num_t, $signed_num_t) -> $num_t       , fn_path: ::core::primitive::$num_t::strict_add_signed      ],
            [ fn strict_sub_signed($num_t, $signed_num_t) -> $num_t       , fn_path: ::core::primitive::$num_t::strict_sub_signed      ],

            [ fn wrapping_add_signed($num_t, $signed_num_t) -> $num_t     , fn_path: ::core::primitive::$num_t::wrapping_add_signed    ],
            [ fn wrapping_sub_signed($num_t, $signed_num_t) -> $num_t     , fn_path: ::core::primitive::$num_t::wrapping_sub_signed    ],
        }}}

        macros::impl_std_binary_fns! {
            [ fn abs_diff($num_t, $num_t) -> $unsigned_num_t              , fn_path: ::core::primitive::$num_t::abs_diff               ],
            [ fn div_euclid($num_t, $num_t) -> $num_t                     , fn_path: ::core::primitive::$num_t::div_euclid             ],
            [ fn ilog($num_t, $num_t) -> u32                              , fn_path: ::core::primitive::$num_t::ilog                   ],
            [ fn midpoint($num_t, $num_t) -> $num_t                       , fn_path: ::core::primitive::$num_t::midpoint               ],

            [ fn overflowing_add($num_t, $num_t) -> ($num_t, bool)        , fn_path: ::core::primitive::$num_t::overflowing_add        ],
            [ fn overflowing_div($num_t, $num_t) -> ($num_t, bool)        , fn_path: ::core::primitive::$num_t::overflowing_div        ],
            [ fn overflowing_div_euclid($num_t, $num_t) -> ($num_t, bool) , fn_path: ::core::primitive::$num_t::overflowing_div_euclid ],
            [ fn overflowing_mul($num_t, $num_t) -> ($num_t, bool)        , fn_path: ::core::primitive::$num_t::overflowing_mul        ],
            [ fn overflowing_pow($num_t, u32) -> ($num_t, bool)           , fn_path: ::core::primitive::$num_t::overflowing_pow        ],
            [ fn overflowing_rem($num_t, $num_t) -> ($num_t, bool)        , fn_path: ::core::primitive::$num_t::overflowing_rem        ],
            [ fn overflowing_rem_euclid($num_t, $num_t) -> ($num_t, bool) , fn_path: ::core::primitive::$num_t::overflowing_rem_euclid ],
            [ fn overflowing_shl($num_t, u32) -> ($num_t, bool)           , fn_path: ::core::primitive::$num_t::overflowing_shl        ],
            [ fn overflowing_shr($num_t, u32) -> ($num_t, bool)           , fn_path: ::core::primitive::$num_t::overflowing_shr        ],
            [ fn overflowing_sub($num_t, $num_t) -> ($num_t, bool)        , fn_path: ::core::primitive::$num_t::overflowing_sub        ],

            [ fn pow($num_t, u32) -> $num_t                               , fn_path: ::core::primitive::$num_t::pow                    ],
            [ fn rem_euclid($num_t, $num_t) -> $num_t                     , fn_path: ::core::primitive::$num_t::rem_euclid             ],
            [ fn rotate_left($num_t, u32) -> $num_t                       , fn_path: ::core::primitive::$num_t::rotate_left            ],
            [ fn rotate_right($num_t, u32) -> $num_t                      , fn_path: ::core::primitive::$num_t::rotate_right           ],

            [ fn saturating_add($num_t, $num_t) -> $num_t                 , fn_path: ::core::primitive::$num_t::saturating_add         ],
            [ fn saturating_div($num_t, $num_t) -> $num_t                 , fn_path: ::core::primitive::$num_t::saturating_div         ],
            [ fn saturating_mul($num_t, $num_t) -> $num_t                 , fn_path: ::core::primitive::$num_t::saturating_mul         ],
            [ fn saturating_pow($num_t, u32) -> $num_t                    , fn_path: ::core::primitive::$num_t::saturating_pow         ],
            [ fn saturating_sub($num_t, $num_t) -> $num_t                 , fn_path: ::core::primitive::$num_t::saturating_sub         ],

            [ fn strict_add($num_t, $num_t) -> $num_t                     , fn_path: ::core::primitive::$num_t::strict_add             ],
            [ fn strict_div($num_t, $num_t) -> $num_t                     , fn_path: ::core::primitive::$num_t::strict_div             ],
            [ fn strict_div_euclid($num_t, $num_t) -> $num_t              , fn_path: ::core::primitive::$num_t::strict_div_euclid      ],
            [ fn strict_mul($num_t, $num_t) -> $num_t                     , fn_path: ::core::primitive::$num_t::strict_mul             ],
            [ fn strict_pow($num_t, u32) -> $num_t                        , fn_path: ::core::primitive::$num_t::strict_pow             ],
            [ fn strict_rem($num_t, $num_t) -> $num_t                     , fn_path: ::core::primitive::$num_t::strict_rem             ],
            [ fn strict_rem_euclid($num_t, $num_t) -> $num_t              , fn_path: ::core::primitive::$num_t::strict_rem_euclid      ],
            [ fn strict_shl($num_t, u32) -> $num_t                        , fn_path: ::core::primitive::$num_t::strict_shl             ],
            [ fn strict_shr($num_t, u32) -> $num_t                        , fn_path: ::core::primitive::$num_t::strict_shr             ],
            [ fn strict_sub($num_t, $num_t) -> $num_t                     , fn_path: ::core::primitive::$num_t::strict_sub             ],

            [ fn unbounded_shl($num_t, u32) -> $num_t                     , fn_path: ::core::primitive::$num_t::unbounded_shl          ],
            [ fn unbounded_shr($num_t, u32) -> $num_t                     , fn_path: ::core::primitive::$num_t::unbounded_shr          ],

            [ fn wrapping_add($num_t, $num_t) -> $num_t                   , fn_path: ::core::primitive::$num_t::wrapping_add           ],
            [ fn wrapping_div($num_t, $num_t) -> $num_t                   , fn_path: ::core::primitive::$num_t::wrapping_div           ],
            [ fn wrapping_div_euclid($num_t, $num_t) -> $num_t            , fn_path: ::core::primitive::$num_t::wrapping_div_euclid    ],
            [ fn wrapping_mul($num_t, $num_t) -> $num_t                   , fn_path: ::core::primitive::$num_t::wrapping_mul           ],
            [ fn wrapping_pow($num_t, u32) -> $num_t                      , fn_path: ::core::primitive::$num_t::wrapping_pow           ],
            [ fn wrapping_rem($num_t, $num_t) -> $num_t                   , fn_path: ::core::primitive::$num_t::wrapping_rem           ],
            [ fn wrapping_rem_euclid($num_t, $num_t) -> $num_t            , fn_path: ::core::primitive::$num_t::wrapping_rem_euclid    ],
            [ fn wrapping_shl($num_t, u32) -> $num_t                      , fn_path: ::core::primitive::$num_t::wrapping_shl           ],
            [ fn wrapping_shr($num_t, u32) -> $num_t                      , fn_path: ::core::primitive::$num_t::wrapping_shr           ],
            [ fn wrapping_sub($num_t, $num_t) -> $num_t                   , fn_path: ::core::primitive::$num_t::wrapping_sub           ],
        }

        //~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~CUSTOM~FNS~~~~~

        //~~~~~UNARY~~~~~~

        macros::impl_unary_fns! {
            [ fn as_u8($num_t) -> u8                                      , fn_path: crate::$extra_mod::as_u8                          , doc_macro_path: doc_unary_as ],
            [ fn as_u16($num_t) -> u16                                    , fn_path: crate::$extra_mod::as_u16                         , doc_macro_path: doc_unary_as ],
            [ fn as_u32($num_t) -> u32                                    , fn_path: crate::$extra_mod::as_u32                         , doc_macro_path: doc_unary_as ],
            [ fn as_u64($num_t) -> u64                                    , fn_path: crate::$extra_mod::as_u64                         , doc_macro_path: doc_unary_as ],
            [ fn as_u128($num_t) -> u128                                  , fn_path: crate::$extra_mod::as_u128                        , doc_macro_path: doc_unary_as ],
            [ fn as_usize($num_t) -> usize                                , fn_path: crate::$extra_mod::as_usize                       , doc_macro_path: doc_unary_as ],

            [ fn as_i8($num_t) -> i8                                      , fn_path: crate::$extra_mod::as_i8                          , doc_macro_path: doc_unary_as ],
            [ fn as_i16($num_t) -> i16                                    , fn_path: crate::$extra_mod::as_i16                         , doc_macro_path: doc_unary_as ],
            [ fn as_i32($num_t) -> i32                                    , fn_path: crate::$extra_mod::as_i32                         , doc_macro_path: doc_unary_as ],
            [ fn as_i64($num_t) -> i64                                    , fn_path: crate::$extra_mod::as_i64                         , doc_macro_path: doc_unary_as ],
            [ fn as_i128($num_t) -> i128                                  , fn_path: crate::$extra_mod::as_i128                        , doc_macro_path: doc_unary_as ],
            [ fn as_isize($num_t) -> isize                                , fn_path: crate::$extra_mod::as_isize                       , doc_macro_path: doc_unary_as ],

            [ fn to_u8($num_t) -> u8                                      , fn_path: crate::$extra_mod::to_u8                          , doc_macro_path: doc_unary_to ],
            [ fn to_u16($num_t) -> u16                                    , fn_path: crate::$extra_mod::to_u16                         , doc_macro_path: doc_unary_to ],
            [ fn to_u32($num_t) -> u32                                    , fn_path: crate::$extra_mod::to_u32                         , doc_macro_path: doc_unary_to ],
            [ fn to_u64($num_t) -> u64                                    , fn_path: crate::$extra_mod::to_u64                         , doc_macro_path: doc_unary_to ],
            [ fn to_u128($num_t) -> u128                                  , fn_path: crate::$extra_mod::to_u128                        , doc_macro_path: doc_unary_to ],
            [ fn to_usize($num_t) -> usize                                , fn_path: crate::$extra_mod::to_usize                       , doc_macro_path: doc_unary_to ],

            [ fn to_i8($num_t) -> i8                                      , fn_path: crate::$extra_mod::to_i8                          , doc_macro_path: doc_unary_to ],
            [ fn to_i16($num_t) -> i16                                    , fn_path: crate::$extra_mod::to_i16                         , doc_macro_path: doc_unary_to ],
            [ fn to_i32($num_t) -> i32                                    , fn_path: crate::$extra_mod::to_i32                         , doc_macro_path: doc_unary_to ],
            [ fn to_i64($num_t) -> i64                                    , fn_path: crate::$extra_mod::to_i64                         , doc_macro_path: doc_unary_to ],
            [ fn to_i128($num_t) -> i128                                  , fn_path: crate::$extra_mod::to_i128                        , doc_macro_path: doc_unary_to ],
            [ fn to_isize($num_t) -> isize                                , fn_path: crate::$extra_mod::to_isize                       , doc_macro_path: doc_unary_to ],
        }

    )*}
}
pub(crate) use impl_ints;
