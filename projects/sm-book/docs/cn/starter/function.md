

## Standard call

标准调用 `( )`

f(x)


## Smart Dot Call

智能调用 `.`

x.f

智能调用规则比较复杂

大体上分为两种, 

a.1

a.f


## Pipeline Stepping

管道步进 `|>` 以及 `|>>`

x |> f

```sm
prime(100)
|>> factor
|>> flatten
|>> total
```


## Prefix Operator

前缀式 `@`

f@x