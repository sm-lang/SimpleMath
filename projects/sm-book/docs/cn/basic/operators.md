r##########"                          "##########
r##########"           ""             "##########
r##########"          "  "            "##########
r##########"         """"""           "##########
r##########"        """  """          "##########
r##########"       """ "" """         "##########
r##########"     r"""  \\  """        "##########

## Constant

| Operator | Namespace     | Name        | Mathematica | Input  |
| :------: | :------------ | :---------- | :---------- | :----- |
|   `∞`    | std::constant | Infinity    | Infinity    | `\inf` |
|   `⊤`    | std::constant | TopType?    |             |
|   `⊥`    | std::constant | ButtomType? |             |
|   `i`    | std::constant |             |             | `\i`   |
|   `e`    | std::constant | Euler       |             | `\e`   |
|   `∅`    | std::set      | Empty       |             |
|   `ℙ`    | std::set      | Prime       |             |        |
|   `ℕ`    | std::set      | Natural     |             |        |
|    ?     | std::set      | Postive     |             |        |
|   `ℤ`    | std::set      | Integer     |             |        |
|   `ℚ`    | std::set      |             |             |        |
|   `ℝ`    | std::set      |             |             |        |
|   `ℂ`    | std::set      |             |             |

## Function

| Operator | Namespace |       Name | Mathematica | Input |
| :------: | --------: | ---------: | :---------- | :---- |
|   `∁`    |         ? | complement | Complement  |
|   `∂`    |         ? | derivative | D           |

$\operatorname{}$

## Prefix

| Operator | Name | Precedence | Mathematica | Input |
| :------: | ---: | ---------: | :---------- | :---- |
|   `+`    |      |            |             |
|   `!`    |
|   `¬`    |
|    ?     |


## Suffix

| Operator | Name | Precedence | Mathematica | Input |
| :------: | ---: | ---------: | :---------- | :---- |
|   `?`    |      |            |             |
|   `°`    |      |            | Degree      |
|   `⊺`    |
|   `⊹`    |
|    ?     |

## Infix


| Operator |             Name | Precedence | Mathematica       | Input  |
| :------: | ---------------: | ---------: | :---------------- | :----- |
|   `.`    |       `dot_call` |        900 |                   | `.`    |
|   `+`    |       `additive` |        310 | Plus `+`          | `+`    |
|   `-`    |       `additive` |            |                   | `-`    |
|   `*`    | `multiplicative` |            |                   | `*`    |
|   `×`    |          `cross` |        500 | Cross `×`         | `\*`   |
|   `·`    |            `dot` |        490 | Dot `.`           | `\.`   |
|   `=>`   |                ? |
|  `<=>`   |                ? |
|   `>>`   |          export? |
|   `≫`    | ---------------- | ---------- | ----------------- | `\>>`  |
|   `<<`   |           input? |
|   `≪`    | ---------------- | ---------- | ----------------- | `\<<`  |
|   `>`    |        `greater` |
|   `<`    |           `less` |
|   `>=`   |  `greater_equal` |
|   `<=`   |     `less_equal` |
|   `==`   |           equal? |
|   `≈`    |          approx? |
|  `===`   |      equivalent? |
|   `≡`    | ---------------- | ---------- | ----------------- | `\>>`  |
|   `!=`   |        not_equal |
|   `≠`    | ---------------- | ---------- | ----------------- | `\!=`  |
|  `=!=`   |                ? |
|   `≢`    | ---------------- | ---------- | ----------------- | `\!==` |
|   `∈`    |         element? |        250 | Element `∈`       |
|   `^`    |                ? |            |                   |
|   `/`    |                ? |        470 | Divide `/`        |
|   `//`   |                ? |            | Quotient          |
|   `÷`    |                ? |            | QuotientRemainder |
|   `<>`   |            join? |        670 | Join              |
|   `/@`   |                ? |            |                   |
|   `/*`   |                ? |            |                   |
|   `@`    |                ? |            |                   |
|   `@@`   |                ? |            |                   |
|  `@@@`   |                ? |            |                   |
|   `@*`   |                ? |            |                   |
|   `|>`   |                ? |         70 | Postfix `//`      |
|   `->`   |                ? |        120 | Rule `//`         |
|   `:>`   |                ? |        120 | RuleDelayed `//`  |
|   `√`    |            surd? |
|   `∧`    |        logic_or? |
|   `∨`    |       logit_and? |
|   `∩`    |          set_or? |
|   `∪`    |         set_and? |
|   `⊻`    |                ? |
|   `⊼`    |                ? |
|   `⊽`    |                ? |
|   `⋖`    |                ? |
|   `⋗`    |                ? |
|   `±`    |     `plus_minus` | ---------- | ----------------- | `\+-`  |
|   `∓`    |     `minus_plus` | ---------- | ----------------- | `\-+`  |

## Infix

| Operator |      Name | Precedence | Mathematica  |
| :------: | --------: | ---------: | :----------- |
|   `?=`   |         ? |
|   `≟`    |         ? |            |              |
|   `:=`   |         ? |
|   `≔`    |         ? |            |              |
|   `=`    |         ? |            |              |
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
