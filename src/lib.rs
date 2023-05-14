pub mod bird;
pub mod frame;
pub mod obstacles;
pub mod render;

pub const NUM_ROWS: usize = 25;
pub const NUM_COLS: usize = 80;
pub const PIPE_WIDTH: usize = 15;
pub const MIN_PIPE_HEIGHT: usize = 4;
pub const MAX_PIPE_HEIGHT: usize = NUM_ROWS - (MIN_PIPE_HEIGHT * 2);
