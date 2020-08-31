## Constant

| Operator | Namespace     | Name        | Mathematica | Input  |
| :------: | :------------ | :---------- | :---------- | :----- |
|   `∞`    | std::constant | Infinity    | Infinity    | `\inf` |
|   `⊤`    | std::constant | TopType?    |             |
|   `⊥`    | std::constant | ButtomType? |             |
|   `i`    | std::constant |             |             | `\i`   |
|   `ℯ`    | std::constant | Euler       |             | `\e`   |
|   `∅`    | std::set      | Empty       |             |
|   `ℙ`    | std::set      | Prime       |             |        |
|   `ℕ`    | std::set      | Natural     |             |        |
|    ?     | std::set      | Postive     |             |        |
|   `ℤ`    | std::set      | Integer     |             |        |
|   `ℚ`    | std::set      |             |             |        |
|   `ℝ`    | std::set      |             |             |        |
|   `ℂ`    | std::set      |             |             |

## Function

| Operator | Namespace | Name | Mathematica | Input |
| :------: | --------: | ---: | ----------- |: | :---- |
|   `∁`    |         ? | complement | Complement  |
|   `∂`    |         ? | derivative | D           |

$\operatorname{}$

## Prefix

| Operator |    Name | Precedence | Mathematica | Input |
| :------: | ------: | ---------: | :---------- | :---- |
|   `+`    |  `plus` |            |             |
|   `!`    | `minus` |
|   `¬`    |   `not` |
|   `?`    |         |            |             |
|   `??`   |         |            |             |

## Suffix

| Operator |     Name | Precedence | Mathematica | Input |
| :------: | -------: | ---------: | :---------- | :---- |
|   `°`    | `degree` |            | Degree      |
|   `⊺`    |
|   `⊹`    |
|    ?     |

## Infix


| Operator |            Name | Precedence | Mathematica       | Input  |
| :------: | --------------: | ---------: | :---------------- | :----- |
|   `.`    |      `dot_call` |        900 |                   | `.`    |
|   `<<`   |         import? |        720 | Get               | `<<`   |
|   `≪`    |                 |            | `\<<`             | `\<<`  |
|   `@`    |               ? |        640 | Prefix            |
|   `@*`   |               ? |        625 | Composition       |
|   `/*`   |               ? |        624 | RightComposition  |
|   `/@`   |               ? |        620 | Map               |
|   `@@`   |               ? |        620 | Apply             |
|  `@@@`   |               ? |        620 | Apply             |
|   `++`   |          `join` |        600 | Join `<>`         | `++`   |
|   `×`    |         `cross` |        500 | Cross `×`         | `\*`   |
|   `·`    |           `dot` |        490 | Dot `.`           | `\.`   |
|   `/`    |        `divide` |        470 | Divide `/`        | `/`    |
|   `*`    |         `times` |        400 | Times             | `*`    |
|   `+`    |           `add` |        310 | Plus `+`          | `+`    |
|   `-`    |      `subtract` |        310 | Subtract          | `-`    |
|   `∈`    |       `element` |        250 | Element `∈`       | `\in`  |
|   `->`   |               ? |        120 | Rule `->`         | `->`   |
|   `->`   |                 |            |                   | `\to`  |
|  `:->`   |               ? |        120 | RuleDelayed `:>`  | `:->`  |
|   `|>`   |               ? |         70 | Postfix `//`      | `|>`   |
|   `=>`   |               ? |
|  `<=>`   |               ? |
|   `>>`   |         export? |
|   `<>`   |                 |            |                   | `<>`   |
|   `≫`    |         export? |         30 | Put               | `\>>`  |
|   `>`    |       `greater` |
|   `<`    |          `less` |
|   `>=`   | `greater_equal` |
|   `<=`   |    `less_equal` |
|   `==`   |          equal? |
|   `≈`    |         approx? |
|  `===`   |     equivalent? |
|   `≡`    |                 |            | `\>>`             |
|   `!=`   |       not_equal |
|   `≠`    |                 |            | `\!=`             |
|  `=!=`   |               ? |
|   `≢`    |                 |            |                   | `\!==` |
|   `^`    |               ? |            |                   |
|   `//`   |               ? |        670 | Quotient          |
|   `÷`    |               ? |        670 | QuotientRemainder |
|   `√`    |           surd? |
|   `∧`    |       logic_or? |
|   `∨`    |      logit_and? |
|   `∩`    |         set_or? |
|   `∪`    |        set_and? |
|   `⊻`    |               ? |
|   `⊼`    |               ? |
|   `⊽`    |               ? |
|   `⋖`    |               ? |
|   `⋗`    |               ? |
|   `±`    |    `plus_minus` |            |                   | `\+-`  |
|   `∓`    |    `minus_plus` |            |                   | `\-+`  |

## Assign

| Operator |      Name | Precedence | Mathematica  |
| :------: | --------: | ---------: | :----------- |
|   `?=`   |         ? |
|   `≟`    |         ? |            |              |
|   `:=`   |         ? |         40 | SetDelayed   |
|   `≔`    |         ? |            |              |
|   `=`    |         ? |         40 | Set          |
|   `+=`   |    add_by |            | AddTo        |
|   `-=`   |  minus_by |            | SubtractFrom |
|   `*=`   |  times_by |            | TimesBy      |
|   `/=`   | divide_by |            | DivideBy     |
|   `^=`   |         ? |            |              |
|   `≈`    |         ? |


## Paired

| Operator | Name | Mathematica |
| :------: | ---: | :---------- |
|  `〈 〉`   |      |             |
