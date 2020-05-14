Simple Math Pest Parser
=======================



```js
.1
a.1

1.       // 1.0
1.2      // 1.2
1..2     // Range(1, 2)
1...2    // Range(1, 2)

1.fx     // fx(1)
1..fx    // fx(1.0)
1...fx   // error
```



def f(x: str = 1, *arg, *kv) {

}



a()

1.a

1.1

a.1

a..1
a()..1
a()[1]...1

a.0.1

2*[1, 2, 3]

a b => a * b
2 a b => 2 * a * b
2 sin(x) => 2 * sin(x)

D($a * $b, $x) := D(a,x)*b + a*D(b,x)


(a+b) => a + b

a_String + b_String := a <> b


$a:str + $b: str := a.join(b)


f[a_,b_]:=a+b


f(a, b)


@local a

@def 


@trait

@extension {

}

@if {

}


f[x_, y_ : 0] := {x, y}


f(x: str = "", y: int = 0) := [x, y]

f(x: str, y = 0) := [x, y]



// Integrate[f,x]
// $$\int f(x) \,\mathrm{d}x$$
//
// Integrate[f,{x,Subscript[x, min],Subscript[x, max]}]
// $$\iint f(x,y) \,\mathrm{d}x\,\mathrm{d}y$$
//
// Integrate[f,{x,Subscript[x, min],Subscript[x, max]},{y,Subscript[y, min],Subscript[y, max]},\[Ellipsis]]
//
//
// Integrate[f,{x,y,\[Ellipsis]}\[Element]reg]
//
// $$\int_{x \in \mathbb{R}} f(x) \,\mathrm{d}x$$



|               input |                            tex |                         rander |
| ------------------: | -----------------------------: | -----------------------------: |
|            `sin(x)` |                       `\sin x` |                       $\sin x$ |
|          `sin(x^2)` |                     `\sin x^2` |                     $\sin x^2$ |
|        `sin(x + 1)` |                  `\sin(x + 1)` |                  $\sin(x + 1)$ |
|        `sin(x + 1)` |       `\sin\left(x + 1\right)` |       $\sin\left(x + 1\right)$ |
|      `sin(x^2 + 1)` |     `\sin\left(x^2 + 1\right)` |     $\sin\left(x^2 + 1\right)$ |
| `cos(sin(x^2 + 1))` | `\cos\sin\left(x^2 + 1\right)` | $\cos\sin\left(x^2 + 1\right)$ |

- omit parentheses: sin/cos

D(a_*b_,x_) := D(a,x)*b + a*D(b,x)

1.Factor

2 x.m

2 x y

```js
integrate(5*E^(3x),[x,2,a]).(|x_| => expand(x))
```

```js
sum(i, (i, 1, n))
```


sum(a_Expression, b__Tuple)


a.map {

}


$2 x y$

`sin(x) cos(x)`

`sin(x)cos(x)`

`3 mod 5`


`a b if a {b}`

`a*b*if a {b}`

a.map[1]

```s
@TeXForm {
	sum(i, (i, 1, n))
}
```

2 (1, 2, 3)

1(2)



bad decimal

由于残数是 atom, 所以总会优先解析

```
.123
123.

a.1
a .1

1.a
1. a
```

