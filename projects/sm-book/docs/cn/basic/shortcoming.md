
wolfram 中合法但是 simple math 中非法的例子

```
1[2]
```

sm 中 caller 必须是 symbol

```sm
1(2)
```


```
f'[x]'[y]
Derivative[1][Derivative[1][f][x]][y]
```

我想不明白要多想不开才会写这样的东西出来

sm 中的 derivative 是特设模式, 无法扩展

```
f^2[x]
```

