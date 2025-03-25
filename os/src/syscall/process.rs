//! Process management syscalls
use crate::{config::PAGE_SIZE, mm::{PageTable, PhysAddr, VirtAddr}, task::{change_program_brk, current_user_token, exit_current_and_run_next, suspend_current_and_run_next}, timer::{get_time_val, TimeVal}};



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

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let page_table = PageTable::from_token(current_user_token());
    let start = ts as usize;
    let end = start + core::mem::size_of::<TimeVal>();
    if (start / PAGE_SIZE) != (end / PAGE_SIZE) {
        // 跨页
        let first_page_end = (start / PAGE_SIZE + 1) * PAGE_SIZE;
        let first_part_size = first_page_end - start;
        let second_part_size = end - first_page_end;
        
        // 写入第一个页中的数据
        let first_part_ptr = VirtAddr::from(start);
        let first_offset = first_part_ptr.page_offset();
        let first_part_vpn = first_part_ptr.floor();
        let mut first_pa:PhysAddr = page_table.translate(first_part_vpn).unwrap().ppn().into();
        first_pa.0 += first_offset;
        let first_part = unsafe {
            core::slice::from_raw_parts_mut( first_pa.get_mut() as *mut u8, first_part_size)
        };
 
        // 写入第二个页中的数据
        let second_part_ptr = VirtAddr::from(first_page_end);
        let second_part_vpn = second_part_ptr.floor();
        let second_part_pa:PhysAddr = page_table.translate(second_part_vpn).unwrap().ppn().into();
        let second_part = unsafe {
            core::slice::from_raw_parts_mut(second_part_pa.get_mut() as *mut u8, second_part_size)
        };

        let time_val = get_time_val();
        let time_val_bytes = unsafe {
            core::slice::from_raw_parts(&time_val as *const TimeVal as *const u8, core::mem::size_of::<TimeVal>())
        };
        first_part.copy_from_slice(&time_val_bytes[..first_part_size]);
        second_part.copy_from_slice(&time_val_bytes[first_part_size..]);
    } else {
        // 不跨页
        let kernel_vaddr = VirtAddr::from(start);
        let offset = kernel_vaddr.page_offset();
        let kernel_vpn = kernel_vaddr.floor();
        let mut pa: PhysAddr = page_table.translate(kernel_vpn).unwrap().ppn().into();
        pa.0 += offset;
        let timeval = get_time_val();
        unsafe {
            *(pa.get_mut() as *mut TimeVal) = timeval;
        }
        
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    -1
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
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
