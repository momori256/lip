import { Repl } from "lip";

const repl = Repl.new();
console.log(repl.eval("(def nand (lambda (a b) (^ (& a b))))"));
console.log(repl.eval("(nand T T)"));
