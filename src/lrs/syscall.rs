// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Syscall wrappers.

pub use lrs_syscall::{
    openat, close, lseek, fcntl_dupfd_cloexec, fcntl_getfl, fcntl_setfl, fcntl_getfd,
    fcntl_setfd, ftruncate, getpid, getppid, setresuid, setresgid, fsync, fdatasync, sync,
    syncfs, fadvise, fchmod, fallocate, timerfd_create, epoll_create, flock, readahead,
    read, write, pread, pwrite, readv, writev, preadv, pwritev, getresuid, getresgid,
    getgroups, setgroups, statfs, fstatfs, prlimit, getdents, fstatat, faccessat,
    truncate, linkat, utimensat, renameat2, mkdirat, unlinkat, symlinkat, readlinkat,
    fchownat, fchmodat, mknodat, setxattr, lsetxattr, fsetxattr, getxattr, lgetxattr,
    fgetxattr, removexattr, lremovexattr, fremovexattr, listxattr, llistxattr, flistxattr,
    clock_getres, clock_gettime, clock_settime, clock_nanosleep, timerfd_settime,
    timerfd_gettime, epoll_ctl, epoll_pwait, sched_getaffinity, uname, sysinfo, getrandom,
    acct, mount, umount, sethostname, setdomainname, socket, connect, accept4, recvfrom,
    recvmsg, recvmmsg, sendto, sendmsg, sendmmsg, shutdown, bind, listen, getsockname,
    getpeername, socketpair, setsockopt, getsockopt, futex_wait, futex_wake, exit,
    exit_group, execveat, mmap, munmap, mremap, waitid, getcwd, chdir, ioctl_siocgstampns,
    ioctl_siocinq, ioctl_siocoutq,
};