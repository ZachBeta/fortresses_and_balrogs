# Entity Components (MVP)

## Overview
This document describes the minimal set of components for entities in the MVP. Components are small, reusable data structures attached to entities in the ECS.

## Core Components

### Position
- Stores the (x, y) coordinates of an entity on the map.
```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
```

### Renderable
- Stores the glyph and color used to display the entity in the terminal UI.
```rust
#[derive(Component, Debug, Clone)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}
```

### Player
- Marker component for the player entity.
```rust
#[derive(Component, Debug, Default)]
pub struct Player;
```

### Creature
- Marker component for non-player creatures (NPCs, monsters).
```rust
#[derive(Component, Debug, Default)]
pub struct Creature;
```

### Health (Optional for MVP)
- Tracks current and maximum health for living entities.
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
```

## Extensibility
- Additional components (Inventory, Faction, StatusEffects, etc.) can be added in future phases.

## Best Practices
- Keep components focused and data-only.
- Use marker components for simple flags (e.g., Player, Creature).
- Prefer composition over inheritance.

---
This minimal set supports player, NPCs, and basic game logic for the MVP.
