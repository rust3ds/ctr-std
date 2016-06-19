// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// // file at the top-level directory of this distribution and at
// // http://rust-lang.org/COPYRIGHT.
// //
// // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// // option. This file may not be copied, modified, or distributed
// // except according to those terms.
//
// #![allow(missing_docs, bad_style)]

use io::{self, ErrorKind};
use libc;
use num::One;
use ops::Neg;

#[cfg(target_os = "linux")] pub use os::linux as platform;

pub use libc::signal;

pub mod ext;
pub mod fd;
pub mod fs;
pub mod os;
pub mod os_str;
pub mod time;

pub fn decode_error_kind(errno: i32) -> ErrorKind {
    match errno as libc::c_int {
        libc::ECONNREFUSED => ErrorKind::ConnectionRefused,
        libc::ECONNRESET => ErrorKind::ConnectionReset,
        libc::EPERM | libc::EACCES => ErrorKind::PermissionDenied,
        libc::EPIPE => ErrorKind::BrokenPipe,
        libc::ENOTCONN => ErrorKind::NotConnected,
        libc::ECONNABORTED => ErrorKind::ConnectionAborted,
        libc::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        libc::EADDRINUSE => ErrorKind::AddrInUse,
        libc::ENOENT => ErrorKind::NotFound,
        libc::EINTR => ErrorKind::Interrupted,
        libc::EINVAL => ErrorKind::InvalidInput,
        libc::ETIMEDOUT => ErrorKind::TimedOut,
        libc::EEXIST => ErrorKind::AlreadyExists,

        // These two constants can have the same value on some systems,
        // but different values on others, so we can't use a match
        // clause
        x if x == libc::EAGAIN || x == libc::EWOULDBLOCK => ErrorKind::WouldBlock,

        _ => ErrorKind::Other,
    }
}

pub fn cvt<T: One + PartialEq + Neg<Output = T>>(t: T) -> io::Result<T> {
    let one: T = T::one();
    if t == -one {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

pub fn cvt_r<T, F>(mut f: F) -> io::Result<T>
    where T: One + PartialEq + Neg<Output = T>,
          F: FnMut() -> T
{
    loop {
        match cvt(f()) {
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            other => return other,
        }
    }
}
