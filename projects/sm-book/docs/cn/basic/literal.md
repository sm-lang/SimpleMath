

## Symbol

```sm
a
a1
a_1
sin
```

## String

```sm
single = "single line"
multiline = """
line one
line two
"""
```


## Numbers

- 浮点输入: `**`
  - `2**5` => `2e5`
- 精度/精密度: `¯`
  - `1¯20` Precision
- 准度/准确度: `¨`
  - `1¨20` Accuracy

## REPL only

```sm
¶1: 1 + 1
⁋1: 2
```

- 输入
  - 上个输入 `¶`
  - 第 n 个输入 `¶n`
  - 上上个输入 `¶¶`
  - 上上上个输入 `¶¶¶` 以此类推
- 输出
  - 上个输出 `⁋`
  - 第 n 个输出 `⁋n`
  - 上上个输出 `⁋⁋`
  - 上上上个输出 `⁋⁋⁋` 以此类推

## Function

```sm
f @ x
x.f
x |> f
```


## Lambda

```sm
#
#1
##
f = #.1 + #2 + 3&
```

## Macro

```sm
@TeXForm {sin(1 + 1)}
TeXForm@sin(1 + 1)
```

## Special

| Operator | Name | Mathematica |
| :------: | ---: | :---------- |
|   `x_`   |
|  `x__`   |
|  `x___`  |


§
