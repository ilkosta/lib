// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use atomic::{AtomicU8};

const UNINITIALIZED: u8 = 0;
const WORKING:       u8 = 1;
const INITIALIZED:   u8 = 2;

/// The status of a once.
pub enum OnceStatus {
    Uninitialized,
    Working,
    Initialized,
}

#[repr(C)]
pub struct Once {
    status: AtomicU8,
}

impl<'a> Once {
    /// Creates a new, uninitialized, once.
    pub const fn new() -> Once {
        Once { status: AtomicU8::new(UNINITIALIZED) }
    }

    /// Returns the status of the once.
    pub fn status(&self) -> OnceStatus {
        match self.status.load_unordered() {
            UNINITIALIZED => OnceStatus::Uninitialized,
            WORKING       => OnceStatus::Working,
            _             => OnceStatus::Initialized,
        }
    }

    pub fn once<F>(&self, f: F) -> bool
        where F: FnOnce(),
    {
        let mut status = self.status.load_acquire();
        if status == INITIALIZED {
            return false;
        }
        if status == UNINITIALIZED {
            status = self.status.compare_exchange(UNINITIALIZED, WORKING);
        }
        if status == UNINITIALIZED {
            f();
            self.status.store_release(INITIALIZED);
            return true;
        }
        while status == WORKING {
            status = self.status.load_acquire();
        }
        false
    }
}
