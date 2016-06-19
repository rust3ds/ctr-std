// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Linux-specific raw type definitions

#![allow(deprecated, unused_imports)]

use os::raw::c_ulong;

pub type dev_t = u64;
pub type mode_t = u32;

// #[unstable(feature = "pthread_t", issue = "29791")] pub type pthread_t = c_ulong;

#[doc(inline)]
pub use self::arch::{off_t, ino_t, nlink_t, blksize_t, blkcnt_t, stat, time_t};

#[cfg(any(target_arch = "x86",
          target_arch = "le32",
          target_arch = "powerpc",
          target_arch = "arm",
          target_arch = "asmjs"))]
mod arch {
    use os::raw::{c_long, c_short, c_uint};

    pub type blkcnt_t = u64;
    pub type blksize_t = u64;
    pub type ino_t = u64;
    pub type nlink_t = u64;
    pub type off_t = u64;
    pub type time_t = i64;

    #[repr(C)]
    #[derive(Clone)]
    pub struct stat {
        pub st_dev: u64,
        pub __pad1: c_short,
        pub __st_ino: u32,
        pub st_mode: u32,
        pub st_nlink: u32,
        pub st_uid: u32,
        pub st_gid: u32,
        pub st_rdev: u64,
        pub __pad2: c_uint,
        pub st_size: i64,
        pub st_blksize: i32,
        pub st_blocks: i64,
        pub st_atime: i32,
        pub st_atime_nsec: c_long,
        pub st_mtime: i32,
        pub st_mtime_nsec: c_long,
        pub st_ctime: i32,
        pub st_ctime_nsec: c_long,
        pub st_ino: u64,
    }
}

#[cfg(target_arch = "mips")]
mod arch {
    use os::raw::{c_long, c_ulong};

    #[cfg(target_env = "musl")]
    pub type blkcnt_t = i64;
    #[cfg(not(target_env = "musl"))]
    pub type blkcnt_t = u64;
    pub type blksize_t = u64;
    #[cfg(target_env = "musl")]
    pub type ino_t = u64;
    #[cfg(not(target_env = "musl"))]
    pub type ino_t = u64;
    pub type nlink_t = u64;
    #[cfg(target_env = "musl")]
    pub type off_t = u64;
    #[cfg(not(target_env = "musl"))]
    pub type off_t = u64;
    pub type time_t = i64;

    #[repr(C)]
    #[derive(Clone)]
    pub struct stat {
        pub st_dev: c_ulong,
        pub st_pad1: [c_long; 3],
        pub st_ino: u64,
        pub st_mode: u32,
        pub st_nlink: u32,
        pub st_uid: u32,
        pub st_gid: u32,
        pub st_rdev: c_ulong,
        pub st_pad2: [c_long; 2],
        pub st_size: i64,
        pub st_atime: i32,
        pub st_atime_nsec: c_long,
        pub st_mtime: i32,
        pub st_mtime_nsec: c_long,
        pub st_ctime: i32,
        pub st_ctime_nsec: c_long,
        pub st_blksize: i32,
        pub st_blocks: i64,
        pub st_pad5: [c_long; 14],
    }
}

#[cfg(target_arch = "aarch64")]
mod arch {
    use os::raw::{c_long, c_int};

    pub type blkcnt_t = u64;
    pub type blksize_t = u64;
    pub type ino_t = u64;
    pub type nlink_t = u64;
    pub type off_t = u64;
    pub type time_t = i64;

    #[repr(C)]
    #[derive(Clone)]
    pub struct stat {
        pub st_dev: u64,
        pub st_ino: u64,
        pub st_mode: u32,
        pub st_nlink: u32,
        pub st_uid: u32,
        pub st_gid: u32,
        pub st_rdev: u64,
        pub __pad1: u64,
        pub st_size: i64,
        pub st_blksize: i32,
        pub __pad2: c_int,
        pub st_blocks: i64,
        pub st_atime: i64,
        pub st_atime_nsec: c_long,
        pub st_mtime: i64,
        pub st_mtime_nsec: c_long,
        pub st_ctime: i64,
        pub st_ctime_nsec: c_long,
        pub __unused: [c_int; 2],
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc64"))]
mod arch {
    use os::raw::{c_long, c_int};

    pub type blkcnt_t = u64;
    pub type blksize_t = u64;
    pub type ino_t = u64;
    pub type nlink_t = u64;
    pub type off_t = u64;
    pub type time_t = i64;

    #[repr(C)]
    #[derive(Clone)]
    pub struct stat {
        pub st_dev: u64,
        pub st_ino: u64,
        pub st_nlink: u64,
        pub st_mode: u32,
        pub st_uid: u32,
        pub st_gid: u32,
        pub __pad0: c_int,
        pub st_rdev: u64,
        pub st_size: i64,
        pub st_blksize: i64,
        pub st_blocks: i64,
        pub st_atime: i64,
        pub st_atime_nsec: c_long,
        pub st_mtime: i64,
        pub st_mtime_nsec: c_long,
        pub st_ctime: i64,
        pub st_ctime_nsec: c_long,
        pub __unused: [c_long; 3],
    }
}
