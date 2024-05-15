# 编程题目

## 更改sys_xx使其在新的分支上可以使用
将原先写在全局MANAGER中的函数更改到TaskControlBlock中
task的初始调度时刻需要在run_tasks中更新即可
syscall中用current_task()来得到对应当前的tcb，不再像之前那样写几个全局函数。

## sys_spawn
基本上是fork和exec的缝合怪。传入一个app_name，返回子进程arc。
需要注意我们不需要拷贝父进程的内存空间，直接从from_elf中获取即可。
pid和kernel_stack需要alloc，更改trap_cx为from_elf中得到的用户sp和entry_point
在sys_spawn中不需要像sys_fork一样设置x[11]作为子进程返回值。

## stride
更改TaskManager，使用vec，fetch_task暴力遍历取最小Stride的tcb
为Stride实现Add，AddAssign，PartialOrd等，防止溢出，见问答题。

# 问答作业

## 实际情况是轮到p1执行吗？
不是，p2溢出，直接比较比p1小，会选取p2

## 简单说明 StrideMAX - StrideMIN <= Biggest / 2
由题意，一个Stride不可能加上超过Biggest/2的pass，而该调度不会给较大的数加pass。
所以只有发生溢出，此assertion不成立。

## 补全partialOrd
~~~
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut res = self.0 < other.0;
        if u64::abs_diff(self.0, other.0) > u64::MAX / 2 {
            res = !res;
        }
        if res {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
~~~

# 荣誉准则

## 1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

## 2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

实现add时找copilot查了是哪几个trait和需要实现的函数。

## 3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

## 4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。