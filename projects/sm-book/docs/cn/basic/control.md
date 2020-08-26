## If Statement



`iff cond {}` 等价于 `if is_true(cond) {}`


```sm
% 三值逻辑
if 


% 二值逻辑
iff a {

}
```




## For-In Loop

```sm
for a in b {

}
```


## For Notation

```sm
for {


}
```

## While Loop

```sm
while a > 0 {


}

```


## Infinite Loop

等价于 `while true`

```sm
loop {
    return true;
}
```

## Switch

```sm
switch {
    a == 1, "1"
    a == 2, "2"


}
```



## Match

```sm
match a {


}
```


## Do Notation

```sm
do {
    a <- b
}
```
