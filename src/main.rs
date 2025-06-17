mod components;
mod systems;

use specs::prelude::*;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::MoveTo,
    style::Print,
};
use std::io::stdout;
use specs::prelude::*;

use components::*;
use systems::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;

    // Create ECS world and register components
    let mut world = World::new();
    register_components(&mut world);
    // Insert GameControl resource
    world.insert(systems::input_system::GameControl::default());

    // Create systems
    let mut dispatcher = DispatcherBuilder::new()
        .with(RenderSystem, "render", &[])
        .with(InputProcessor, "input", &["render"])
        .build();
    dispatcher.setup(&mut world);

    // Spawn player
    world
        .create_entity()
        .with(Position { x: 0, y: 0 })
        .with(Renderable {
            glyph: '@',
            fg: "white",
            bg: "green",
        })
        .with(Player)
        .build();

    // Spawn Ringwraith
    world
        .create_entity()
        .with(Position { x: 10, y: 10 })
        .with(Renderable {
            glyph: 'N',
            fg: "black",
            bg: "gray",
        })
        .with(Ringwraith)
        .build();

    // Main game loop
    let mut quit = false;
    while !quit {
        // Run all systems
        dispatcher.dispatch(&world);
        world.maintain();

        // Check if we should quit
        let game_control = world.read_resource::<systems::input_system::GameControl>();
        quit = game_control.quit;

    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print("\nExited game.\n")
    )?;
    Ok(())
}
