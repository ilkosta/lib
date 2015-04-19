// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Experimental Linux standard library.
//!
//! This library tries to create a rust standard library on top of the Linux API. It is
//! not bound by any other standards. In particular, it does not try to create a
//! POSIX-like api or an API that can easily be ported to other platforms.
//!
//! Currently only `x86_64` is supported.

#![crate_name = "linux"]
#![crate_type = "lib"]
#![feature(plugin, no_std, macro_reexport)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
#[macro_reexport(abort, assert, try, println, matches, vec, format)]
extern crate linux_core;
extern crate linux_base;

extern crate linux_dev;
extern crate linux_file;
// extern crate linux_user_group;
// extern crate linux_core;
extern crate linux_sys;
extern crate linux_time_base;
extern crate linux_time_ext;
extern crate linux_dir;
extern crate linux_fs;
extern crate linux_process;
extern crate linux_poll;

pub use linux_core as core;
pub use linux_core::{intrinsics};
pub use linux_base::{fmt, rmo, path, string, ops, parse};
pub mod file;
pub mod stdio;
//pub mod user;
//pub mod group;
//pub mod errno;
pub mod result;
pub mod dir;
pub mod fs;
pub mod process;
//pub mod string;
pub mod time;
pub mod vec;
//pub mod path;
pub mod poll;
pub mod sys;
//pub mod fd;

pub mod prelude {
    pub use linux_base::prelude::*;
    pub use linux_base::parse::{Parse};
}
