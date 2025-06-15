# ECS Architecture

## Overview
The Entity-Component-System (ECS) architecture will be implemented using the `specs` crate, which provides a high-performance ECS implementation in Rust.

## Core Components

### World
Central container for all ECS data and systems.

### Entities
Simple identifiers that group components together.

### Components
Data-only structures that represent specific aspects of game objects.

### Systems
Logic that operates on components.

## Implementation

### Component Definitions
```rust
// Basic components
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct Renderable {
    glyph: char,
    fg: Color,
    bg: Color,
}

// Game-specific components
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Player;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Health {
    current: i32,
    max: i32,
}
```

### System Implementation
```rust
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut positions, velocities, time): Self::SystemData) {
        use specs::Join;
        
        for (pos, vel) in (&mut positions, &velocities).join() {
            pos.x += (vel.dx * time.delta_seconds()) as i32;
            pos.y += (vel.dy * time.delta_seconds()) as i32;
        }
    }
}
```

### World Setup
```rust
fn create_world() -> World {
    let mut world = World::new();
    
    // Register components
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Health>();
    
    // Create dispatcher for systems
    let mut dispatcher = DispatcherBuilder::new()
        .with(MovementSystem, "movement", &[])
        .with(RenderSystem, "render", &[])
        .build();
    
    dispatcher.setup(&mut world.res);
    
    world
}
```

## Resource Management

### Common Resources
- `DeltaTime`: Time since last update
- `InputState`: Current input state
- `RandomNumberGenerator`: Centralized RNG
- `Map`: Game world state

### Resource Definition
```rust
pub struct DeltaTime(pub f32);

impl Default for DeltaTime {
    fn default() -> Self {
        DeltaTime(1.0 / 60.0)  // Default to 60 FPS
    }
}
```

## System Dependencies

### Execution Order
1. Input processing
2. AI and decision making
3. Physics and movement
4. Combat resolution
5. State updates
6. Rendering

### Parallel Execution
- Systems without conflicting data can run in parallel
- Use `par_join()` for parallel iteration
- Be mindful of mutable access patterns

## Best Practices

### Component Design
- Keep components small and focused
- Use `Option` for optional components
- Prefer multiple simple components over complex ones
- Use marker components for behavior flags

### System Design
- Keep systems small and focused
- Minimize system dependencies
- Use resources for global state
- Implement `System` trait for custom systems

## Performance Considerations
- Use appropriate storage types
- Leverage parallel iteration
- Cache lookups when possible
- Profile and optimize hot paths

## Testing
- Unit test systems in isolation
- Test component interactions
- Benchmark critical systems
- Verify system execution order

## Future Extensions
- Save/load system
- Network replication
- Runtime system graph editing
- Hot reloading of components and systems
