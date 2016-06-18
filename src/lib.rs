#![feature(alloc)]
#![feature(allow_internal_unstable)]
#![feature(collections)]
#![feature(collections_bound)]
#![feature(core_intrinsics)]
#![feature(dropck_parametricity)]
#![feature(fmt_internals)]
#![feature(filling_drop)]
#![feature(float_extras)]
#![feature(heap_api)]
#![feature(int_error_internals)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(oom)]
#![feature(prelude_import)]
#![feature(rand)]
#![feature(raw)]
#![feature(reflect_marker)]
#![feature(slice_concat_ext)]
#![feature(start)]
#![feature(stmt_expr_attributes)]
#![feature(try_from)]
#![feature(type_ascription)]
#![feature(unicode)]
#![feature(unique)]
#![feature(unsafe_no_drop_flag)]
#![feature(unwind_attributes)]
#![feature(zero_one)]
#![no_std]

#[macro_reexport(assert, assert_eq, debug_assert, debug_assert_eq,
                 unreachable, unimplemented, write, writeln)]
extern crate core as __core;

#[macro_use]
#[macro_reexport(vec, format)]
extern crate collections as core_collections;

#[allow(deprecated)]
extern crate rand;
extern crate alloc;
extern crate rustc_unicode;
extern crate libc;

extern crate alloc_system;

pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::intrinsics;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::ptr;
pub use core::raw;
pub use core::result;
pub use core::option;

pub mod error;

pub use alloc::arc;
pub use alloc::boxed;
pub use alloc::rc;

pub use core_collections::borrow;
pub use core_collections::fmt;
pub use core_collections::slice;
pub use core_collections::str;
pub use core_collections::string;
pub use core_collections::vec;

pub use rustc_unicode::char;

#[macro_use]
pub mod macros;

#[prelude_import]
pub mod prelude;

pub use core::isize;
pub use core::i8;
pub use core::i16;
pub use core::i32;
pub use core::i64;

pub use core::usize;
pub use core::u8;
pub use core::u16;
pub use core::u32;
pub use core::u64;

#[path = "num/f32.rs"] pub mod f32;
#[path = "num/f64.rs"] pub mod f64;

pub mod ascii;

pub mod num;

pub mod collections;
pub mod ffi;
mod memchr;

#[macro_use]
#[path = "sys/common/mod.rs"] mod sys_common;

pub mod panicking;
pub mod rt;
