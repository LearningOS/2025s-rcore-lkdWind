# 功能总结

​	关于spawn,相比fork就是不要复制父进程的内存布局,将传进来的字符串指针转换成string,通过这个名字获取应用elf_data,后面就对TCB进行初始化,和TCB::new一样,注意三点,父进程设置为当前进程;将该进程加入父进程的child列表;将该进程加入运行队列.
​	关于stride,我定义了两个常量BASE_STRIDE=91,BIG_STRIDE=99991.在TCBinner里加入两个量:pass=0,prio=1,封装updatepass: 
pass += (BASE_STRIDE / prio) % BIG_STRIDE
留出相关接口.再到run_task调用updatepass, 在fatch里对ready_queue取出最小pass的TCB.

# 简答作业

1. p2执行后：p2.stride = 250 + (BIG_STRIDE/prio)如果BIG_STRIDE/prio > 5，p2.stride会溢出变成很小的值

2. 由于单次增量≤BIG_STRIDE/2,所以两个stride的真实差值不会超过BIG_STRIDE/2
   任何更大的"差值"都说明发生了溢出
   p1=255, p2=250 (假设BIG_STRIDE=256)
   p2执行后：p2=250+128=378->122(溢出)
   比较122和255：
   122-255=378-255=123 > 128->实际255更小

3. 
use core::cmp::Ordering;
struct Stride(u64);
impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        const BIG_STRIDE: u64 = 255; 
        
        // 计算差值（考虑溢出）
        let diff = self.0.wrapping_sub(other.0);
        
        if diff <= BIG_STRIDE / 2 {
            // self <= other
            Some(Ordering::Less)
        } else {
            // 溢出情况：self > other
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

# 荣誉准则

    1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容： 无
        2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容： 无
        3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
        4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计