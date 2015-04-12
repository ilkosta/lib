// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use ::syscall::arch::abi::{
    syscall0, syscall1, syscall2, syscall3, syscall4, syscall5, syscall6, SCT,
};

pub use ::syscall::common::{
    accept4, accept, access, acct, add_key, adjtimex, alarm, bdflush, bind, bpf, brk,
    capget, capset, chdir, chmod, chown16, chown, chroot, clock_adjtime, clock_getres,
    clock_gettime, clock_nanosleep, clock_settime, close, connect, creat, delete_module,
    dup2, dup3, dup, epoll_create1, epoll_create, epoll_ctl, epoll_pwait, epoll_wait,
    eventfd2, eventfd, execveat, execve, exit, exit_group, faccessat, fadvise64_64,
    fadvise64, fallocate, fanotify_init, fanotify_mark, fchdir, fchmodat, fchmod,
    fchown16, fchownat, fchown, fcntl64, fcntl, fdatasync, fgetxattr, finit_module,
    flistxattr, flock, fork, fremovexattr, fsetxattr, fstat64, fstatat64, fstat,
    fstatfs64, fstatfs, fsync, ftruncate64, ftruncate, futex, futimesat, getcpu, getcwd,
    getdents64, getdents, getegid, getegid16, geteuid, geteuid16, getgid, getgid16,
    getgroups16, getgroups, gethostname, getitimer, get_mempolicy, getpeername, getpgid,
    getpgrp, getpid, getppid, getpriority, getrandom, getresgid16, getresgid, getresuid16,
    getresuid, getrlimit, get_robust_list, getrusage, getsid, getsockname, getsockopt,
    gettid, gettimeofday, getuid, getuid16, getxattr, init_module, inotify_add_watch,
    inotify_init, inotify_init1, inotify_rm_watch, io_cancel, ioctl, io_destroy,
    io_getevents, ioprio_get, ioprio_set, io_setup, io_submit, ipc, kcmp, kexec_file_load,
    kexec_load, keyctl, kill, lchown16, lchown, lgetxattr, linkat, link, listen,
    listxattr, llistxattr, llseek, lookup_dcookie, lremovexattr, lseek, lsetxattr,
    lstat64, lstat, madvise, mbind, memfd_create, migrate_pages, mincore, mkdirat, mkdir,
    mknodat, mknod, mlockall, mlock, mount, move_pages, mprotect,
    mq_getsetattr, mq_notify, mq_open, mq_timedreceive, mq_timedsend, mq_unlink, mremap,
    msgctl, msgget, msgrcv, msgsnd, msync, munlockall, munlock, munmap, name_to_handle_at,
    nanosleep, newfstatat, newfstat, newlstat, newstat, newuname, nice, old_getrlimit,
    old_mmap, old_readdir, old_select, oldumount, olduname, openat, open_by_handle_at,
    open, pause, pciconfig_read, pciconfig_write, perf_event_open, personality, pipe2,
    pipe, pivot_root, poll, ppoll, prctl, pread64, preadv, prlimit64, process_vm_readv,
    process_vm_writev, pselect6, ptrace, pwrite64, pwritev, quotactl, readahead, read,
    readlinkat, readlink, readv, reboot, recv, recvfrom, recvmmsg, recvmsg,
    remap_file_pages, removexattr, renameat2, renameat, rename, request_key,
    restart_syscall, rmdir, rt_sigaction, rt_sigpending, rt_sigprocmask, rt_sigqueueinfo,
    rt_sigsuspend, rt_sigtimedwait, rt_tgsigqueueinfo, sched_getaffinity, sched_getattr,
    sched_getparam, sched_get_priority_max, sched_get_priority_min, sched_getscheduler,
    sched_rr_get_interval, sched_setaffinity, sched_setattr, sched_setparam,
    sched_setscheduler, sched_yield, seccomp, select, semctl, semget, semop, semtimedop,
    send, sendfile64, sendfile, sendmmsg, sendmsg, sendto, setdomainname, setfsgid16,
    setfsgid, setfsuid16, setfsuid, setgid16, setgid, setgroups16, setgroups, sethostname,
    setitimer, set_mempolicy, setns, setpgid, setpriority, setregid16, setregid,
    setresgid16, setresgid, setresuid16, setresuid, setreuid16, setreuid, setrlimit,
    set_robust_list, setsid, setsockopt, set_tid_address, settimeofday, setuid16, setuid,
    setxattr, sgetmask, shmat, shmctl, shmdt, shmget, shutdown, sigaction, sigaltstack,
    signalfd4, signalfd, signal, sigpending, sigprocmask, socketcall, socket, socketpair,
    splice, ssetmask, stat64, stat, statfs64, statfs, stime, swapoff, swapon, symlinkat,
    symlink, sync, sync_file_range2, sync_file_range, syncfs, sysctl, sysfs, sysinfo,
    syslog, tee, tgkill, timer_create, timer_delete, timerfd_create, timerfd_gettime,
    timerfd_settime, timer_getoverrun, timer_gettime, timer_settime, times, time, tkill,
    truncate64, truncate, umask, umount, uname, unlinkat, unlink, unshare, uselib, ustat,
    utime, utimensat, utimes, vfork, vhangup, vmsplice, wait4, waitid, waitpid, write,
    writev,
};

use cty::{
};

#[cfg(target_pointer_width = "32")]
#[path = "x32.rs"]
mod abi;

#[cfg(target_pointer_width = "64")]
#[path = "x64.rs"]
mod abi;

mod common {
    #[inline(always)]
    pub unsafe fn syscall0(n: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall1(n: SCT, a1: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall2(n: SCT, a1: SCT, a2: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall3(n: SCT, a1: SCT, a2: SCT, a3: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall4(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                         "{r10}"(a4)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall5(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                         "{r10}"(a4), "{r8}"(a5)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall6(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT,
                           a6: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                         "{r10}"(a4), "{r8}"(a5), "{r9}"(a6)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }
}

pub unsafe fn iopl(level: c_uint) -> k_int {
    call!(__NR_iopl, level) as k_int
}

pub unsafe fn set_thread_area(u_info: *mut user_desc) -> k_int {
    call!(__NR_set_thread_area, u_info) as k_int
}

pub unsafe fn get_thread_area(u_info: *mut user_desc) -> k_int {
    call!(__NR_get_thread_area, u_info) as k_int
}

pub unsafe fn mmap(addr: k_ulong, len: k_ulong, prot: k_ulong, flags: k_ulong,
                   fd: k_ulong, off: k_ulong) -> k_long {
    call!(__NR_mmap, addr, len, prot, flags, fd, off) as k_long
}

pub unsafe fn vm86old(v86: *mut vm86_struct) -> k_int {
    call!(__NR_vm86old, v86) as k_int
}

pub unsafe fn vm86(cmd: k_ulong, arg: k_ulong) -> k_int {
    call!(__NR_vm86, cmd, arg) as k_int
}
