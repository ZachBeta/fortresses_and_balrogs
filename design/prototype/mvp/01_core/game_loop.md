# Game Loop Implementation

## Overview
The game loop is responsible for maintaining a consistent frame rate, processing input, updating game state, and rendering.

## Requirements
- Fixed timestep for consistent simulation
- Variable rendering for smooth display
- Input handling
- State management

## Implementation

### Core Loop
```rust
fn run(&mut self) -> Result<()> {
    let target_dt = Duration::from_secs_f32(1.0 / 60.0); // 60 FPS
    let mut accumulator = Duration::from_secs(0);
    let mut previous_instant = Instant::now();

    while self.running {
        // Calculate delta time
        let now = Instant::now();
        let frame_time = now.duration_since(previous_instant);
        previous_instant = now;
        
        // Cap frame time to avoid spiral of death
        let frame_time = frame_time.min(Duration::from_millis(250));
        
        // Accumulate time
        accumulator += frame_time;
        
        // Process input
        self.process_input()?;
        
        // Fixed timestep updates
        while accumulator >= target_dt {
            self.update(target_dt);
            accumulator -= target_dt;
        }
        
        // Render with interpolation
        let alpha = accumulator.as_secs_f32() / target_dt.as_secs_f32();
        self.render(alpha)?;
        
        // Yield to the OS
        std::thread::yield_now();
    }
    
    Ok(())
}
```

### State Management
- `RunState` enum for different game states
- Stack-based state management for nested states (e.g., main menu → options)
- Clean transitions between states

### Input Handling
- Event-based input system
- Support for key combinations
- Configurable key bindings
- Input context system for different game modes

### Performance Considerations
- Frame time limiting
- Frame skipping for slow systems
- Performance metrics collection
- Adaptive quality settings

## Testing
- Unit tests for timing accuracy
- Performance profiling
- Input simulation
- State transition validation

## Future Extensions
- Replay system
- Network synchronization
- Save/load functionality
- Debug visualization
