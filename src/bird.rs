use std::time::Duration;

use rusty_time::Timer;

use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};

pub struct Bird {
    x: usize,
    y: usize,
    timer: Timer,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 4,
            y: NUM_ROWS / 2,
            timer: Timer::from_millis(180),
        }
    }

    fn move_up(&mut self) {
        if self.y >= 2 {
            self.y -= 2;
        }
    }

    fn move_down(&mut self) {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;
        }
    }

    pub fn update(&mut self, delta: Duration, is_move_up: bool) {
        self.timer.update(delta);
        if is_move_up {
            self.move_up();
        } else if self.timer.ready {
            self.move_down();
            self.timer.reset();
        }
    }
}

impl Drawable for Bird {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = "b";
    }
}
