#![recursion_limit = "1024"]



use wasm_bindgen::prelude::*;
use web_sys::window;



fn main() {
    console_error_panic_hook::set_once();
    start_app();
}

fn start_app() {
    let document = document();
    let location = document.location().unwrap();
    let body = body(&document);
    let text_node = document.create_text_node(&location.pathname().unwrap());
    body.append_child(text_node.as_ref()).expect("failed to append text");
}

#[wasm_bindgen]
pub fn wasm_ffi() {
    web_sys::console::log_1(&"Something, something, something...".into());
}



fn document() -> web_sys::Document {
    window()
        .and_then(|win| win.document())
        .expect("could not access document")
}

fn body(doc: &web_sys::Document) -> web_sys::HtmlElement {
    doc.body().expect("could not access document.body")
}
