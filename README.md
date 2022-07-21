# Hypatia
This project is exploration into building an ergonomic domain specific language
for arithmetic calculations - well suited for both simple math and larger homework problems. 
Physical quantities and unit conversions is a central part of the language. Another main 
focus is the ability to evaluate your expressions into LaTeX text strings. The language will most 
likely be interacted with using an WASM based notebook environment.

## A feature highlight
* Quantities `10km + 20m`
* Hex and binary literals `0x20 bytes + 0b101 kbit`
* String interpolation `"hello \(name)!"` 
* LaTeX pretty printing `x = 20 km` => `x = 20\,\text{km}`

## Ideas for syntax and semantics
```
// Unsure of the syntax for comments
// would be nice to support markdown formatting
// so it might we wise to avoid using `#` for comments.

// SI-units will of course be included in the prelude
// but they should be easy to define yourself
unit meter
unit Ångström = 10^(-10) meter

x = 20 km
y = 30 m
z = x + y

binary_literal = 0b10101
hex_literal = 0x32ab bytes
scientific_form = 15.3e2

// Some examples of functions
f(x) = 10 + x

// Easy to declare functions
is_long_distance(distance) = distance > 20 km

min(a, b) = if a < b {
  a
} else {
  b
}

// Curly braces can be used to create a block with its own scope
area = {
  length = 20m
  height = 30m
  // the last expression of a block will also be the value of the block expression itself
  length * height 
}

// Everything in the language is an expression
foo = bar = 20

// Will most likely have some kind of postfix function application
print(20)
20 @ print
20 |> print

// All blocks (curly braces), including in if expressions will introduce their own scope
example = 20
if cond {
  example = 30
}
// That won't change the value of "example"
// instead use the assigment operator "update"
example = 20
if cond {
  update example = 30
}

// ...or more idiomatically use the if expression itself
example = if cond {
  30
} else {
  20
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
