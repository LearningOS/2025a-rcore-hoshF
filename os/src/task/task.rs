//! Types related to task management

use super::TaskContext;

pub const MAX_SYSCALL_NUM: usize = 20;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize,
}

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// task id
    pub id: usize,
    /// syscall info
    pub syscall_status: [SyscallInfo; MAX_SYSCALL_NUM],
    /// running time
    pub running_time: usize,
    /// start time
    pub start_time: usize,
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
