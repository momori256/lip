import { Repl } from "lip";

const html = String.raw;

const createElementFromHtml = (html) => {
  const template = document.createElement("template");
  template.innerHTML = html.trim();
  return template.content.firstElementChild;
};

class Result {
  constructor(isSuccess, expr, value) {
    this.isSuccess = isSuccess;
    this.expr = expr;
    this.value = value;
  }

  element() {
    const { isSuccess, expr, value } = this;
    return createElementFromHtml(html`
      <li>
        <pre>${expr}</pre>
        <p class="${isSuccess ? "ok" : "err"}">=> ${value}</p>
      </li>
    `);
  }
}

const repl = Repl.new();
const results = document.querySelector("ul.results");
const textarea = document.querySelector("textarea#text");

{
  const result = new Result(
    false,
    "(& T T F",
    'error: Parse("call is not closed with `)`")',
  );
  results.prepend(result.element());
}
{
  const result = new Result(
    true,
    "(def nand (lambda (a b) (^ (& a b))))",
    'lambda: ["a", "b"] -> Call(Operator(Not), [Call(Operator(And), [Ident("a"), Ident("b")])])',
  );
  results.prepend(result.element());
}

textarea.value = `(if (& T T F)
  (^ F)
  (| T F F))`;

document.querySelector("button.run").addEventListener("click", (e) => {
  const input = textarea.value;
  if (!input.length) {
    return;
  }
  textarea.value = "";
  let [isSuccess, expr, value] = [true, input, undefined];
  try {
    value = repl.eval(input);
  } catch (ex) {
    value = `error: ${ex}`;
    isSuccess = false;
  }
  const result = new Result(isSuccess, expr, value);
  results.prepend(result.element());
});

document.querySelector("button.clear").addEventListener("click", (e) => {
  results.innerHTML = "";
});
