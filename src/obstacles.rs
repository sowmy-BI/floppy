use std::time::Duration;

use rand::{rngs::ThreadRng, Rng};
use rusty_time::Timer;

use crate::{NUM_COLS, PIPE_WIDTH, NUM_ROWS, MIN_PIPE_HEIGHT, frame::Drawable, MAX_PIPE_HEIGHT};


pub struct Pipe {
    x: usize,
    y: usize,
    height: usize
}

pub struct Obstacles {
    pipes: [Pipe; 2],
    timer: Timer
}

impl Obstacles {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let up_height = rng.gen_range(MIN_PIPE_HEIGHT..MAX_PIPE_HEIGHT);

        let up: Pipe = Pipe { x: NUM_COLS - PIPE_WIDTH, y: 0, height: up_height };
        let down_y = MIN_PIPE_HEIGHT + up_height;
        print!("{}", down_y);
        let down : Pipe = Pipe { x: NUM_COLS - PIPE_WIDTH, y: down_y, height: NUM_ROWS -  down_y };
        Self { pipes: [up, down], timer: Timer::from_millis(300) }
    } 

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready {
            if self.pipes[0].x > 0 {
                self.pipes[0].x -= 1;
                self.pipes[1].x -= 1;
            }
            self.timer.reset();
        }
    }
}


impl Drawable for Obstacles {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for (_, pipe) in self.pipes.iter().enumerate() {
            for y_ in 0..pipe.height  {
                frame[pipe.x][pipe.y + y_] = "|";
            }
        }
    }
}