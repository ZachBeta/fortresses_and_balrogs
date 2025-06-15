# Rust Prototype: Technical Design

## Overview
This document outlines the technical design for the Rust-based prototype with a Dwarf Fortress-style UI.

## Architecture

### Core Dependencies
- `crossterm`: Terminal UI and input handling
- `specs` or `legion`: ECS (Entity-Component-System)
- `serde`: Serialization/deserialization
- `rand`: Random number generation
- `tracing`: Structured logging

### Project Structure
```
src/
├── main.rs            # Application entry point
├── lib.rs             # Library root
├── ui/                # UI components
│   ├── mod.rs
│   ├── panels/        # Different UI panels
│   └── widgets/       # Reusable UI components
├── ecs/               # ECS systems and components
│   ├── mod.rs
│   ├── components/    # ECS components
│   └── systems/       # ECS systems
├── world/             # World generation and management
│   ├── mod.rs
│   └── gen/           # World generation code
└── util/              # Utility functions
    └── mod.rs
```

## UI Design

### Screen Layout
```
+----------------------------------------+
|                  MAP                   |
|                                        |
|                                        |
|                                        |
|                                        |
+------------------+-------------------+
|     STATUS       |      LOG          |
|                  |                   |
|                  |                   |
+------------------+-------------------+
|               MENU BAR                |
+----------------------------------------+
```

### UI Components
1. **Map Panel**
   - Main game view
   - Rendered with Unicode box-drawing characters
   - Scrollable viewport

2. **Status Panel**
   - Character stats
   - Current time/date
   - Important notifications

3. **Log Panel**
   - Game events
   - System messages
   - Combat logs

4. **Menu Bar**
   - Current mode indicator
   - Shortcut hints
   - Context-sensitive help

## ECS Design

### Core Components
```rust
// Basic components
struct Position { x: i32, y: i32 }
struct Renderable { glyph: char, fg: Color, bg: Color }
struct Name(String);

// Game-specific components
struct Player;
struct BlocksMovement;
struct BlocksSight;
struct Viewshed { visible_tiles: Vec<Point>, range: i32, dirty: bool }
```

### Systems
1. **Input System**
   - Handles player input
   - Manages UI state
   - Processes commands

2. **Visibility System**
   - Updates field of view
   - Handhes fog of war
   - Manages exploration

3. **Rendering System**
   - Draws the game world
   - Updates UI elements
   - Handles screen updates

## Development Phases

### Phase 1: Basic Setup
- [ ] Initialize Rust project
- [ ] Set up basic window with crossterm
- [ ] Implement main game loop
- [ ] Create basic UI layout

### Phase 2: Core Gameplay
- [ ] Implement world generation
- [ ] Add player character
- [ ] Implement movement
- [ ] Add basic interactions

### Phase 3: Polish
- [ ] Add UI animations
- [ ] Implement save/load
- [ ] Add sound effects
- [ ] Performance optimization

## Known Challenges
1. **Terminal Performance**
   - Minimize redraws
   - Optimize rendering
   - Handle window resizing

2. **UI Complexity**
   - Keep UI code modular
   - Support multiple screen sizes
   - Handle input focus

3. **Game State Management**
   - Clean separation between UI and game state
   - Efficient serialization
   - Handle loading/saving

## Next Steps
1. Set up the Rust project with initial dependencies
2. Create a basic window with crossterm
3. Implement the main game loop
4. Start with a simple map renderer
