//! Refinement types.

#![feature(adt_const_params)]
#![feature(const_array)]
#![feature(const_binary_search)]
#![feature(const_clone)]
#![feature(const_closures)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_heap)]
#![feature(const_index)]
#![feature(const_ops)]
#![feature(const_result_trait_fn)]
#![feature(const_trait_impl)]
#![feature(core_intrinsics)]
#![feature(freeze)]
#![feature(generic_const_args)]
#![feature(generic_const_items)]
#![feature(generic_const_parameter_types)]
#![feature(macro_metavar_expr_concat)]
#![feature(macroless_generic_const_args)]
#![feature(min_generic_const_args)]
#![feature(unsized_const_params)]
//
#![deny(missing_docs)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
//
#![allow(clippy::match_bool)]
#![allow(incomplete_features)]
#![allow(internal_features)]
#![allow(long_running_const_eval)]
//
#![no_std]
//
extern crate alloc;

mod bendn_sort;
mod const_helpers;
mod macros;
mod spec;

/// Defines the `Sure` type and its methods.
pub mod base;

/// Helpers for common set operations.
pub mod set;

/// The `SureEq` trait and it's implementations which this crate relies on to make some operations sound.
pub mod sure_eq;

macros::impl_ints! {
    the_dolla: $,
    [num_t: u8,     unsigned_num_t: u8,      signed_num_t: i8,      t_alias: SureU8,     wide_num_t: u16,     private_macro_prefix: ඞඞ__private_macro_sure_u8_,    extra_mod: sure_u8    ],
    [num_t: u16,    unsigned_num_t: u16,     signed_num_t: i16,     t_alias: SureU16,    wide_num_t: u32,     private_macro_prefix: ඞඞ__private_macro_sure_u16_,   extra_mod: sure_u16   ],
    [num_t: u32,    unsigned_num_t: u32,     signed_num_t: i32,     t_alias: SureU32,    wide_num_t: u64,     private_macro_prefix: ඞඞ__private_macro_sure_u32_,   extra_mod: sure_u32   ],
    [num_t: u64,    unsigned_num_t: u64,     signed_num_t: i64,     t_alias: SureU64,    wide_num_t: u128,    private_macro_prefix: ඞඞ__private_macro_sure_u64_,   extra_mod: sure_u64   ],
    [num_t: u128,   unsigned_num_t: u128,    signed_num_t: i128,    t_alias: SureU128,   wide_num_t: u128,    private_macro_prefix: ඞඞ__private_macro_sure_u128_,  extra_mod: sure_u128  ],
    [num_t: usize,  unsigned_num_t: usize,   signed_num_t: isize,   t_alias: SureUsize,  wide_num_t: usize,   private_macro_prefix: ඞඞ__private_macro_sure_usize_, extra_mod: sure_usize ],
    [num_t: i8,     unsigned_num_t: u8,      signed_num_t: i8,      t_alias: SureI8,     wide_num_t: i16,     private_macro_prefix: ඞඞ__private_macro_sure_i8_,    extra_mod: sure_i8    ],
    [num_t: i16,    unsigned_num_t: u16,     signed_num_t: i16,     t_alias: SureI16,    wide_num_t: i32,     private_macro_prefix: ඞඞ__private_macro_sure_i16_,   extra_mod: sure_i16   ],
    [num_t: i32,    unsigned_num_t: u32,     signed_num_t: i32,     t_alias: SureI32,    wide_num_t: i64,     private_macro_prefix: ඞඞ__private_macro_sure_i32_,   extra_mod: sure_i32   ],
    [num_t: i64,    unsigned_num_t: u64,     signed_num_t: i64,     t_alias: SureI64,    wide_num_t: i128,    private_macro_prefix: ඞඞ__private_macro_sure_i64_,   extra_mod: sure_i64   ],
    [num_t: i128,   unsigned_num_t: u128,    signed_num_t: i128,    t_alias: SureI128,   wide_num_t: i128,    private_macro_prefix: ඞඞ__private_macro_sure_i128_,  extra_mod: sure_i128  ],
    [num_t: isize,  unsigned_num_t: usize,   signed_num_t: isize,   t_alias: SureIsize,  wide_num_t: isize,   private_macro_prefix: ඞඞ__private_macro_sure_isize_, extra_mod: sure_isize ],
}
