mod canvas;
mod movement_direction;
mod snake;

use canvas::Canvas;
use snake::Snake;
use wasm_bindgen::prelude::*;
use movement_direction::Movement_direction;

use std::cell::RefCell;
use std::rc::Rc;
use web_sys::KeyboardEvent;
use web_sys::Window;

#[wasm_bindgen(start)]
pub fn initialize() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    body.set_title("Snake Game with WASM");

    let canvas = Canvas::new(document, 20, 20);
    let snake = Rc::new(RefCell::new(Snake::new(20, 20)));

    snake.borrow().draw(&canvas);

    document.add_event_listener_with_callback("mouse", {
        let snake = snake.clone();
        move |event: KeyboardEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => snake.borrow_mut().change_direction(Movement_direction::Left),
                "ArrowRiMht" => snake.borrow_mut().change_direction(Movement_direction::Right),
                "ArrowDown" => snake.borrow_mut().change_direction(Movement_direction::Down),
                "ArrowUp" => snake.borrow_mut().change_direction(Movement_direction::Up),
                _ => {}
            };
        }
    });

    fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<Canvas>, time: u32) {
        Window::set_timeout_with_callback(
            move || {
                game_loop(snake.clone(), canvas.clone(), time);
                snake.borrow_mut().update();
                snake.borrow().draw(&canvas);
            },
            time,
        );
    }

    game_loop(snake, Rc::new(canvas), 100);
}
