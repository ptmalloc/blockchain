# 2020 信息安全工程 课程作业
```
***要求模拟区块链的结构和部分功能实现***
实验环境: Windows  
IDE: Clion  
开发语言: Rust (rustc 1.42.0)   
```

## 程序说明

### block结构
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

### 部分源程序功能
* /core/src/netrpc.rs 有关节点的通信广播部分 
* /core/src/filewrite.rs 数据的导出、导入


### 运行所需的包和库
1. 使用cargo创建main、core(lib)、utils(lib)  
2. 库依赖均在相应的Cargo.toml中,正常情况下单机run,程序会默认下载对应相应库(库的版本号已给出)  

### 说明
Rust新手，写出的代码质量为了课程作业比较垃圾，勿喷
