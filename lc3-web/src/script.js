// import { highlight_text, update, sync_scroll, check_tab } from './main.js';
import init from "../pkg/lc3_web.js";
import { get_tokens, highlight_text, assemble, WebVM } from "../pkg/lc3_web.js";
import { VM, jumpToPc, render_memory } from "./before.js";
await init();

const Error = Object.freeze({ NONE: 0, FAIL: 1 });

// EVENT LISTENERS -----------------------------------------
const inputStream = document.getElementById("inputStream");
// inputStream.addEventListener("keydown", async (e) => {});

async function inputToStream(e) {
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
    key == "ArrowDown" ||
    key == "Backspace"
  ) {
    return;
  }

  let c = key;
  if (key == "Enter") {
    c = "\n";
  }
  if (key == "Tab") {
    c = "\t";
  }

  if (await VM.is_awaiting_input()) {
    await VM.set_reg(0, c.charCodeAt(0));
    await VM.set_awaiting_input(false);
    enableStepAndRunButtons();
    if (VM.get_is_running()) {
      await run(); // continue execution
      return;
    }
    return;
  }
  inputStream.value += c;
}

const innerConsole = document.getElementById("innerConsole");
innerConsole.addEventListener("keydown", async (e) => {
  e.preventDefault();
  await inputToStream(e);
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
  localStorage.setItem("file", this.value);
});

const runButton = document.getElementById("runButton");
runButton.addEventListener("click", async (e) => {
  await run();
});

async function run() {
  VM.set_is_running(true);
  await VM.set_awaiting_input(false);

  if (!VM.get_program_loaded()) {
    console.log("No program loaded");
    return;
  }

  while (!VM.is_halted()) {
    await stepInstruction();

    let awaiting_input = await VM.is_awaiting_input();
    if (awaiting_input) {
      updateRenderSidePanel();
      return;
    }
  }

  if (VM.is_halted) {
    updateRenderSidePanel();
    VM.set_is_running(false);
    disableStepAndRunButtons();
  }
}

const loadAndRunButton = document.getElementById("loadAndRunButton");
loadAndRunButton.addEventListener("click", async (e) => {
  let file = editor.value;
  await loadAndRun(file);
});

async function loadAndRun(file) {
  console.log("Hello from load and run");
  let result = await loadToMachine(file);
  if (!result) {
    console.log("COULD NOT LOAD FILE TO MACHINE, ERROR LIKELY OCCURRED");
    return;
  }

  await run();
}

const stepButton = document.getElementById("stepButton");
stepButton.addEventListener("click", (e) => {
  stepInstruction();
});

const loadButton = document.getElementById("loadButton");
loadButton.addEventListener("click", async (e) => {
  let file = editor.value;
  let result = await loadToMachine(file);
  if (!result) {
    console.log("COULD NOT LOAD FILE TO MACHINE, ERROR LIKELY OCCURRED");
    return;
  }
  jumpToPc();
});

async function loadToMachine(file) {
  let binary = assemble(file);

  if (binary === undefined) {
    return false;
  }

  VM.set_pc(binary[0]);

  await VM.reset_machine();
  VM.load_into_memory(binary);

  updateRegisterDisplay();
  render_memory(true);

  VM.set_program_loaded(true);
  enableStepAndRunButtons();

  return true;
}

async function stepInstruction() {
  if (VM.is_halted()) {
    return;
  }

  // const isAwaitingInput = await VM.is_awaiting_input();
  // if (isAwaitingInput) {
  //   stepButton.disabled = true;
  // }

  let stepResult = await VM.step();

  if (!VM.get_is_running()) {
    updateRenderSidePanel();
  }

  if (await VM.is_awaiting_input()) {
    disableStepAndRunButtons();
    return;
  }

  // let result = await VM.is_awaiting_input();
  // if (isAwaitingInput && result) {
  //   await VM.set_awaiting_input(false);
  // }
}

function updateRenderSidePanel() {
  updateRegisterDisplay();
  render_memory(true);
  jumpToPc();
}

function updateRegisterDisplay() {
  let r0 = VM.get_reg_value_as_hex(0);
  let r1 = VM.get_reg_value_as_hex(1);
  let r2 = VM.get_reg_value_as_hex(2);
  let r3 = VM.get_reg_value_as_hex(3);
  let r4 = VM.get_reg_value_as_hex(4);
  let r5 = VM.get_reg_value_as_hex(5);
  let r6 = VM.get_reg_value_as_hex(6);
  let r7 = VM.get_reg_value_as_hex(7);
  let pc = VM.get_pc_value_as_hex();
  let ir = VM.get_ir_value_as_hex();
  let n = VM.get_n_reg_value();
  let z = VM.get_z_reg_value();
  let p = VM.get_p_reg_value();
  let halt = VM.is_halted();

  const bool_to_int = (b) => {
    if (b) {
      return 1;
    }
    return 0;
  };

  const handle_enable_bit = (e, b, value) => {
    e.classList.remove("bit-enabled");
    if (b) {
      e.classList.add("bit-enabled");
    }
    e.innerHTML = value;
  };

  document.getElementById("r0Value").innerHTML = r0;
  document.getElementById("r1Value").innerHTML = r1;
  document.getElementById("r2Value").innerHTML = r2;
  document.getElementById("r3Value").innerHTML = r3;
  document.getElementById("r4Value").innerHTML = r4;
  document.getElementById("r5Value").innerHTML = r5;
  document.getElementById("r6Value").innerHTML = r6;
  document.getElementById("r7Value").innerHTML = r7;
  document.getElementById("pcValue").innerHTML = pc;
  document.getElementById("irValue").innerHTML = ir;

  handle_enable_bit(document.getElementById("nRegValue"), n, bool_to_int(n));
  handle_enable_bit(document.getElementById("zRegValue"), z, bool_to_int(z));
  handle_enable_bit(document.getElementById("pRegValue"), p, bool_to_int(p));
  handle_enable_bit(document.getElementById("haltValue"), halt, halt);
}

const clearConsoleButton = document.getElementById("clearConsoleButton");
clearConsoleButton.addEventListener("click", async (e) => {
  innerConsole.value = "";
});

const clearInputBufferButton = document.getElementById(
  "clearInputStreamButton",
);
clearInputBufferButton.addEventListener("click", async (e) => {
  inputStream.value = "";
});

function disableStepAndRunButtons() {
  stepButton.disabled = true;
  runButton.disabled = true;
}

function enableStepAndRunButtons() {
  stepButton.disabled = false;
  runButton.disabled = false;
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
