import init from "../pkg/lc3_web.js";
import { make_memory_table } from "../pkg/lc3_web.js";
await init();
await make_memory_table();

let th = localStorage.getItem("theme");
if (th === null) {
  th = "dark";
}

const select = document.getElementById("themeSelect");
select.theme = th;
document.body.dataset.theme = select.value; // to make sure that the default is loaded on init
console.log("Initialized");
select.addEventListener("change", () => {
  document.body.dataset.theme = select.value;
  localStorage.setItem("theme", select.value);
});

const inputStream = document.getElementById("inputStream");
inputStream.value = "";

const innerConsole = document.getElementById("innerConsole");
innerConsole.value = "";
