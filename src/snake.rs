use js_sys::Math;
use web_sys::console;

use crate::canvas::Canvas;
use crate::movement_direction::MovementDirection;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

#[derive(Debug)]
pub struct Snake {
    head: Block,
    tail: Vec<Block>,
    height: u32,
    width: u32,
    direction: Option<MovementDirection>,
    next_direction: Option<MovementDirection>,
    last_direction: MovementDirection,
    food: Block,
    total_score: u32,
    start_game: bool,
    max_score: u32,
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        let head_x: u32 = (Math::random() * f64::from(width)) as u32;
        let head_y: u32 = (Math::random() * f64::from(height)) as u32;
        let head = Block(head_x, head_y);

        let food_x: u32 = (Math::random() * f64::from(width)) as u32;
        let food_y: u32 = (Math::random() * f64::from(height)) as u32;

        let food = Block(food_x, food_y);
        let tail = Vec::new();

        Snake {
            head,
            tail,
            food,
            height,
            width,
            direction: None,
            next_direction: None,
            last_direction: MovementDirection::Right,
            total_score: 0,
            start_game: false,
            max_score: 0,
        }
    }
    pub fn change_direction(&mut self, direction: MovementDirection) {
        if !self.last_direction.is_in_opposite_direction(direction) && self.direction.is_none() {
            self.direction = Some(direction)
        } else if self
            .direction
            .iter()
            .any(|d| !d.is_in_opposite_direction(direction))
        {
            self.next_direction = Some(direction)
        }
    }

    pub fn change_screen_main_menu(&mut self, x: i32, y: i32) {
        console::log_1(&format!("I was called brother").into());
        console::log_1(&format!("x: {}, y: {}", x, y).into());
        if x >= 110 && x <= 413 && y >= 350 && y <= 450 {
            self.start_game = true;
        }
    }

    pub fn update(&mut self) {
        let direction = self.direction.unwrap_or(self.last_direction);
        self.last_direction = direction;

        let new_head = match direction {
            MovementDirection::Up => Block(
                (self.head.0) % self.width,
                (self.head.1.checked_sub(1).unwrap_or(self.height - 1)) % self.height,
            ),
            MovementDirection::Down => {
                Block((self.head.0) % self.width, (self.head.1 + 1) % self.height)
            }
            MovementDirection::Right => {
                Block((self.head.0 + 1) % self.width, (self.head.1) % self.height)
            }
            MovementDirection::Left => Block(
                (self.head.0.checked_sub(1).unwrap_or(self.width - 1)) % self.width,
                (self.head.1) % self.height,
            ),
        };

        self.tail.insert(0, self.head);
        let last_end = self.tail.pop();

        if self.tail.contains(&new_head) {
            if self.total_score > self.max_score {
                self.max_score = self.total_score;
            }
            let temporary_store = self.max_score;
            *self = Snake::new(self.width, self.height);
            self.max_score = temporary_store;
        }

        self.head = new_head;
        if self.head == self.food {
            self.total_score += 1;
            let mut food = self.food;
            while food == self.head || self.tail.contains(&food) {
                let food_x: u32 = (Math::random() * f64::from(self.width)) as u32;
                let food_y: u32 = (Math::random() * f64::from(self.height)) as u32;
                food = Block(food_x, food_y);
            }
            self.food = food;
            last_end.map(|x| self.tail.push(x));
        }
        self.direction = self.next_direction.take();
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.clear();
        if !self.start_game {
            canvas.draw_text(
                format!("Max score: {}", self.max_score.to_string()).as_str(),
                canvas.scaled_width,
                canvas.scaled_height + 80,
                "black",
            );
            canvas.draw_rect(
                canvas.scaled_width + 100,
                canvas.scaled_height + 335,
                300,
                100,
                "red",
            );
            canvas.draw_text(
                "Start the game",
                canvas.scaled_width + 150,
                canvas.scaled_height + 400,
                "black",
            );
        } else {
            canvas.draw(self.head.0, self.head.1, "green");
            for &Block(x, y) in &self.tail {
                canvas.draw(x, y, "lightgreen ");
            }
            canvas.draw(self.food.0, self.food.1, "red");
            canvas.draw_text(
                self.total_score.to_string().as_str(),
                canvas.scaled_width + 470,
                canvas.scaled_height + 40,
                "black",
            );
        }
    }
}
