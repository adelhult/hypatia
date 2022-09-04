# Hypatia

Hypatia is a experimental language and notebook environment well suited for both
simple calculations and larger homework problems.

[Try it in your browser](https://adelhult.github.io/hypatia)

Many features are yet to be implemented, see the
[roadmap](https://github.com/adelhult/Hypatia/issues/1).

## A taste of the language

```
// Physical quantites
totalTime = 20 s + 3 h

// Declare your own units
// (although most should already be included in the prelude)
unit mile mi = 1 609 m

// Variables
x = 2 m

// Block expressions
area = {
	height = 2 km
	width = 10 m
	height * width
}

// Conditionals
foo = if true {
	20
} else {
	30
}

// Hex and binary literals (todo) 
data = 0x20 bytes
header = 0b1010 bit

// Functions (todo)
f(x) = x + 10
isVeryLong(length) = length > 2 km

// Strings (todo)
name = "Hypatia"
"Hi \(name)"

// Latex output (todo)
```

Note: more demos, can be found in the
[core/samples directory](https://github.com/adelhult/hypatia/tree/main/core/samples).

## Development

`core` contains the parser and interpreter written in Rust. Run cargo test to
check a test suite of sample programs.

```
cd core
cargo test      # test all of the sample files
```

`web` is the notebook interface written in Typescript using React. Run this to
try it locally.

```
cd web
npm install     # install the dependencies
npm run wasm    # build the wasm web bindings
npm run dev     # start a vite development server
```

Note: every commit to the main branch will update the online version using a
github actions workflow.
