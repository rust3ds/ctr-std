// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unix-specific extension to the primitives in the `std::ffi` module

use ffi::{OsStr, OsString};
use mem;
use prelude::v1::*;
use sys::os_str::Buf;
use sys_common::{FromInner, IntoInner, AsInner};

/// Unix-specific extensions to `OsString`.
pub trait OsStringExt {
    /// Creates an `OsString` from a byte vector.
    fn from_vec(vec: Vec<u8>) -> Self;

    /// Yields the underlying byte vector of this `OsString`.
    fn into_vec(self) -> Vec<u8>;
}

impl OsStringExt for OsString {
    fn from_vec(vec: Vec<u8>) -> OsString {
        FromInner::from_inner(Buf { inner: vec })
    }
    fn into_vec(self) -> Vec<u8> {
        self.into_inner().inner
    }
}

/// Unix-specific extensions to `OsStr`.
pub trait OsStrExt {
    fn from_bytes(slice: &[u8]) -> &Self;

    /// Gets the underlying byte view of the `OsStr` slice.
    fn as_bytes(&self) -> &[u8];
}

impl OsStrExt for OsStr {
    fn from_bytes(slice: &[u8]) -> &OsStr {
        unsafe { mem::transmute(slice) }
    }
    fn as_bytes(&self) -> &[u8] {
        &self.as_inner().inner
    }
}
