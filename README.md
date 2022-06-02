# Hypatia
This project is exploration into building an ergonomic domain specific language
for arithmetic calculations - well suited for both simple math and larger homework problems. 
Physical quantities and unit conversions is a central part of the language. Another main 
focus is the ability to evaluate your expressions into LaTeX text strings. The language will most 
likely be interacted with using an WASM based notebook environment.

## Features
* Quantities
* Hex and binary literals
* String interpolation
* LaTeX representation

## Ideas for syntax and semantics
```
// Unsure of the syntax for comments
// would be nice to support markdown formatting
// so it might we wise to avoid using `#` for comments.

// SI-units will of course be included in the prelude
// but they should be easy to define yourself
unit meter
unit Ångström := 10^(-10) <meter>

x := 20 <km>
y := 30 <m>

binary_literal = 0b10101
hex_literal = 0x32ab <KiB>

f(x) := 10 + x

is_long_distance(distance) = distance > 20 <km>

g(y, z) := {
  if y < z {
    z
  } else {
    y
  }
}
```

## Comparison to other tools/languages
There are a lot of other great languages and tools with a similar purpose,
but I still think there is room for Hypatia to offer another set of features and trade-offs. 
Some insperation is taken from:
* [CalcuLaTeX](https://github.com/mkhan45/CalcuLaTeX)
* [Insect](https://github.com/sharkdp/insect)
* [Wolfram language](https://www.wolfram.com/language/)
* [Frink](https://frinklang.org/) 