# lip ![example workflow](https://github.com/momori256/lip/actions/workflows/general.yml/badge.svg)

**lip** is a logical operation language with a Lisp-like syntax ([Live Demo](https://momori256.github.io/lip/lip/www/)). This repository contains CLI and WebAssembly (WASM) runtimes implemented in Rust.

`lip` is inspired by the article [Risp (in (Rust) (Lisp))](https://stopa.io/post/222) by Stepan Parunashvili.

## lp

`lp` is a mini `lip` for illustration created by extracting essential parts of `lip`. The implementation and deployment are discussed in the following articles:

- [Building a Lisp-like Language from Scratch in Rust](https://momori-nakano.hashnode.dev/building-a-lisp-like-language-from-scratch-in-rust)
- [Deploying a Rust WebAssembly (WASM) App to GitHub Pages](https://momori-nakano.hashnode.dev/deploying-a-rust-wasm-app-to-github-pages)

## Examples

**Literal (T = true, F = false)**

```
T
```

**Logical operations (not, and, or)**

```lisp
(^ (& T (| F F T)))
```

**Branching**

```lisp
(if (& T T F)
  (^ F)
  (| T F F))
```

**Defining variable**

```lisp
(def nand (lambda (a b) (^ (& a b))))
(nand T T)
```

## Usage

### CLI

Run `cargo run` to launch REPL (an interactive environment for Read-Evaluate-Print Loop) in the terminal.  

- `:exit` exits from the REPL.
- `:env` prints the current environment.

```
$ cargo run
lip> (& T T F)
false
lip> :exit
```

### WASM

Explore the [Live demo](https://momori256.github.io/lip/lip/www/) via a browser.

Screenshot:
![Screenshot of WASM version](https://github.com/momori256/lip/assets/90558309/aece5b0a-1d26-4e74-b18e-42a3a3ef08c8)

## Development

`lip` codes are in `lip/lip` directory.

### CLI

- Run `cargo run` to build and launch the REPL.
- Execute `cargo test` to run tests. Unit tests are in each file in `src`, and integration tests are in the `tests/it` directory.

### WASM

- Build using `./cmd.sh build`. This utilizes [wasm-pack](https://github.com/rustwasm/wasm-pack), and the output is stored in `www/pkg`.

- Launch an HTTP server with `./cmd.sh start` to host the web app on http://localhost:8080/. This command leverages [miniserve](https://github.com/svenstaro/miniserve). Install it using `cargo install miniserve` or use another HTTP server.

## Backus-Naua Form (BNF)

[BNF Playground](https://bnfplayground.pauliankline.com/?bnf=%3Cexpression%3E%20%3A%3A%3D%20%3Cbool%3E%20%7C%20%3Cidentifier%3E%20%7C%20%3Ccall%3E%20%7C%20%3Cif%3E%20%7C%20%3Clambda%3E%20%7C%20%3Cdef%3E%0A%0A%3Cbool%3E%20%3A%3A%3D%20%22T%22%20%7C%20%22F%22%0A%3Cidentifier%3E%20%3A%3A%3D%20%5Ba-z%5D%2B%0A%3Ccall%3E%20%3A%3A%3D%20%22(%22%20(%3Coperator%3E%20%7C%20%3Clambda%3E%20%7C%20%3Cidentifier%3E)%20(E%20%7C%20%22%20%22%20%3Cexpression_list%3E)%20%22)%22%0A%3Cexpression_list%3E%20%3A%3A%3D%20%3Cexpression%3E%20%7C%20%3Cexpression%3E%20(%22%20%22%20%3Cexpression%3E)*%0A%3Coperator%3E%20%3A%3A%3D%20%22%26%22%20%7C%20%22%7C%22%20%7C%20%22%5E%22%0A%3Cif%3E%20%3A%3A%3D%20%22(if%20%22%20%3Cexpression%3E%20%22%20%22%20%3Cexpression%3E%20%22%20%22%20%3Cexpression%3E%20%22)%22%0A%3Clambda%3E%20%3A%3A%3D%20%22(lambda%20%22%20%3Cargument_list%3E%20%22%20%22%20%3Cexpression%3E%20%22)%22%0A%3Cargument_list%3E%20%3A%3A%3D%20%22()%22%20%7C%20%22(%22%20%3Cidentifier%3E%20%20(%22%20%22%20%3Cidentifier%3E)*%20%22)%22%0A%3Cdef%3E%20%3A%3A%3D%20%22(def%20%22%20%3Cidentifier%3E%20%22%20%22%20%3Cexpression%3E%20%22)%22&name=Simple%20Programming%20Language)

```
<expression> ::= <bool> | <identifier> | <call> | <if> | <lambda> | <def>

<bool> ::= "T" | "F"
<identifier> ::= [a-z]+
<call> ::= "(" (<operator> | <lambda> | <identifier>) (E | " " <expression_list>) ")"
<expression_list> ::= <expression> | <expression> (" " <expression>)*
<operator> ::= "&" | "|" | "^"
<if> ::= "(if " <expression> " " <expression> " " <expression> ")"
<lambda> ::= "(lambda " <argument_list> " " <expression> ")"
<argument_list> ::= "()" | "(" <identifier>  (" " <identifier>)* ")"
<def> ::= "(def " <identifier> " " <expression> ")"
```
