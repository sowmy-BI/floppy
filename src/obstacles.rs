use std::time::Duration;

use rand::{rngs::ThreadRng, Rng};
use rusty_time::Timer;

use crate::{frame::Drawable, MAX_PIPE_HEIGHT, MIN_PIPE_HEIGHT, NUM_COLS, NUM_ROWS, PIPE_WIDTH};

pub struct Pipe {
    x: usize,
    y: usize,
    height: usize,
}

pub struct Obstacles {
    pipes: Vec<Pipe>,
    update_timer: Timer,
}

impl Obstacles {
    pub fn new() -> Self {
        Self {
            pipes: Vec::new(),
            update_timer: Timer::from_millis(200),
        }
    }

    pub fn add(&mut self, rng: &mut ThreadRng) {
        if self.pipes.len() == 0 || (NUM_COLS - self.pipes.last().unwrap().x) > PIPE_WIDTH {
            let up_height = rng.gen_range(MIN_PIPE_HEIGHT..MAX_PIPE_HEIGHT);
            self.pipes.push(Pipe {
                x: NUM_COLS - 1,
                y: 0,
                height: up_height,
            });
            let down_y = MIN_PIPE_HEIGHT + up_height;
            self.pipes.push(Pipe {
                x: NUM_COLS - 1,
                y: down_y,
                height: NUM_ROWS - down_y,
            });
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.update_timer.update(delta);
        if self.update_timer.ready && self.pipes.len() > 0 {
            if self.pipes[0].x == 0 {
                self.pipes.remove(0);
            }
            for pipe in self.pipes.iter_mut().filter(|p| p.x > 0) {
                pipe.x -= 1;
            }
            self.update_timer.reset();
        }
    }

    pub fn hit(&self, x: &usize, y: &usize) -> bool {
        let obstacles_count = self.pipes.len();
        if obstacles_count >= 6 {
            for index in 0..6 {
                if self.pipes[index].x == *x
                    && self.pipes[index].y <= *y
                    && self.pipes[index].y + self.pipes[index].height >= *y
                {
                    return true;
                }
            }
        }
        return false;
    }
}

impl Drawable for Obstacles {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for (_, pipe) in self.pipes.iter().enumerate() {
            for y_ in 0..pipe.height {
                frame[pipe.x][pipe.y + y_] = "|";
            }
        }
    }
}
