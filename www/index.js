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
        <p>${expr}</p>
        <p class="${isSuccess ? "ok" : "ng"}">=> ${value}</p>
      </li>
    `);
  }
}

const repl = Repl.new();
const results = document.querySelector("ul.results");

document.querySelector("button.run").addEventListener("click", (e) => {
  const input = document.querySelector("textarea#text").value;
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
