use specs::{Join, ReadStorage, System, WriteExpect};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use crate::components;

use crate::components::color_from_str;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::Renderable>,
        WriteExpect<'a, crate::systems::game_log::GameLog>,
    );

    fn run(&mut self, (positions, renderables, mut game_log): Self::SystemData) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();

        // Render entities
        // Build a map grid (let's use 20x20 for now, can adjust as needed)
        const WIDTH: usize = 20;
        const HEIGHT: usize = 20;
        let mut grid = vec![vec!['.'; WIDTH]; HEIGHT];

        let mut frame_lines = Vec::new();
        for (pos, rend) in (&positions, &renderables).join() {
            // Clamp positions to grid
            if pos.x >= 0 && pos.x < WIDTH as i32 && pos.y >= 0 && pos.y < HEIGHT as i32 {
                grid[pos.y as usize][pos.x as usize] = rend.glyph;
            }
            execute!(
                stdout,
                MoveTo(pos.x as u16, pos.y as u16),
                SetForegroundColor(color_from_str(rend.fg)),
                SetBackgroundColor(color_from_str(rend.bg)),
                Print(rend.glyph),
                ResetColor
            )
            .unwrap();
            frame_lines.push(format!("{}{} '{}' fg:{} bg:{}", pos.x, pos.y, rend.glyph, rend.fg, rend.bg));
        }

        // Render UI
        execute!(
            stdout,
            MoveTo(0, 20),
            ResetColor,
            Print("[ESC] Quit | Use Arrow Keys or WASD to move"),
        )
        .unwrap();

        stdout.flush().unwrap();

        // Log this frame
        game_log.log_line("--- Frame ---");
        for row in &grid {
            let line: String = row.iter().collect();
            game_log.log_line(&line);
        }
        game_log.log_line("");
        // Optionally, still log the entity list for debugging:
        for line in frame_lines {
            game_log.log_line(&line);
        }
        game_log.log_line("");
    }
}
