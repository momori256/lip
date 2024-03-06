# lip ![example workflow](https://github.com/momori256/lip/actions/workflows/general.yml/badge.svg)

**lip** is a logical operation language designed for performing logical operations using a Lisp-like syntax ([Live Demo](https://momori256.github.io/lip/lip/www/)). This repository contains CLI and WebAssembly (WASM) runtimes implemented in Rust.

`lip` is inspired by the article [Risp (in (Rust) (Lisp))](https://stopa.io/post/222) by Stepan Parunashvili.

## lp

`lp` is a mini-version of `lip` created for illustration. The implementation and deployment are discussed in the following articles:

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

`lip` codes are in `src/lip`.

### CLI

- Run `cargo run` to build and launch the REPL.
- Execute `cargo test` to run tests. Unit tests are in each file in `src`, and integration tests are in the `tests/it` directory.

### WASM

- Build using `./cmd.sh build`. This utilizes [wasm-pack](https://github.com/rustwasm/wasm-pack), and the output is stored in `www/pkg`.

- Launch an HTTP server with `./cmd.sh start` to host the web app on http://localhost:8080/. This command leverages [miniserve](https://github.com/svenstaro/miniserve). Install it using `cargo install miniserve` or use another HTTP server.
