//! Process management syscalls
use core::{ops::BitOr, slice};


use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE}, mm::{translated_byte_buffer, MapPermission, VirtAddr}, task::{
        change_program_brk, current_first_time, current_mmap, current_munmap, current_syscall_times, current_user_token, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus
    }, timer::{get_time_ms, get_time_us}
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// # Safty
/// ptr is valid
unsafe fn write_to_user_space<T>(ptr: *mut T, val: &T) {
    let dst = translated_byte_buffer(current_user_token(), ptr as *mut u8, core::mem::size_of::<T>());
    let src =  unsafe { slice::from_raw_parts((val as *const T) as *const u8, core::mem::size_of::<T>())}; 
    {
        let mut start = 0;
        for i in dst {
            i.copy_from_slice(&src[start..start+i.len()]);
            start += i.len();
        }
    }
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let time = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    unsafe { write_to_user_space(_ts, &time) };
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    if let Some(current_first) = current_first_time() {
        let time = get_time_ms();
        if current_first <= time {
            let res = TaskInfo {
                status: TaskStatus::Running,
                syscall_times: current_syscall_times(),
                time: time - current_first,
            };
            unsafe { write_to_user_space(_ti, &res) };
            return 0;
        }
    }
    -1
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _prot: usize) -> isize {
    trace!("kernel: sys_mmap");
    if _start % PAGE_SIZE != 0 {
        return -1;
    }
    if _prot & !0x7 != 0 || _prot & 0x7 == 0 {
        return -1;
    }
    let perm =  MapPermission::from_bits((_prot << 1) as u8)
        .unwrap()
        .bitor(MapPermission::U);
    if current_mmap(VirtAddr::from(_start),VirtAddr::from( _start + _len), perm) {
        0
    } else {
        -1
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: munmap");
    if _start % PAGE_SIZE != 0 {
        return -1;
    }
    if current_munmap(VirtAddr::from(_start), VirtAddr::from(_start+_len)) {
        0
    } else {
        -1
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
