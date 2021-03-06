// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Process handling.

pub use lrs_cty::alias::{ProcessId};
pub use lrs_process::{
    process_id, parent_process_id, exit, set_file_mask, Times, used_time, new_session,
    session, set_process_group, process_group, resource_usage, ResourceUsage,
    resource_limit, set_resource_limit,
};
pub use lrs_process::res::{Resource};
pub use lrs_process::res_user::{ResourceUser};
pub use lrs_process::exec::{exec};
pub use lrs_process::wait::{
    ChildStatus, WaitFlags, WAIT_EXITED, WAIT_STOPPED, WAIT_CONTINUED, WAIT_DONT_BLOCK,
    WAIT_DONT_REAP, wait_all, wait_id,
};
pub use lrs_clone::{fork};
pub use lrs_clone::flags::{CloneFlags};

pub mod clone {
    pub use lrs_clone::flags::{
        CLONE_VM, CLONE_FS, CLONE_FILES, CLONE_SIGHAND, CLONE_PTRACE, CLONE_VFORK,
        CLONE_PARENT, CLONE_THREAD, CLONE_NEWMOUNT, CLONE_SYSVSEM, CLONE_SETTLS,
        CLONE_UNTRACED, CLONE_NEWUTS, CLONE_NEWIPC, CLONE_NEWUSER, CLONE_NEWPID,
        CLONE_NEWNET, CLONE_IO,
    };
}

pub mod res_user {
    pub use lrs_process::res_user::{
        Process, Children, Thread,
    };
}

pub mod resource {
    pub use lrs_process::res::{
        VirtualMemory, CoreDumpSize, CpuTime, ContiguousCpuTime, DataSegment, FileSize,
        LockedMemory, MsgQueue, Niceness, FileDescriptors, Processes, Priority,
        PendingSignals, Stack,
    };
}
