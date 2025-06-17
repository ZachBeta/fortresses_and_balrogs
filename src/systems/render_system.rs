use specs::{Join, ReadStorage, System};
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
    );

    fn run(&mut self, (positions, renderables): Self::SystemData) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();

        // Render entities
        for (pos, rend) in (&positions, &renderables).join() {
            execute!(
                stdout,
                MoveTo(pos.x as u16, pos.y as u16),
                SetForegroundColor(color_from_str(rend.fg)),
                SetBackgroundColor(color_from_str(rend.bg)),
                Print(rend.glyph),
                ResetColor
            )
            .unwrap();
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
    }
}
