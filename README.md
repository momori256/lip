# lip

`lip` is an interpreted language similar to Lisp written in Rust, inspired by an article [Risp (in (Rust) (Lisp))](https://stopa.io/post/222) by Stepan Parunashvili.

## Usage

`cargo run` launches REPL (an interactive environment for Read-Evaluate-Print Loop).

```
$ cargo run
lip> (& T T F)
false
lip> :exit
```
