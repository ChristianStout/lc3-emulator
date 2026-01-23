pub mod highlight;
pub mod webio;
pub mod webvm;
use lc3::asm::asm::Asm;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlDivElement;
use web_sys::js_sys;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn assemble(file: String) -> Option<Vec<u16>> {
    let mut asm = Asm::new();
    return asm.run(file);
}

#[wasm_bindgen]
pub async fn make_memory_table() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let memory_table = document
        .get_element_by_id("memoryBody")
        .expect("expected to be able to get the memory body");

    for i in 0..u16::MAX {
        let table_item = document
            .create_element("div")
            .ok()
            .expect(&format!("Expected to be able to make table item #{i}"));

        table_item
            .append_child(&make_div(&document, &format!("x{:04X}", i)))
            .ok()
            .expect(&format!("Expected to be able to put addr in #{i}"));
        table_item
            .append_child(&make_div(&document, &format!("x{:04X}", 0)))
            .ok()
            .expect(&format!("Expected to be able to put hex value in in #{i}"));
        table_item
            .append_child(&make_div(&document, &format!("0")))
            .ok()
            .expect(&format!("Expected to be able to put decimal value in #{i}"));
        table_item
            .append_child(&make_div(&document, &format!("\\0")))
            .ok()
            .expect(&format!("Expected to be able to put ASCII value in #{i}"));

        table_item.set_class_name("memory-row");

        memory_table.append_child(&table_item).ok().expect(&format!(
            "Expected to be able to append the table value #{i} to memory table"
        ));
    }
}

fn make_div(document: &Document, contents: &str) -> Element {
    let div = document
        .create_element("div")
        .ok()
        .expect("Expected to be able to make a div");

    div.set_text_content(Some(contents));

    return div;
}
