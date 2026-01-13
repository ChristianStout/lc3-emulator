use std::collections::VecDeque;

use crate::log;
use lc3::io::IOTarget;
use serde::Deserialize;
use serde::Serialize;
use tsify::Tsify;
use typetag;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WebIO {
    pub output_stream: VecDeque<char>,
    pub input_stream: VecDeque<char>,
}

#[wasm_bindgen]
impl WebIO {
    pub fn new() -> WebIO {
        WebIO {
            output_stream: VecDeque::new(),
            input_stream: VecDeque::new(),
        }
    }
}

#[typetag::serde]
impl IOTarget for WebIO {
    fn get_char(&mut self) -> Option<char> {
        return pop_from_input_stream();
    }

    fn put_char(&mut self, c: char) {
        push_char_to_output(c);
    }
}

#[wasm_bindgen]
pub fn pop_from_input_stream() -> Option<char> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let input_stream = document
        .get_element_by_id("inputStream")
        .expect("Was not able to get the element `inputStream` in WebIO::get_console_value");

    let text_area = input_stream
        .dyn_into::<HtmlTextAreaElement>()
        .expect("Expected to be able to get the input stream value");

    let input_stream_value = text_area.value();
    log(&format!("{:?}", input_stream_value));

    if input_stream_value.len() == 0 {
        return None;
    }

    let c = input_stream_value
        .chars()
        .next()
        .expect("Was not able to get the first value of the string in pop_from_input_string()");

    let new_stream = input_stream_value
        .get(1..)
        .expect("expected to be able to get a substring from the input stream value in pop_from_input_stream()");

    text_area.set_value(new_stream);

    return Some(c);
}

#[wasm_bindgen]
pub fn push_char_to_output(c: char) {
    log("called push_char_to_output()");
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    log("got the document");

    let inner_console = document
        .get_element_by_id("innerConsole")
        .expect("Was not able to get the element `innerConsole` in WebIO::get_console_value");

    log("Got innerConsole element");

    let text_area = inner_console
        .dyn_into::<HtmlTextAreaElement>()
        .expect("Expected to be able to get the input stream value");

    let mut console_value = text_area.value();
    log(&format!("Got textarea : {console_value}"));

    console_value.push(c);
    log(&format!("New innerConsole value : {console_value}"));

    text_area.set_value(&console_value);
}
