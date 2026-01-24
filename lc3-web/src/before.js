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

const TOTAL_ROWS = 65536;
const ROW_HEIGHT = 20;
const BUFFER = 5;

const memory_container = document.getElementById("memoryBody");
const viewport = document.getElementById("rowViewport");

// 1️⃣ Freeze scroll height ONCE
memory_container.querySelector(".scroll-spacer").style.height =
  `${TOTAL_ROWS * ROW_HEIGHT}px`;

// 2️⃣ Determine how many rows to render
const visibleCount =
  Math.ceil(memory_container.clientHeight / ROW_HEIGHT) + BUFFER * 2;

// 3️⃣ Create fixed row pool
const rows = [];
for (let i = 0; i < visibleCount; i++) {
  const row = document.createElement("div");
  row.className = "memory-row";

  const addr = document.createElement("div");
  row.appendChild(addr);

  const addr_val = VM.mem_get(i);

  const hex = document.createElement("div");
  row.appendChild(hex);

  const dec = document.createElement("div");
  row.appendChild(dec);

  const ascii = document.createElement("div");
  row.appendChild(ascii);

  row.className = "memory-row";
  viewport.appendChild(row);
  rows.push(row);
}

let lastFirst = -1;

function render_memory() {
  const scrollTop = memory_container.scrollTop;
  const firstVisible = Math.floor(scrollTop / ROW_HEIGHT);

  if (firstVisible === lastFirst) return;
  lastFirst = firstVisible;

  const start = Math.max(0, firstVisible - BUFFER);

  // 4️⃣ Move viewport instead of resizing spacers
  viewport.style.transform = `translateY(${start * ROW_HEIGHT}px)`;

  // 5️⃣ Update row contents only
  for (let i = 0; i < rows.length; i++) {
    const addr = start + i;
    const row = rows[i];
    const pc = VM.get_pc();

    row.classList.remove("mem-pc-loc");
    if (addr === pc) {
      row.classList.add("mem-pc-loc");
    }

    if (addr >= TOTAL_ROWS) {
      row.style.display = "none";
    } else {
      row.style.display = "";
      let children = row.children;

      let mem_value = VM.mem_get(addr);

      children[0].textContent = `x${addr.toString(16).padStart(4, "0").toUpperCase()}`;
      children[1].textContent = `x${mem_value.toString(16).padStart(4, "0").toUpperCase()}`;
      children[2].textContent = mem_value;
      children[3].textContent = u16_to_ascii_rep(mem_value);
    }
  }
}

memory_container.addEventListener("scroll", render_memory);
window.addEventListener("resize", render_memory);

render_memory();

export { VM, render_memory };
