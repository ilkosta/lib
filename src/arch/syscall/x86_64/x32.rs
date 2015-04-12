// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{
    c_longlong, __X32_SYSCALL_BIT
};

/// Syscall type
pub type SCT = c_longlong;

#[inline(always)]
pub unsafe fn syscall0(n: SCT) -> SCT {
    ::syscall::arch::common::syscall0(n + __X32_SYSCALL_BIT)
}

#[inline(always)]
pub unsafe fn syscall1(n: SCT) -> SCT {
    ::syscall::arch::common::syscall1(n + __X32_SYSCALL_BIT, a1)
}

#[inline(always)]
pub unsafe fn syscall2(n: SCT) -> SCT {
    ::syscall::arch::common::syscall2(n + __X32_SYSCALL_BIT, a1, a2)
}

#[inline(always)]
pub unsafe fn syscall3(n: SCT) -> SCT {
    ::syscall::arch::common::syscall3(n + __X32_SYSCALL_BIT, a1, a2, a3)
}

#[inline(always)]
pub unsafe fn syscall4(n: SCT) -> SCT {
    ::syscall::arch::common::syscall4(n + __X32_SYSCALL_BIT, a1, a2, a3, a4)
}

#[inline(always)]
pub unsafe fn syscall5(n: SCT) -> SCT {
    ::syscall::arch::common::syscall5(n + __X32_SYSCALL_BIT, a1, a2, a3, a4, a5)
}

#[inline(always)]
pub unsafe fn syscall6(n: SCT) -> SCT {
    ::syscall::arch::common::syscall6(n + __X32_SYSCALL_BIT, a1, a2, a3, a4, a5, a6)
}
