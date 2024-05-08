# 编程作业

在taskControlBlock中添加一个存放syscall计数的数组，一个初次调度时间ms的option。
时间初始化为None，在run_first_task中和run_next_task中检查是否已经被更新。
syscall计数在trap_handler中更新，在syscall内部也可以，trap时不改变current task。
在TASK_MANAGER类型中实现了添加syscall计数更新函数和一些get方法，并且封装了外部函数，方便调用。
task_info_syscall，在当前任务初始调度时间为None或小于now时error。

# 简答作业
1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
    1. bad_address: 出现pagefault的提示，标明了错误的访问地址0x0和该指令的位置0x804003ac。
    2. bad_instruction: 提示非法指令。
    3. bad_regester: 提示非法指令。
    4. 如上的错误都在traphandler中处理。bad_address时scause设置为StoreFault错误，后二者设置为IllegalInstruction，在match中处理。
    5. sbi version: RustSBI-QEMU Version 0.2.0-alpha.2

2. 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:

    1. L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。
        a0代表__alltraps传入traphandler的cx: &mut TrapContext。
        a. traphandler完成后恢复到触发trap的task
        b. 初始化时为每个任务分配新建的TaskContext

    2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
    ```asm
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    ```
        sstatus: 得到触发trap前的进程的特权级状态
        spec：记录异常时应当返回的地址
        sscratch：记录用户栈sp
        处理三个特殊寄存器的目的：实现嵌套trap

    3. L50-L56：为何跳过了 x2 和 x4？
    ```asm
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
    LOAD_GP %n
    .set n, n+1
    .endr
    ```
        x2: 别名sp，我们的目的是读取用户栈的sp，在进入__alltraps中sp已经同sscratch交换，用户栈sp保存在sscratch。
        x4：线程相关，单线程下没有作用。

    4. L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？
    csrrw sp, sscratch, sp
        sp: 用户栈栈顶
        sscratch：内核栈栈顶，不含trapcontext

    5. __restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？
        sret，从特权级返回。
        该指令会更新中断状态，设置pc，用sstatus的SPP位更新当前权限模式。故可以正常返回中断前状态。

    6. L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？
    csrrw sp, sscratch, sp
        sp: 内核栈栈顶
        sscratch: 用户栈栈顶

    7. 从 U 态进入 S 态是哪一条指令发生的？
        用户态触发ecall。
        此时跳转到stvec进行中断处理。该CSR预先设置为__alltraps地址

# 荣誉准则

## 1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

## 2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

### ch1
- [riscv manual](https://github.com/riscv-non-isa/riscv-asm-manual/blob/master/riscv-asm.md)
- [rust unstable](https://doc.rust-lang.org/unstable-book)

### ch2
- [rust inline asm](https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html)

### ch3
无额外资料，上述给出材料为读代码时查阅

### Github Copilot Chat usage
- 解释linker脚本，一些riscv汇编指令

## 3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

## 4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。