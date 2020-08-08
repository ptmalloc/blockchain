# 2020 信息安全工程 课程作业
```
***要求模拟区块链的结构和部分功能实现***
实验环境: Windows  
IDE: Clion  
开发语言: Rust (rustc 1.42.0)   
```

## 程序说明

### 参考的block结构（自己查看）
```
区块头共80字节，分为6个部分：version，prevBlockHash，merkleRoot，time difficultyTarget，nonce。
version：大小为4字节，记录了区块头的版本号，用于跟踪软件/协议的更新；
prevBlockHash：大小为32字节 ，记录了该区块的上一个区块的Hash地址；
merkleRoot：大小为32字节，记录了该区块中交易的merkle树根的哈希值；  //tx_hash
time：大小为 4字节，记录了该区块的创建时间戳；
difficultyTarget：大小为4字节，记录了该区块链工作量证明难度目标；
nonce：大小为4字节，记录了用于证明工作量的计算参数。

区块体的内容是该区块的交易信息，包括交易数量和交易数据。区块体共分为三部分：
numTransactionsBytes，numTransactions，transactions。
numTransactionsBytes：大小为1字节，记录了交易数量占用的字节数；
numTransactions：大小为0-8个字节，记录了区块内的交易数量；
transactions：大小不确定，记录了区块内存的多个交易数据。
```
区块头部分保留，区块链内的部分简化为data 

### 实现的功能
* 区块的生成
* 区块信息的数据保存
* 区块信息的传输同步

### 运行所需的包和库
1. 使用cargo创建main、core(lib)、utils(lib)  
2. 库依赖均在相应的Cargo.toml中,正常情况下直接输入命令cargo run,程序会默认下载对应相应库(库的版本号已给出)并运行  

### 程序逻辑说明
* /main/src/main.rs 文件包含了程序测试的主要代码，其中包含区块链的生成测试，区块链信息写入文件的测试，还有Server和client模拟测试
* /core/src/filewrite.rs 将区块链的每一块序列化存入文件中
* /core/src/netrpc.rs 其中有大量的冗余代码（在编写过程的初期逻辑）。已server和client的方式通过udp传输传递区块信息。
* /core/src/block.rs 定义了区块的数据结构
* /core/src/blockchain.rs 定义了区块链的数据结构，和初始化化部分代码
* /core/src/filewrite.rs 数据的导出、导入在Database目录下
* /utils/src/coders.rs 区块的序列化和反序列化

### 项目过程参考
参看项目与视频: https://www.bilibili.com/video/BV145411t7qp  
相比于该视频中的项目，个人的课程作业项目修改了区块链的结构，添加了区块链信息的文件保存以及模拟区块链中的通信功能。

## 说明
Rust新手，写出的代码质量为了课程作业比较垃圾，勿喷  
实现的功能也很少，勿喷  
本人是菜鸡  
项目地址: https://github.com/ptmalloc/blockchain

## 结束与总结 (期末作业完成)
结课最后居然没要源码。。。  
总结：该项目作为rust新手编写，有部分借鉴他人的代码，其次该课程大作业也出现了一些问题，比如merkle树没有实现，通信应该要用到p2p
相关的库去实现，还有并没有对区块生成的>6 进行校验。  
国内也有相关区块链公司，用rust实现以太坊的顶层设计。之前在b站上看到这家公司宣传用他家公司的产品部署智能合约。

End 告一段落

## 计划
* 添加p2p网络 (2020/08/08)