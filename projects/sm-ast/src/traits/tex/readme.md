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
Integrate(5*E^(3x),[x,2,a]).(|x_| => Expand(x))
```

```js
Sum(i, (i, 1, n))
```

