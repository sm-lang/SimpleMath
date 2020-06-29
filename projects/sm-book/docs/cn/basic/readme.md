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


加法符号 `+` 默认与该集合构成阿贝尔群, `0` 是该集合的单位元.

也就是说加法具有交换律

若加法被定义在字符串上, 则下式成立:

"a" + "b" == "b" + "a"

约定 运算 单位元 幂 逆元
加法运算 x + y 0 nx −x

- 单位元
 
a_DoubleAnyThing + 0 := 2a 无法成立






using std.{E, D, N, I, π}

Π(x)

- 英文字母与希腊字母要能区分
- 等宽, 但不要求与中文对齐
- 支持符号连字, 且连字与其 unicode 形式差别要小
- 不需要考虑与中文混合排版导致的问题
- 字重: Regular, Blod, Italic

A B C D E F G H I J K L M N O P Q R S T U V W X Y Z

Α Β Γ Δ Ε Ζ Η Θ Ι Κ Λ Μ Ν Ξ Ο Π Ρ  Σ Τ Υ Φ Χ Ψ Ω

α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ ς σ τ υ φ χ ψ ω

~ ! @ # $ % ^ & * _ + : ? > < >

{ } [ ] ( ) " "

<>  |> >>

a · b
a * b § °