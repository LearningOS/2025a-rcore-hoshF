//! Process management syscalls
use crate::{
    task::{
        exit_current_and_run_next, suspend_current_and_run_next, SyscallInfo, TaskStatus,
        MAX_INFO_NUM, TASK_MANAGER,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task info content
#[repr(C)]
#[derive(Debug)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    -1
}

pub fn sys_task_info(id: usize, ts: *mut TaskInfo) -> isize {
    trace!("Kernel: sys_task_info");
    let mut inner = TASK_MANAGER.inner.exclusive_access();

    if id >= TASK_MANAGER.num_app {
        return -1;
    }

    let task = &inner.tasks[id];
    let task_info = TaskInfo {
        id: id,
        status: task.task_status,
        call: task.syscall_status,
        time: task.running_time,
    };

    drop(inner);

    unsafe {
        *ts = task_info;
    }

    0
}
