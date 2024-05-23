# 编程作业

## 适配sys_spawn
更改app文件打开方式，使用open_file和read_all

## 实现fstat
在DiskInode中添加nlink字段，减少直接索引的数量。
在inode中实现nlink，is_dir等，都是read_inode后直接返回。
在inode中实现inode_id方法，是根据inodeid得到blockid和blockoffset的反转。
在impl File for OSInode中添加fstat，得到Stat，注意stdin等都是unimplemented。
sys_stat做简单检查及写入，注意写入要从已经得到的tast_inner中得到token，否则borrowerror。

## 实现linkat，unlinkat
主要实现部分在inode中。
linkat会在当前inode的最后位置填入一个新的dentry，它有src的inodeid和dst的名字。
同时读出（创建）id对应的inode，修改它的nlink字段。
unlinkat会在一个modify中查找是否存在name对应的id，之后将它后方的dentry一个一个的向前提，然后根据是否找到修改size。
得到id后，根据id创建inode，修改nlink，nlink==0时使用clear方法。
注意在clear前要drop掉fs，否则死锁。

# 问答作业ch6

在我们的easy-fs中，root inode起着什么作用？如果root inode中的内容损坏了，会发生什么？

root inode是该文件系统中的唯一的目录项，用来索引其余文件的inode。
损毁后，所有文件的inode都无法被索引到，在os看来它们不存在。

# 问答作业ch7

举出使用 pipe 的一个实际应用的例子。
cat file | wc -l
此时pipe用来在cat和wc中通信

如果需要在多个进程间互相通信，则需要为每一对进程建立一个管道，非常繁琐，请设计一个更易用的多进程通信机制
我们可以准备一个全局的manager，它接受各个进程发送的数据包，然后根据目的pid，检查是否存在dst对src数据包的请求，然后将它转发出去。数据包使用者只需要预先为其准备一块内存即可。

# 荣誉准则

## 1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

## 2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无

## 3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

## 4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。


