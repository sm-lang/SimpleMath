Simple Math 的基本类型只有 5 个:

- 符号 (Symbol)
- 文本 (String)
- 整数 (Integr)
- 小数 (Decimal)
- 真值 (Boolean)

外加两种常用容器:

- 列表 (List)
- 映射 (Dict)

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

## Booleans

真值只有两个, `true` 和 `false`


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
