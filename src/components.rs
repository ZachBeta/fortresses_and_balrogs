use specs::prelude::*;
use specs_derive::*;
use crossterm::style::Color;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: char,
    pub fg: &'static str,
    pub bg: &'static str,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Ringwraith;

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Ringwraith>();
}

pub fn color_from_str(name: &str) -> Color {
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
