use std::{io::stdout, sync::mpsc, thread, time::{Duration, Instant}};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, Result,
};

use rand::{rngs::ThreadRng};

use floppy::{
    bird::{Bird},
    frame::{self, new_frame, Drawable},
    render, obstacles::Obstacles,
};

fn main() -> Result<()> {
    let mut stdout = stdout();

    // Terminal
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_farme = frame::new_frame();
        let mut stdout = std::io::stdout();
        render::render(&mut stdout, &last_farme, &last_farme, true);

        loop {
            let cur_farme = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_farme, &cur_farme, false);
            last_farme = cur_farme
        }
    });

    let mut bird = Bird::new();
    let mut is_move_up = false;
    let mut instant = Instant::now();
    let mut rng: ThreadRng = rand::thread_rng();
    let mut obstacles = Obstacles::new(&mut rng);

    // Game loop
    'gameloop: loop {
        // per-farme init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut cur_farme: Vec<Vec<&str>> = new_frame();

        // user input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char(' ') => {
                        is_move_up = true;
                    }
                    KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,
                    _ => {},
                }
            }
        }

        // Update
        bird.update(delta, is_move_up);
        obstacles.update(delta);

        // Draw render
        bird.draw(&mut cur_farme);
        obstacles.draw(&mut cur_farme);
        let _ = render_tx.send(cur_farme);
        thread::sleep(Duration::from_millis(1));
        is_move_up = false;
    }

    // End
    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
