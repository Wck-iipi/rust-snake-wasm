mod canvas;
mod movement_direction;

use canvas::Canvas;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn initialize() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    body.set_title("Snake Game with WASM");

    let canvas = Canvas::new(document, 20, 20);
    canvas.draw(5, 5, "red");
    canvas.draw(10, 10, "orange");
    canvas.draw(15, 15, "purple");
}
