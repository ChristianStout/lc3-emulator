import init from "../pkg/lc3_web.js";
import { make_memory_table, WebVM, u16_to_ascii_rep } from "../pkg/lc3_web.js";
await init();
// await make_memory_table();

const VM = new WebVM();

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

const editor = document.getElementById("editor");
let editor_contents = localStorage.getItem("file");
if (editor_contents == "") {
  editor_contents = `.orig x3000

                  br          begin

  prompt          .stringz    "\nwill you give this repo a star? (y/n) > "

  begin           lea         r0, prompt
                  in
                  out
                  br          calc

  char_y          .fill       #121
  char_n          .fill       #110

  calc            ld          r1, char_y
                  not         r1, r1
                  add         r1, r1, #1
                  add         r1, r1, r0
                  brz         thank
                  ld          r1, char_n
                  not         r1, r1
                  add         r1, r1, #1
                  add         r1, r1, r0
                  brz         scold

                  lea         r0, hmm
                  puts
                  br          begin
  hmm             .stringz    "\n?"

  thx_msg         .stringz    "\nwow, tysm :) <3\n"
  thank           lea         r0, thx_msg
                  puts
                  halt

  bad_msg         .stringz    "\nhow dare you"
  scold           lea         r0, bad_msg
                  puts
                  br          begin
                  halt

  .end
`;
}
editor.value = editor_contents;

// Memory View

const ROW_HEIGHT = 20;
const TOTAL_ROWS = 65536;
const BUFFER = 10;

const memory_container = document.getElementById("memoryBody");
const rowsContainer = document.getElementById("rows");
const topSpacer = document.getElementById("topSpacer");
const bottomSpacer = document.getElementById("bottomSpacer");

function render_memory() {
  const scrollTop = memory_container.scrollTop;
  const viewportHeight = memory_container.clientHeight;

  const firstVisible = Math.floor(scrollTop / ROW_HEIGHT);
  const visibleCount = Math.ceil(viewportHeight / ROW_HEIGHT);

  const start = Math.max(0, firstVisible - BUFFER);
  const end = Math.min(TOTAL_ROWS, firstVisible + visibleCount + BUFFER);

  topSpacer.style.height = `${start * ROW_HEIGHT}px`;
  bottomSpacer.style.height = `${(TOTAL_ROWS - end) * ROW_HEIGHT}px`;

  rowsContainer.innerHTML = "";

  for (let i = start; i < end; i++) {
    const row = document.createElement("div");
    row.className = "memory-row";

    const addr = document.createElement("div");
    addr.textContent = `x${i.toString(16).padStart(4, "0").toUpperCase()}`;
    row.appendChild(addr);

    const addr_val = VM.mem_get(i);

    const hex = document.createElement("div");
    hex.textContent = `x${addr_val.toString(16).padStart(4, "0").toUpperCase()}`;
    row.appendChild(hex);

    const dec = document.createElement("div");
    dec.textContent = `${addr_val}`;
    row.appendChild(dec);

    const ascii = document.createElement("div");
    ascii.textContent = u16_to_ascii_rep(addr_val);
    // ascii.textContent = String.fromCharCode(ascii);
    row.appendChild(ascii);

    rowsContainer.appendChild(row);
  }
}

memory_container.addEventListener("scroll", render_memory);
window.addEventListener("resize", render_memory);

render_memory();

export { VM, render_memory };
