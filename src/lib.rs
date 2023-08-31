mod canvas;
mod movement_direction;
mod snake;

use canvas::Canvas;
use js_sys::Function;
use movement_direction::Movement_direction;
use snake::Snake;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::KeyboardEvent;
use web_sys::window;

thread_local! {
    static GAME: Rc<RefCell<Snake>> =
    Rc::new(RefCell::new(Snake::new(40, 40)));

    static HANDLE_TICK: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
        GAME.with(|game| game.borrow_mut().update());
        render();
    }) as Box<dyn FnMut()>);

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> =
    Closure::wrap(Box::new(|evt: KeyboardEvent| GAME.with(|game| {
      let direction = match &evt.key()[..] {
        "ArrowUp" => Some(Movement_direction::Up),
        "ArrowRight" => Some(Movement_direction::Right),
        "ArrowDown" => Some(Movement_direction::Down),
        "ArrowLeft" => Some(Movement_direction::Left),
        _ => None,
      };

      if let Some(direction) = direction {
        game.borrow_mut().change_direction(direction);
      }
    })) as Box<dyn FnMut(KeyboardEvent)>)
}

#[wasm_bindgen(start)]
pub fn start() {
    HANDLE_TICK.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                200,
            )
            .unwrap_throw()
    });
    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
        .unwrap_throw()
        .add_event_listener_with_callback(
            "keydown",
            handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
        )
        .unwrap_throw();
    });

    render();
}

pub fn render() {
    GAME.with(|game| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        body.set_title("Snake Game with WASM");

        let canvas = Canvas::new("canvas", 40, 40);
        game.borrow().draw(&canvas);
    });
}
