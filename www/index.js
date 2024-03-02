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
      <li class="card">
        <pre>${expr}</pre>
        <p class="${isSuccess ? "ok" : "err"}">=> ${value}</p>
      </li>
    `);
  }
}

const repl = Repl.new();
const results = document.querySelector("ul.results");
const textarea = document.querySelector("textarea#text");
const dialog = document.querySelector("dialog#example");

textarea.value = `(if (& T T F)
  (^ F)
  (| T F F))`;

const run = () => {
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
};

document.addEventListener("keydown", (e) => {
  if (e.key === "Enter" && e.ctrlKey) {
    run();
  }
});

document.querySelector("button.run").addEventListener("click", (e) => run());

document.querySelector("button.clear").addEventListener("click", (e) => {
  results.innerHTML = "";
});

document.querySelector("button.example").addEventListener("click", (e) => {
  dialog.showModal();
});

document.querySelector("dialog button").addEventListener("click", (e) => {
  dialog.close();
});
