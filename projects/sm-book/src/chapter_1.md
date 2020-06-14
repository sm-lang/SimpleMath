# Chapter 1

- 稀疏缓存

对涉及整数运算的递归函数采用稀疏缓存

稀疏缓存采用分段打表的思想 :> 缓存一些关键值来加速运算

整数函数的缓存分为三个部分 :> 三个部分共用一个缓存区 `MAX_SIZE`
- 小数堆: Vec :> 编译期计算较小数值并缓存
- 特值堆: BTreeMap :> 固定的特殊值 :> 直接打表写入
- 活跃堆: LRU :> 用户运算到 $f(n)$ 后很可能会继续算 $f(n+1)$



omit function
基本初等函数
log x




stage

rewriting stage

forward stage

rewriting stage