// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_fd"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_rv as rv;
extern crate lrs_io as io;

#[prelude_import] use base::prelude::*;
use io::{Write, Read};
use cty::{c_int};
use syscall::{writev, readv};
use rv::{retry};

mod lrs { pub use base::lrs::*; }

/// FdIo wrapping `0`.
pub const STDIN: FdIo = FdIo(0);

/// FdIo wrapping `1`.
pub const STDOUT: FdIo = FdIo(1);

/// FdIo wrapping `2`.
pub const STDERR: FdIo = FdIo(2);

/// A read/write wrapper for raw file descriptors.
///
/// [field, 1]
/// The numeric value of the file descriptor.
#[derive(Pod, Eq)]
pub struct FdIo(pub c_int);

impl Write for FdIo {
    fn gather_write(&mut self, buf: &[&[u8]]) -> Result<usize> {
        retry(|| writev(self.0, buf)).map(|r| r as usize)
    }
}

impl Read for FdIo {
    fn scatter_read(&mut self, buf: &mut [&mut [u8]]) -> Result<usize> {
        retry(|| readv(self.0, buf)).map(|r| r as usize)
    }
}

/// Objects that are file descriptor wrappers.
pub trait FDContainer {
    /// Consumes the object and returns the file descriptor without closing it.
    fn unwrap(self) -> c_int;

    /// Returns whether the object owns the file descriptor, i.e., whether it closes it
    /// when it goes out of scope.
    fn is_owned(&self) -> bool;

    /// Returns the contained file descriptor.
    fn borrow(&self) -> c_int;

    /// Creates a new owned object from a file descriptor.
    ///
    /// [argument, fd]
    /// The value of the file descriptor.
    ///
    /// = Remarks
    ///
    /// Not all objects support owned file descriptors. Check with `is_owned` if this
    /// matters.
    fn from_owned(fd: c_int) -> Self;

    /// Creates a new borrowed object from a file descriptor.
    ///
    /// [argument, fd]
    /// The value of the file descriptor.
    fn from_borrowed(fd: c_int) -> Self;
}

impl FDContainer for FdIo {
    fn unwrap(self) -> c_int { self.0 }
    fn is_owned(&self) -> bool { false }
    fn borrow(&self) -> c_int { self.0 }
    fn from_owned(fd: c_int) -> FdIo { FdIo(fd) }
    fn from_borrowed(fd: c_int) -> FdIo { FdIo(fd) }
}

impl FDContainer for c_int {
    fn unwrap(self) -> c_int { self }
    fn is_owned(&self) -> bool { false }
    fn borrow(&self) -> c_int { *self }
    fn from_owned(fd: c_int) -> c_int { fd }
    fn from_borrowed(fd: c_int) -> c_int { fd }
}
