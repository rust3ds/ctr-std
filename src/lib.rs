#![feature(alloc)]
#![feature(allow_internal_unstable)]
#![feature(collections)]
#![feature(core_intrinsics)]
#![feature(fmt_internals)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(prelude_import)]
#![feature(rand)]
#![feature(raw)]
#![feature(slice_concat_ext)]
#![feature(unicode)]
#![feature(unwind_attributes)]
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

pub mod panicking;

#[prelude_import]
pub mod prelude;
