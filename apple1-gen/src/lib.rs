use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello from apple1-gen WASM!".to_string()
}

#[wasm_bindgen]
pub fn main_loop() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal = document.get_element_by_id("terminal").expect("should have terminal div");
    let mut output = String::new();
    output.push_str("Apple1 Emulator Ready\n");
    output.push_str("\n");
    output.push_str("> ");
    terminal.set_inner_html(&format!("<pre>{}</pre>", output));
    // For 60 FPS main loop, use requestAnimationFrame or setInterval in JS for now
}
