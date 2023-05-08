use std::io::{Stdout, Write};

use crossterm::{QueueableCommand, style::{SetBackgroundColor, Color}, terminal::{Clear, ClearType, self}, cursor::MoveTo};

use crate::{frame::Frame, NUM_COLS, NUM_ROWS};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, cur_frame: &Frame, force: bool) {
    let (width, height) = terminal::size().unwrap();
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetBackgroundColor(Color::White)).unwrap();
    }
    

    for (x, col) in cur_frame.iter().enumerate() {
        for (y, char) in col.iter().enumerate() {
            if *char != last_frame[x][y] || force {
                stdout.queue(MoveTo((width - NUM_COLS as u16) / 2 + x as u16 , (height - NUM_ROWS as u16) / 2 + y as u16)).unwrap();
                print!("{}", *char);
            }
        }
    }
    stdout.flush().unwrap();
}