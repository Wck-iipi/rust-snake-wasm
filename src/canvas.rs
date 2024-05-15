use wasm_bindgen::prelude::*;

use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

pub struct Canvas {
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub scaled_width: u32,
    pub scaled_height: u32,
    pub width: u32,
    pub height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        // let canvas = document().get_element_by_id("canvas").unwrap();
        let canvas = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(attr_id)
            .expect("Wrong id given");

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Not a canvas element");

        let context = canvas
            .get_context("2d")
            .expect("Context not found")
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;

        return Canvas {
            canvas,
            context,
            scaled_width,
            scaled_height,
            width,
            height,
        };
    }

    pub fn clear(&self) {
        self.context.set_fill_style(&JsValue::from_str("white"));
        self.context.clear_rect(
            0.0,
            0.0,
            (self.scaled_width * self.width) as f64,
            (self.scaled_height * self.height) as f64,
        );
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.context.set_fill_style(&JsValue::from_str(color));

        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        self.context.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.scaled_width),
            f64::from(self.scaled_height),
        );
    }

    pub fn draw_rect(&self, x: u32, y: u32, width: u32, height: u32, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context
            .fill_rect(x as f64, y as f64, width as f64, height as f64);
    }

    pub fn draw_text(&self, text: &str, x: u32, y: u32, color: &str) {
        self.context.set_font("30px Arial");
        self.context.set_fill_style(&JsValue::from_str(color));
        let _ = self.context.fill_text(text, x as f64, y as f64);
    }
}
