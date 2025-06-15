use specs::prelude::*;
use specs_derive::Component;
use crossterm::{execute, style::{Color, Print, SetBackgroundColor, SetForegroundColor, ResetColor}, cursor::MoveTo, terminal::{Clear, ClearType}};
use std::io::{stdout, Write};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Renderable {
    glyph: char,
    fg: &'static str,
    bg: &'static str,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Player;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Ringwraith;

fn color_from_str(name: &str) -> Color {
    match name.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "grey" | "gray" => Color::Grey,
        _ => Color::White,
    }
}

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::time::Duration;

fn main() {
    // Create ECS world
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Ringwraith>();

    // Spawn Frodo (Player)
    world.create_entity()
        .with(Position { x: 0, y: 0 })
        .with(Renderable { glyph: '@', fg: "white", bg: "green" })
        .with(Player)
        .build();

    // Spawn a Ringwraith
    world.create_entity()
        .with(Position { x: 10, y: 10 })
        .with(Renderable { glyph: 'N', fg: "black", bg: "gray" })
        .with(Ringwraith)
        .build();

    // Enter raw mode for input
    enable_raw_mode().unwrap();
    let mut stdout = stdout();
    let mut quit = false;

    while !quit {
        // Render all entities (scoped to drop immutable borrows before input)
        {
            execute!(stdout, Clear(ClearType::All)).unwrap();
            let positions = world.read_storage::<Position>();
            let renderables = world.read_storage::<Renderable>();
            for (pos, rend) in (&positions, &renderables).join() {
                execute!(
                    stdout,
                    MoveTo(pos.x as u16, pos.y as u16),
                    SetForegroundColor(color_from_str(rend.fg)),
                    SetBackgroundColor(color_from_str(rend.bg)),
                    Print(rend.glyph),
                    ResetColor
                ).unwrap();
            }
            execute!(stdout, MoveTo(0, 20), ResetColor, Print("[ESC] Quit | Use Arrow Keys or WASD to move"),).unwrap();
            stdout.flush().unwrap();
        }
        // Now input handling, after borrows are dropped
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(KeyEvent { code, modifiers: _, .. }) = event::read().unwrap() {
                let mut positions = world.write_storage::<Position>();
                let players = world.read_storage::<Player>();
                for (pos, _) in (&mut positions, &players).join() {
                    match code {
                        KeyCode::Left | KeyCode::Char('a') => {
                            if pos.x > 0 { pos.x -= 1; }
                        }
                        KeyCode::Right | KeyCode::Char('d') => {
                            pos.x += 1;
                        }
                        KeyCode::Up | KeyCode::Char('w') => {
                            if pos.y > 0 { pos.y -= 1; }
                        }
                        KeyCode::Down | KeyCode::Char('s') => {
                            pos.y += 1;
                        }
                        KeyCode::Esc => {
                            quit = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    disable_raw_mode().unwrap();
    execute!(stdout, MoveTo(0, 22), ResetColor, Print("\nExited game.\n")).unwrap();
}
