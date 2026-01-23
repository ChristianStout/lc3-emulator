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

#[wasm_bindgen]
pub fn u16_to_ascii_rep(n: u16) -> String {
    match n {
        0 => "NUL".to_string(),
        1 => "SOH".to_string(),
        2 => "STX".to_string(),
        3 => "ETX".to_string(),
        4 => "EOT".to_string(),
        5 => "ENQ".to_string(),
        6 => "ACK".to_string(),
        7 => "BEL".to_string(),
        8 => "BS".to_string(),
        9 => "TAB".to_string(),
        10 => "LF".to_string(),
        11 => "VT".to_string(),
        12 => "FF".to_string(),
        13 => "CR".to_string(),
        14 => "SO".to_string(),
        15 => "SI".to_string(),
        16 => "DLE".to_string(),
        17 => "DC1".to_string(),
        18 => "DC2".to_string(),
        19 => "DC3".to_string(),
        20 => "DC4".to_string(),
        21 => "NAK".to_string(),
        22 => "SYN".to_string(),
        23 => "ETB".to_string(),
        24 => "CAN".to_string(),
        25 => "EM".to_string(),
        26 => "SUB".to_string(),
        27 => "ESC".to_string(),
        28 => "FS".to_string(),
        29 => "GS".to_string(),
        30 => "RS".to_string(),
        31 => "US".to_string(),
        32 => "SPACE".to_string(),
        127 => "DEL".to_string(),
        _ => String::from(n as u8 as char),
    }
}
