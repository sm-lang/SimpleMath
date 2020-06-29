


## `@` 与 `.` 与 `|>` 的区别

- dot_call: `.`
  - 优先查找自身定义的方法
  - 如果没有, 在 global 中寻找
  - 如果仍没有, 保持该形式
- pipeline: `|>`
  - 直接在

```sm
[1, 2, 3].first       # ok
[1, 2, 3] |> first    # ok
[1, 2, 3].first()     # ok
[1, 2, 3] |> first()  # error
```

```sm
[1, 2, 3].sort_by(N)
# List::sort_by(N, [1, 2, 3])
[1, 2, 3] |> sort_by(N)
# sort_by(N)([1, 2, 3])
```

## `¯` 与 `¨` 的区别

- 精度/精密度: `¯`
  - `1¯20` Precision
- 准度/准确度: `¨`
  - `1¨20` Accuracy