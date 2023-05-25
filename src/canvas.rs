use wasm_bindgen::prelude::*;

use web_sys::Document;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

pub struct Canvas {
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(document: Document, width: u32, height: u32) -> Canvas {
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;
        Canvas {
            canvas,
            context,
            scaled_width,
            scaled_height,
            width,
            height,
        }
    }

    pub fn clear(&self) {
        self.context.set_fill_style(&JsValue::from_str("white"));
        self.context.clear_rect(
            0.0,
            0.0,
            self.scaled_width as f64,
            self.scaled_height as f64,
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
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.fill_text(text, x as f64, y as f64);
    }
}
