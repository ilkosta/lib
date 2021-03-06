// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod tls {
    use {AtExit};
    use lock::{SingleThreadMutex};

    pub unsafe fn init() {
        // Managed by libc
    }

    thread_local! {
        static AT_EXIT: SingleThreadMutex<AtExit> = SingleThreadMutex::new(AtExit::new());
    }

    pub fn at_exit() -> &'static SingleThreadMutex<AtExit> {
        &AT_EXIT
    }
}
