

|         sm |              wl |
| ---------: | --------------: |
|  `[1,2,3]` |     `List[1,2]` |
| `*[1,2,3]` | `Sequence[1,2]` |



| sm                          | wl                        |
| :-------------------------- | :------------------------ |
| `$x`                        | `x_`                      |
| `$x+`                       | `x__`                     |
| `$x*`                       | `x___`                    |
| `$x.`                       | `x_.`                     |
| `($x: Integer)`             | `x_Integer`               |
| `($x: Integer ? _ > 0)`     | `x_Integer ? (# > 0)&`    |
| `($x: Integer = 1 ? _ > 0)` | `x_Integer: 2 ? (# > 0)&` |

return *[1,2]


```js
//Deconstruction
*($x: Integer = 2 ? _ > 0) := x^2


```



```js
*($l : List) := Sequence@@l
```


*2

```
expect($?is_numberic):> 1
```

```js
f($x: Integer) := x
```

```js
add_two($x?_>0,$y?_>0):= x + y

add_two($x,$y):= x + y :? (_>0)(x)&&(_>0)(y)

def add_two($x, $y)
iff x > 0 && y > 0 {
    x + y
}
```

```s
add_two(x_?$>0,y_?$>0):= x + y

add_two(x_,y_):= x + y /; ($>0)(x)&&($>0)(y)

def add_two(x_, y_)
iff x > 0 && y > 0 {
    x + y
}
```

k[x : _Integer?(# > 0 &) : 2] := x^2

k(x_: Integer : 2 ?($ > 0 &)) := x^2


f($x: Integer = 2) := x^2

```wl
(*with Optional*)
f[x_Integer : 2] := x^2
f[]
```

```wl
(*with PatternTest*)
g[x_Integer?(# > 0 &)] := x^2
g[2]
```

```wl
(*with Optional+Condition*)
h[x_Integer : 2] := x^2 /; x > 0
h[]
h[2]
```

```wl
if $1 > 2 {2 $0($1 - $0($1 - 2))} else {1} & /@ range(50)
```

```s
# pm
range(e_) := range(1,x,1)
range(s_, e_) := range(s,e,1)
```

```s
f(1)=f(0)==1
f(n_):=f(n-1)+f(n-2)
```

```
max(a_, b_) := if a > b
then {
    a
}
else {
    b
}
otherwise {
    a
}

max(a_,b_) := if a > b {
    a,
    b,
    a,
}
```




大写是容器, 需要转化为 Iterator 执行
Range

小写是函数, 立即生效
range