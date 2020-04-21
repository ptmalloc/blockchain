# 2020 信息安全工程 课程作业
```
***要求模拟区块链的结构和部分功能实现***
实验环境: Windows  
IDE: Clion  
开发语言: Rust (rustc 1.42.0)   
```

2020/4/22
/core/src/netrpc.rs 有关节点的通信广播部分 
/core/src/filewrite.rs 数据的导出、导入

1. 使用cargo创建main、core(lib)、utils(lib)  
2. 库依赖均在相应的Cargo.toml中,正常情况下单机run,程序会默认下载对应相应库(库的版本号已给出)   