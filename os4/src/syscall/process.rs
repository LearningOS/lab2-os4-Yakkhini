//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::mm;
use crate::task;
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: task::TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    task::exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    task::suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let ts_phy_ptr = mm::get_refmut(task::current_user_token(), ts);

    /*
    println!(
        "[debug] Now time is {} sec and {} usec, {} us.",
        us / 1_000_000,
        us % 1_000_000,
        us
    );
    */

    *ts_phy_ptr = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    // println!("[debug] Now sys_get_time function finished.");

    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    task::mmap(start, len, port)
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    task::munmap(start, len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let syscall_times_vector = task::get_syscall_times();

    let mut syscall_times = [0; MAX_SYSCALL_NUM];

    let mut i = 0;
    for item in syscall_times_vector.iter() {
        syscall_times[i] = *item;
        i = i + 1;
    }

    let ti_phy_ptr = mm::get_refmut(task::current_user_token(), ti);

    *ti_phy_ptr = TaskInfo {
        status: task::TaskStatus::Running,
        syscall_times,
        time: task::get_current_task_time(),
    };

    0
}
