// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem, slice};

/// Objects that can be immutably borrowed.
pub trait AsRef<Target: ?Sized> {
    /// Borrows the object.
    fn as_ref(&self) -> &Target;
}

/// Objects that can be mutably borrowed.
pub trait AsMut<Target: ?Sized> {
    /// Borrows the object.
    fn as_mut(&mut self) -> &mut Target;
}

impl<T> AsRef<T> for T {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> AsMut<T> for T {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: Pod> AsRef<[u8]> for [T] {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<T: Pod> AsMut<[u8]> for [T] {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_bytes()
    }
}

impl<T: Pod> AsRef<[u8]> for T {
    fn as_ref(&self) -> &[u8] {
        mem::as_bytes(self)
    }
}

impl<T: Pod> AsMut<[u8]> for T {
    fn as_mut(&mut self) -> &mut [u8] {
        mem::as_mut_bytes(self)
    }
}

impl AsRef<[u8]> for str {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<[u8]> for char {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_ptr(self as *const _ as *const _, mem::size_of::<char>())
        }
    }
}

impl AsRef<[u8]> for [char] {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            let ptr = self.as_ptr();
            let size = mem::size_of::<char>() * self.len();
            slice::from_ptr(ptr as *const _, size)
        }
    }
}
