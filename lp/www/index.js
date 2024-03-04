import init, { Repl } from "./pkg/lp.js";

(async () => {
  await init();
  exec();
})();

function exec() {
  const input = document.querySelector("input");
  const history = document.querySelector("ol");

  document.querySelector("button").addEventListener("click", () => {
    const text = input.value;
    if (!text.length) {
      return;
    }
    let output;
    try {
      output = Repl.eval(text);
    } catch (e) {
      output = e;
    }
    const element = () => {
      const li = document.createElement("li");
      li.innerHTML = `<pre>${text}</pre> => ${output}`;
      return li;
    };
    history.appendChild(element());
    input.value = "";
  });
}
