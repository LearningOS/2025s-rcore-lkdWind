//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

/// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// trace syscall
const SYSCALL_TRACE: usize = 410;


mod fs;
mod process;

use core::usize;

use fs::*;
use process::*;

extern crate alloc;
use alloc::collections::BTreeMap;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::task::get_current_app_id;

lazy_static! {
    static ref SYSCALL_COUNT: Mutex<BTreeMap<usize, BTreeMap<usize, usize>>> = Mutex::new(BTreeMap::new());
}

/// 为当前任务初始化记录,已有记录则返回
pub fn insert_syscall_count (current_app_id: usize) {
    if SYSCALL_COUNT.lock().contains_key(&current_app_id) {
        return;
    }
    let mut map = BTreeMap::new();
    map.insert(SYSCALL_WRITE, 0);
    map.insert(SYSCALL_EXIT, 0);
    map.insert(SYSCALL_YIELD, 0);
    map.insert(SYSCALL_GET_TIME, 0);
    map.insert(SYSCALL_TRACE, 0);
    SYSCALL_COUNT.lock().insert(current_app_id, map);
}

/// 计数器
fn syscall_count(id: &usize){
    // println!("==========={}-------",get_current_app_id());
    if let Some(v) = SYSCALL_COUNT.lock().get_mut(&get_current_app_id()).unwrap().get_mut(id) {
        *v += 1;
    }
}

/// 任务退出删除记录
pub fn delete_syscall_count(current_app_id: usize) {
    SYSCALL_COUNT.lock().remove(&current_app_id);
}

/// 获取当前任务某系统调用的调用次数
pub fn get_syscall_count(id: &usize) -> usize {
    *SYSCALL_COUNT.lock().get(&get_current_app_id()).unwrap().get(id).unwrap()
}

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {

    syscall_count(&syscall_id);
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TRACE => sys_trace(args[0], args[1], args[2]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
