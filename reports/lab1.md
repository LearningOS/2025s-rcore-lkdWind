# 功能总结

​	关于trace的系统调用实现,需要判断request来实现不同功能. 在request为0时:需要将id转换成\*const u8 再解引用,再转换为isize返回;在request为1时:要将id转换成\*mut u8,再将data转换成u8,复制给id转换后指针的解引用,返回0;

​	当request为2时: 想要获取每个任务的某个系统调用次数,并且考虑到批处理系统会进行任务切换,所以在切换到其他任务时也要保存着其他刮起的人物的调用次数,所以我就在syscall/mod.rs中实现了个全局变量SYSCALL_COUNT,类型为Mutex<BTreeMap<usize, BTreeMap<usize, usize>>>,Mutex包裹是因为他是全局变量(rust的要求? 按理说批处理单进程用不到锁,但是我编译没过就加上了.),外层BTree key为app_id,value为内层BTree,内层BTree为(sys_id, count).这样就可以通过app_id来得到所有系统调用的次数.syscall_count方法在syscall里用来计数,insert_syscall_count为当前任务初始化记录,已有记录则返回,delete_syscall_count在进程退出时删除记录,get_syscall_count就用来在sys_trace里获取当前任务的系统调用次数.

# 简答作业

1.  SBI 版本：RustSBI 0.2.0-alpha.1
    目标架构：riscv64gc-unknown-none-elf
    ch2b_bad_instruction.rs: 用户态程序尝试执行 S 态特权指令 sret。由于用户态没有权限执行 sret，CPU 会触发非法指令异常。内核捕获异常后终止程序
    ch2b_bad_register.rs: 用户态程序尝试读取 S 态寄存器 sstatus。由于用户态没有权限访问 sstatus，CPU 会触发非法指令异常。内核捕获异常后终止程序
    ch2b_bad_address.rs: 用户态程序尝试访问非法地址 0x0。由于该地址不在用户程序的合法地址范围内，CPU 会触发页错误异常。内核捕获异常后终止程序

2. 1) 刚进入restore时sp为内核栈指针,restore有两种适用情形:中断恢复和上下文切换,在中断发生时用来保存寄存器状态,进入内核态,在任务切换时需要保存当前任务的寄存器状态并家在下一个任务的寄存器状态.
   2) 将t0,t1,t2分别写入sstatus, sepc, sscratch; sstatus 的值告诉cpu为用户态模式,  sepc 为进程在中断结束后的返回地址, sscratch 作为sp在交换用户栈与内核栈的中间量.
   3) x2是sp栈指针寄存器在切换上下文和中断都不能被覆盖, x4为tp线程指针寄存器（Thread Pointer），通常用于指向线程局部存储（TLS）的基地址,不需要保存.
   4) 该指令后sp(内核栈指针) 和 sscratch的值(用户栈指针)进行了交换.
   5) sret;执行 sret 时，CPU 会从 sepc 中加载PC的值，并从 sstatus 寄存器中恢复运行模式
   6) sp(用户栈指针) 和 sscratch的值(内核栈指针)进行了交换.
   7) call trap_handler后通过ecall中断进入s态

# 荣誉准则
    1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容： 无
    2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容： 无
    3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
    4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计

