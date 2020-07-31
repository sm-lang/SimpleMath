


## 运算顺序

- pre-parse stage
- parse stage
- macro stage
- forward stage
- rewriting stage


### 预处理器

当以模块导入时

- 将换行符统一转为 `LF`
- 每行末尾的续行标记 `\` 
- 首行如果是 `#!` 开头将被移除


因此续行是顶级转义, 能够豁免被转义

因此, 在多行字符串中有如下行为:

```sm
string_multi = """
line 1 \
still line 1
line 2 \\
still line 2
line 3 \\\
still line 3
"""

% `\s` become ` `
string_single = "
line 1 still line 1
line 2 till line 2
line 3 \\still line 3
"
assert_equal(string_multi, string_single)
```

