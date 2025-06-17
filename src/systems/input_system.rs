use specs::prelude::*;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

use crate::components::{Player, Position};

// GameControl resource to track quit
#[derive(Default)]
pub struct GameControl {
    pub quit: bool,
}

pub struct InputProcessor;

use specs::{WriteExpect};

impl<'a> System<'a> for InputProcessor {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteExpect<'a, GameControl>,
    );

    fn run(&mut self, (mut positions, players, mut game_control): Self::SystemData) {
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                for (pos, _) in (&mut positions, &players).join() {
                    match code {
                        KeyCode::Left | KeyCode::Char('a') => {
                            if pos.x > 0 {
                                pos.x -= 1;
                            }
                        }
                        KeyCode::Right | KeyCode::Char('d') => {
                            pos.x += 1;
                        }
                        KeyCode::Up | KeyCode::Char('w') => {
                            if pos.y > 0 {
                                pos.y -= 1;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('s') => {
                            pos.y += 1;
                        }
                        KeyCode::Esc => {
                            game_control.quit = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
