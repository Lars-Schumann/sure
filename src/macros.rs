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

#[rustfmt::skip]
macro_rules! _doc_unary_debug {
    (fn_name: $fn_name:ident, input_t: $input_t:ident, codomain_t: $codomain_t:ident) => {
        concat!("fn_name: `",stringify!($fn_name),"`, input_t: `",stringify!($input_t),"`, codomain_t: `",stringify!($codomain_t),"`.")
    };
}
// pub(crate) use _doc_unary_debug;

#[rustfmt::skip]
macro_rules! doc_unary_std {
    (fn_name: $fn_name:ident, input_t: $input_t:ident, codomain_t: $codomain_t:ident) => {
        concat!("This method is the equivalent of [`",stringify!($input_t),"::",stringify!($fn_name),"`].")
    };
}
pub(crate) use doc_unary_std;

#[rustfmt::skip]
macro_rules! doc_unary_as {
    (fn_name: $fn_name:ident, input_t: $input_t:ident, codomain_t: $codomain_t:ident) => {
        concat!(
            "This method is the equivalent of: `",stringify!($input_t)," as ",stringify!($codomain_t),
            "` \n \n **I do not recommend using this method**, it exists purely for completeness.",
            " \n \n If at all possible prefer the `.to_",stringify!($codomain_t),"` method, since it is guaranteed to be lossless."
        )
    };
}
pub(crate) use doc_unary_as;

#[rustfmt::skip]
macro_rules! doc_unary_to {
    (fn_name: $fn_name:ident, input_t: $input_t:ident, codomain_t: $codomain_t:ident) => {
        concat!(
            "This perfoms a lossless conversion from `",stringify!($input_t),"` to `",stringify!($codomain_t),"` or fails to compile if this isn't possible."
        )
    };
}
pub(crate) use doc_unary_to;

macro_rules! impl_simple_unary_ops {
    ($([inner_t: $inner_t:ident, trait_fn_name: $trait_fn_name:ident, op_trait: $(::$op_trait:ident)+, op: $op:tt]),+ $(,)?) => {$(

        #[expect(non_snake_case)]
        mod ${concat(ඞඞ__,$inner_t,_,$trait_fn_name)} {
            use crate::base::Sure;

            const CODOMAIN<const SET: &'static[$inner_t]>: &[$inner_t] = const {
                &crate::const_helpers::array_from_fn::<$inner_t, { crate::set::LENGTH::<$inner_t, SET> }>(
                    const |i| {
                        let a: $inner_t = SET[i];
                        $op a
                    }
                )
            };

            const impl<const SET: &'static [$inner_t]> $(::$op_trait)+ for Sure<$inner_t, SET> {
                type Output = Sure<$inner_t, { CODOMAIN::<{ SET }> }>;


            fn $trait_fn_name(self) -> Self::Output {
                    let self_inner: $inner_t = self.inner();
                    let res_inner: $inner_t = $op self_inner;
                    // SAFETY: something something cartesian product and pure function... TODO: make this less bad.
                    unsafe { Sure::new_unchecked(res_inner) }
                }
            }
        }
    )+}
}
pub(crate) use impl_simple_unary_ops;

macro_rules! impl_simple_binary_ops {
    ($([inner_t: $inner_t:ident, trait_fn_name: $trait_fn_name:ident, op_trait: $(::$op_trait:ident)+, op: $op:tt]),+ $(,)?) => {$(


        #[expect(non_snake_case)]
        mod ${concat(ඞඞ__,$inner_t,_,$trait_fn_name)} {
            use crate::base::Sure;

            const CODOMAIN<const A: &'static[$inner_t], const B: &'static[$inner_t]>: &[$inner_t] = const {
                &crate::const_helpers::array_from_fn::<$inner_t, { crate::set::CARTESIAN_LENGTH::<$inner_t, $inner_t, A, B> }>(
                    const |i| {
                        let b_len: usize = B.len();
                        let a_index: usize = i / b_len;
                        let b_index: usize = i % b_len;
                        let a: $inner_t = A[a_index];
                        let b: $inner_t = B[b_index];
                        a $op b
                    }
                )
            };

            const impl<const A_SET: &'static [$inner_t], const B_SET: &'static [$inner_t]> $(::$op_trait)+<Sure<$inner_t,B_SET> > for Sure<$inner_t, A_SET> {
                type Output = Sure<$inner_t, { CODOMAIN::<{ A_SET }, { B_SET }> }>;

                fn $trait_fn_name(self, rhs: Sure<$inner_t, B_SET>) -> Self::Output {
                    let self_inner: $inner_t = self.inner();
                    let rhs_inner: $inner_t = rhs.inner();
                    let res_inner: $inner_t = self_inner $op rhs_inner;
                    // SAFETY: something something cartesian product and pure function... TODO: make this less bad.
                    unsafe { Sure::new_unchecked(res_inner) }
                }
            }
        }
    )+}
}
pub(crate) use impl_simple_binary_ops;

macro_rules! impl_unary_fns {
    ($([fn $fn_name:ident($input_t:ident) -> $codomain_t:ident, fn_path: $fn_path:path, doc_macro_path: $doc_macro_path:ident]),+ $(,)?) => {$(

        #[expect(non_snake_case)]
        mod ${concat(ඞඞ__,$input_t,_,$fn_name)} {
            use crate::base::Sure;

            const CODOMAIN<const SET: &'static[$input_t]>: &[$codomain_t] = const {
                &crate::const_helpers::array_from_fn::<$codomain_t, { crate::set::LENGTH::<$input_t, SET> }>(
                    const |i| {
                        let a: $input_t = SET[i];
                        $fn_path(a)
                    }
                )
            };

            impl<const SET: &'static [$input_t]> Sure<$input_t, SET> {
                #[doc = crate::macros::$doc_macro_path!(fn_name: $fn_name, input_t: $input_t, codomain_t: $codomain_t)]
                #[must_use]
                pub const fn $fn_name(self) -> Sure<$codomain_t, { CODOMAIN::<{ SET }> }> {
                    let input_inner: $input_t = self.inner();
                    let output_inner: $codomain_t = $fn_path(input_inner);
                    // SAFETY: something something cartesian product and pure function... TODO: make this less bad.
                    unsafe { Sure::new_unchecked(output_inner) }
                }
            }
        }

    )+}
}
pub(crate) use impl_unary_fns;

macro_rules! impl_std_binary_fns {
    ($([fn $fn_name:ident($lhs_t:ident, $rhs_t:ident) -> $codomain_t:ty, fn_path: $fn_path:path]),+ $(,)?) => {$(

        #[expect(non_snake_case)]
        mod ${concat(ඞඞ__,$lhs_t,_,$rhs_t,_,$fn_name)} {
            use crate::base::Sure;

            const CODOMAIN<const LHS: &'static[$lhs_t], const RHS: &'static[$rhs_t]>: &[$codomain_t] = const {
                &crate::const_helpers::array_from_fn::<$codomain_t, { crate::set::CARTESIAN_LENGTH::<$lhs_t, $rhs_t, LHS, RHS> }>(
                    const |i| {
                        let rhs_len: usize = RHS.len();
                        let lhs_index: usize = i / rhs_len;
                        let rhs_index: usize = i % rhs_len;
                        let lhs: $lhs_t = LHS[lhs_index];
                        let rhs: $rhs_t = RHS[rhs_index];
                        $fn_path(lhs, rhs)
                    }
                )
            };

            impl<const LHS_SET: &'static [$lhs_t]> Sure<$lhs_t, LHS_SET> {
                #[doc = concat!("This method is the equivalent of [`",stringify!($lhs_t),"::",stringify!($fn_name),"`].")]
                #[must_use]
                pub const fn $fn_name<const RHS_SET: &'static [$rhs_t]>(self, rhs: Sure<$rhs_t, RHS_SET>) -> Sure<$codomain_t, { CODOMAIN::<{ LHS_SET }, { RHS_SET }> }> {
                    let lhs_inner: $lhs_t = self.inner();
                    let rhs_inner: $rhs_t = rhs.inner();
                    let output_inner: $codomain_t = $fn_path(lhs_inner, rhs_inner);
                    // SAFETY: something something cartesian product and pure function... TODO: make this less bad.
                    unsafe { Sure::new_unchecked(output_inner) }
                }
            }
        }

    )+}
}
pub(crate) use impl_std_binary_fns;

macro_rules! impl_ints {
    (the_dolla: $d:tt, $([num_t: $num_t:ident, unsigned_num_t: $unsigned_num_t:ident, signed_num_t: $signed_num_t:ident, t_alias: $t_alias:ident, wide_num_t: $wide_num_t:ident, private_macro_prefix: $private_macro_prefix:ident, extra_mod: $extra_mod:ident],)*) => {$(

        #[doc = concat!("A type alias for `Sure<",stringify!($num_t),", _>`.")]
        pub type $t_alias<const SET: &'static [$num_t]> = base::Sure<$num_t, SET>;

        #[doc = concat!("Helper consts for `",stringify!($t_alias),"`.")]
        pub mod $extra_mod {

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

                &core::array::from_fn::<$num_t, { RANGE_LENGTH_HELPER::<MIN, MAX, IS_INCLUSIVE> }, _>(
                    const |i| {
                        let wide_index = $wide_num_t::try_from(i).ok().expect("");
                        $num_t::try_from(wide_min + wide_index).ok().unwrap()
                    }
                )
            };

            #[doc = concat!("Creates a slice of`", stringify!($num_t), "`'s containing the numbers in `MIN..MAX`.")]
            pub const RANGE             <const MIN: $num_t, const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<                MIN ,                 MAX   , false >;
            #[doc = concat!("Creates a slice of`", stringify!($num_t), "`'s containing the numbers in `MIN..`.")]
            pub const RANGE_FROM        <const MIN: $num_t                   >: &[$num_t] = RANGE_HELPER::<                MIN ,{const { $num_t::MAX }}, true  >;
            // RANGE_FULL omitted
            #[doc = concat!("Creates a slice of`", stringify!($num_t), "`'s containing the numbers in `MIN..=MAX`.")]
            pub const RANGE_INCLUSIVE   <const MIN: $num_t, const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<                MIN ,                 MAX   , true  >;
            #[doc = concat!("Creates a slice of`", stringify!($num_t), "`'s containing the numbers in `..=MAX`.")]
            pub const RANGE_TO          <                   const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<{const{ $num_t::MIN }},               MAX   , false >;
            #[doc = concat!("Creates a slice of`", stringify!($num_t), "`'s containing the numbers in `..=MAX`.")]
            pub const RANGE_TO_INCLUSIVE<                   const MAX: $num_t>: &[$num_t] = RANGE_HELPER::<{const{ $num_t::MIN }},               MAX   , true  >;

            #[doc = concat!("A macro to make a slice of`", stringify!($num_t), "`'s using the familiar Range syntaxes.")]
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

            #[doc = concat!("A macro to make a union over of slices of`", stringify!($num_t), "`'s.")]
            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, union) } {
                ($d($set:expr),+ $d(,)?) => {
                    $d crate::set::UNION::<$num_t, { &[$d($set, )+] }>
                };
            }
            pub use ${ concat($private_macro_prefix, union) } as Union;

            #[doc = concat!("A macro to make a intersection over of slices of`", stringify!($num_t), "`'s.")]
            #[cfg_attr(doc, doc(hidden))]
            #[macro_export]
            macro_rules! ${ concat($private_macro_prefix, intersection) } {
                ($d($set:expr),+ $d(,)?) => {
                    $d crate::set::INTERSECTION::<$num_t, { &[$d($set, )+] }>
                };
            }
            pub use ${ concat($private_macro_prefix, intersection) } as Intersection;

            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_u8   (value: $num_t) -> u8    { value as u8    }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_u16  (value: $num_t) -> u16   { value as u16   }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_u32  (value: $num_t) -> u32   { value as u32   }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_u64  (value: $num_t) -> u64   { value as u64   }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_u128 (value: $num_t) -> u128  { value as u128  }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_usize(value: $num_t) -> usize { value as usize }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_i8   (value: $num_t) -> i8    { value as i8    }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_i16  (value: $num_t) -> i16   { value as i16   }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_i32  (value: $num_t) -> i32   { value as i32   }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_i64  (value: $num_t) -> i64   { value as i64   }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_i128 (value: $num_t) -> i128  { value as i128  }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
            pub(super) const fn as_isize(value: $num_t) -> isize { value as isize }

            pub(super) const fn to_u8   (value: $num_t) -> u8    { u8   ::try_from(value).ok().expect("Unable to losslessly convert to a u8"   ) }
            pub(super) const fn to_u16  (value: $num_t) -> u16   { u16  ::try_from(value).ok().expect("Unable to losslessly convert to a u16"  ) }
            pub(super) const fn to_u32  (value: $num_t) -> u32   { u32  ::try_from(value).ok().expect("Unable to losslessly convert to a u32"  ) }
            pub(super) const fn to_u64  (value: $num_t) -> u64   { u64  ::try_from(value).ok().expect("Unable to losslessly convert to a u64"  ) }
            pub(super) const fn to_u128 (value: $num_t) -> u128  { u128 ::try_from(value).ok().expect("Unable to losslessly convert to a u128" ) }
            pub(super) const fn to_usize(value: $num_t) -> usize { usize::try_from(value).ok().expect("Unable to losslessly convert to a usize") }
            pub(super) const fn to_i8   (value: $num_t) -> i8    { i8   ::try_from(value).ok().expect("Unable to losslessly convert to a i8"   ) }
            pub(super) const fn to_i16  (value: $num_t) -> i16   { i16  ::try_from(value).ok().expect("Unable to losslessly convert to a i16"  ) }
            pub(super) const fn to_i32  (value: $num_t) -> i32   { i32  ::try_from(value).ok().expect("Unable to losslessly convert to a i32"  ) }
            pub(super) const fn to_i64  (value: $num_t) -> i64   { i64  ::try_from(value).ok().expect("Unable to losslessly convert to a i64"  ) }
            pub(super) const fn to_i128 (value: $num_t) -> i128  { i128 ::try_from(value).ok().expect("Unable to losslessly convert to a i128" ) }
            pub(super) const fn to_isize(value: $num_t) -> isize { isize::try_from(value).ok().expect("Unable to losslessly convert to a isize") }

        }

        impl<const SET: &'static [$num_t]> crate::base::Sure<$num_t, SET> {
            #[doc = concat!("Creates a `NonZero<",stringify!($num_t),">`, or fails to compile if this could fail.")]
            pub const fn to_non_zero(self) -> core::num::NonZero<$num_t> {
                const { assert!(!Self::set_contains(&0), "Sure containing a 0 cannot be converted to NonZero")}
                let self_inner = self.inner();
                // SAFETY: we just asserted that `self_inner` can never be 0
                unsafe { core::num::NonZero::new_unchecked(self_inner) }
            }
        }

        #[doc = concat!("A convenience macro to macro writing types take fewer `<{[]}>` brackets.")]
        #[macro_export]
        macro_rules! $t_alias {
            ($elem:literal) => {
                $d crate::base::Sure::<$num_t, { &[$elem] }>
            };
            ($set:expr) => {
                $d crate::base::Sure::<$num_t, { $set }>
            };
            ($d($elem:expr),+ $d(,)?) => {
                $d crate::base::Sure::<$num_t, { &[$d($elem, )+] }>
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
