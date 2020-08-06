


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

### 解析器

以下语句不能换行
- 函数语法 `expr(a)`
- 索引语法 `expr[a]`
- 空格乘法 `a b`
- 系数乘法 `2x`

中缀符号会自动续行, 无论是写末尾还是另一行行首

```sm
a = 1
  - 2x + 3x^2
```

如果想表达它其实是个前缀, 要另起表达式的话可以加个括号

```sm
x = 1
(-2x) + 3x^2
% 如果不需要输出可以直接加 `;`
y = 1;
-2y + 3y^2
```

又想加括号又想续行可以这么写

```sm
a = 1 + 
(-2x) + 3x^2
```