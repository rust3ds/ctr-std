// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of `std::os` functionality for unix systems

#![allow(dead_code, unused_imports)] // lots of cfg code here

use prelude::v1::*;
use os::unix::prelude::*;

use error::Error as StdError;
use ffi::{CString, CStr, OsString, OsStr};
use fmt;
use io;
use iter;
use libc::{self, c_int, c_char, c_void};
use mem;
use memchr;
use path::{self, PathBuf};
use ptr;
use slice;
use str;
// use sys_common::mutex::Mutex;
use sys::cvt;
use sys::fd;
use vec;

const TMPBUF_SZ: usize = 128;
// static ENV_LOCK: Mutex = Mutex::new();

#[cfg(not(target_os = "dragonfly"))]
pub fn errno() -> i32 {
    #[cfg_attr(any(target_os = "linux", target_os = "emscripten"),
               link_name = "__errno_location")]
    #[cfg_attr(any(target_os = "bitrig",
                   target_os = "netbsd",
                   target_os = "openbsd",
                   target_os = "android",
                   target_env = "newlib"),
                        link_name = "__errno")]
    #[cfg_attr(target_os = "solaris", link_name = "___errno")]
    #[cfg_attr(any(target_os = "macos",
                                  target_os = "ios",
                                  target_os = "freebsd"),
                              link_name = "__error")]
    extern "C" {
        fn __errno() -> *const c_int;
    }

    unsafe { (*__errno()) as i32 }
}

/// Gets a detailed string description for the given error number.
pub fn error_string(errno: i32) -> String {
    extern "C" {
                    #[cfg_attr(any(target_os = "linux", target_env = "newlib"),
                                       link_name = "__xpg_strerror_r")]
        fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int;
    }

    let mut buf = [0 as c_char; TMPBUF_SZ];

    let p = buf.as_mut_ptr();
    unsafe {
        if strerror_r(errno as c_int, p, buf.len() as libc::size_t) < 0 {
            panic!("strerror_r failure");
        }

        let p = p as *const _;
        str::from_utf8(CStr::from_ptr(p).to_bytes()).unwrap().to_owned()
    }
}

pub fn exit(code: i32) -> ! {
    unsafe { libc::exit(code as c_int) }
}
