use std::error::Error;
use std::thread;
use std::sync::mpsc;
use invaders::frame;
use invaders::frame::new_frame;
use invaders::frame::Drawable;
use invaders::player::Player;
use invaders::render;
use rusty_audio::Audio;
use std::io;
use crossterm::terminal;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::cursor::Hide;
use crossterm::cursor::Show;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::ExecutableCommand;

use core::time::Duration;
// use std::time::Duration;

use crossterm::event;
use crossterm::event::Event;

use crossterm::event::KeyCode;





fn main() -> Result <(), Box<dyn Error>>{
    let mut audio = Audio::new();
    audio.add("explode","explode.wav");
    audio.add("lose","lose.wav");
    audio.add("move","move.wav");
    audio.add("pew","pew.wav");
    audio.add("startup","startup.wav");
    audio.add("win","win.wav");

    audio.play("startup");

    // Terminal 

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread

    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop{
            let curr_frame = match render_rx.recv(){
                Ok(x) => x,
                Err(_) => break,

            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
      
    });

    // Game loop

    let mut player = Player::new();
    let mut instant = std::time::Instant::now();



    'gameloop: loop {
                    // Per-frame init

                    let delta = instant.elapsed();
                    instant = std::time::Instant::now();
                    let mut curr_frame = new_frame();

                    // Input
                    while event::poll(Duration::default())? {
                        if let Event::Key(key_event) = event::read()? {
                            match key_event.code {
                                KeyCode::Left => {
                                    player.move_left();
                                    audio.play("move");
                                }

                                KeyCode::Right => {
                                    player.move_right();
                                    audio.play("move");
                                }

                                KeyCode::Char(' ') | KeyCode::Enter => {
                                    if player.shoot() {
                                        audio.play("pew");
                                    }
                                }

                                KeyCode::Esc | KeyCode::Char('q') => {
                                    audio.play("lose");
                                    break 'gameloop;
                                }

                                _ => {}
                            }
                        }
                    }

                    // Updates

                    player.update(delta);

                    // Draw & render

                    player.draw(&mut curr_frame);
                    let _ = render_tx.send(curr_frame);
                    thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);

    render_handle.join().unwrap();

    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;


    Ok(())
}
