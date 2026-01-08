// import { highlight_text, update, sync_scroll, check_tab } from './main.js';
import init from "../pkg/lc3_web.js";
import {
  get_tokens,
  highlight_text,
  get_vm,
  get_io,
  assemble,
} from "../pkg/lc3_web.js";
await init();

const VM = get_vm();
const IO = get_io();

const Error = Object.freeze({ NONE: 0, FAIL: 1 });

// // EVENT LISTENERS --------------------------------
const innerConsole = document.getElementById("innerConsole");
innerConsole.addEventListener("keydown", (e) => {
  e.preventDefault();
  let key = e.key;
  if (
    key == "Shift" ||
    key == "Control" ||
    key == "Alt" ||
    key == "Meta" ||
    key == "CapsLock" ||
    key == "ArrowLeft" ||
    key == "ArrowRight" ||
    key == "ArrowUp" ||
    key == "ArrowDown"
  ) {
    return;
  }

  if (key == "Enter") {
    IO.input_stream.push("\n".charCodeAt(0));
    return;
  }
  if (key == "Tab") {
    IO.input_stream.push("\t".charCodeAt(0));
    return;
  }

  IO.input_stream.push(key.charCodeAt(0));
  console.log(IO.input_stream.toString());
});

const editor = document.getElementById("editor");
editor.addEventListener("keydown", function (e) {
  if (e.key == "Tab") {
    e.preventDefault();
    var start = this.selectionStart;
    var end = this.selectionEnd;
    // set textarea value to: text before caret + tab + text after caret
    this.value =
      this.value.substring(0, start) + "\t" + this.value.substring(end);
    // put caret at right position again
    this.selectionStart = this.selectionEnd = start + 1;
  }
  console.log(this.value);
});

const runButton = document.getElementById("runButton");
runButton.addEventListener("click", (e) => {
  run();
});

function run() {
  while (!VM.registers.halt) {
    if (IO.output_stream.length > 0) {
      let out = "";
      for (c in IO.output_stream) {
        out = out + c;
      }
      innerConsole.value = innerConsole.value + out;
      IO.output_stream = [];
    }
    VM.run_single_command(IO);
  }
}

const loadAndRunButton = document.getElementById("loadAndRunButton");
loadAndRunButton.addEventListener("click", (e) => {
  let file = editor.value;
  loadAndRun(file);
});

function loadAndRun(file) {
  console.log("Hello from load and run");
  if (!loadToMachine(file)) {
    console.log("COULD NOT LOAD FILE TO MACHINE, ERROR LIKELY OCCURRED");
  }

  run();
}

function loadToMachine(file) {
  let binary = assemble(file);

  if (binary === undefined) {
    return false;
  }

  VM.registers.pc = binary[0];

  // VM.memory.clear();
  VM.memory.load_file(binary);

  return true;
}

// let textarea = document.querySelector("#editing");
// textarea.addEventListener("input", (event) => {
//     update(textarea.value);
//     sync_scroll(textarea);
// });

// textarea.addEventListener("onscroll", (event) => {
//     sync_scroll(textarea);
// });

// textarea.addEventListener("onkeydown", (event) => {
//     check_tab(textarea, event);
// });

// textarea.addEventListener("onload", (event) => {
//     update(textarea.value);
// });

/*
The following three functions and corresponding html & css are from:
https://css-tricks.com/creating-an-editable-textarea-that-supports-syntax-highlighted-code/
*/

// /**
//  * replaces text in a textarea with text in a <code> tag, formatting, and highlighting
//  *
//  * @param {string} text
//  * @returns {}
//  */
// function update(text) {
//   let result_element = document.querySelector("#highlighted-content");

//   if (text[text.length - 1] == "\n") {
//     text += " ";
//   }

//   text = text
//     .replace(new RegExp("&", "g"), "&")
//     .replace(new RegExp("<", "g"), "<");

//   result_element.innerHTML = highlight_text(text);
// }

// /**
//  * replaces text in a textarea with text in a <code> tag, formatting, and highlighting
//  *
//  * @param {HTMLTextAreaElement} element
//  * @returns {}
//  */
// function sync_scroll(element) {
//     console.log(`SYNC_SCROLL: ${element}`);
//     let result_element = document.querySelector("#highlighting");

//     result_element.scrollTop = element.scrollTop;
//     result_element.scrollLeft = element.scrollLeft;
// }

// function check_tab(element, event) {
//   console.log(`CHECK_TAB: ${element}, ${event}`);
//   let code = element.value;
//   if (event.key == "Tab") {
//     event.preventDefault(); // stop normal
//     let before_tab = code.slice(0, element.selectionStart); // text before tab
//     let after_tab = code.slice(element.selectionEnd, element.value.length); // text after tab
//     let cursor_pos = element.selectionEnd + 1; // where cursor moves after tab - moving forward by 1 char to after tab
//     element.value = before_tab + "\t" + after_tab; // add tab char
//     // move cursor
//     element.selectionStart = cursor_pos;
//     element.selectionEnd = cursor_pos;
//     update(element.value); // Update text to include indent
//   }
// }

// export { highlight_text, update, sync_scroll, check_tab };
// export { update };
