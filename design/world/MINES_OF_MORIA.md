# Mines of Moria - Level Design

## Overview
This document outlines the design for the Mines of Moria level, focusing on the Fellowship's journey from the West-gate to the Bridge of Khazad-dûm.

## Key Locations

### 1. West-gate (Doors of Durin)
- **Description**: Massive stone doors with intricate Dwarven runes
- **Features**:
  - Moonlight reveals the door's inscription
  - Requires the Elvish word for "friend" to open
- **Gameplay**:
  - Puzzle to open the door
  - First major decision point

### 2. First Hall
- **Description**: Grand entrance hall with towering pillars
- **Features**:
  - Crumbling architecture
  - Echoing acoustics
- **Gameplay**:
  - Stealth mechanics introduced
  - First signs of orc activity

### 3. Chamber of Mazarbul (Tomb of Balin)
- **Description**: Ancient dwarven tomb with a large stone table
- **Features**:
  - Book of Mazarbul (lore entry)
  - Evidence of the Dwarves' last stand
- **Gameplay**:
  - Major story beat
  - Combat encounter with orcs
  - First appearance of the Watcher in the Water

### 4. The Bridge of Khazad-dûm
- **Description**: Narrow stone bridge over a fiery chasm
- **Features**:
  - Sheer drops on both sides
  - Glowing embers below
- **Gameplay**:
  - Final escape sequence
  - Balrog boss battle
  - Gandalf's sacrifice

## Map Structure

### Tileset
- **Floor**: Carved stone, cracked stone, chasms
- **Walls**: Dwarven stonework, carved pillars, broken walls
- **Features**:
  - Stairs (intact/collapsed)
  - Bridges (intact/collapsed)
  - Pits/chasms
  - Ancient machinery

### Navigation
- **Main Path**: Linear progression through key locations
- **Side Paths**:
  - Collapsed tunnels (blocked)
  - Treasure rooms (loot)
  - Secret passages (shortcuts/lore)

## Gameplay Mechanics

### 1. Light System
- **Torches**: Limited duration, attract enemies
- **Gandalf's Staff**: Unlimited but reveals the party
- **Darkness**: Reduces visibility, increases fear checks

### 2. Sound System
- **Noise Levels**:
  - Walking (quiet)
  - Running (moderate)
  - Combat (loud)
- **Echoes**: Sounds carry further in large chambers

### 3. Party Management
- **Formation**:
  - Single file (stealthy, slow)
  - Double file (balanced)
  - Loose (fast, noisy)
- **Morale**: Affected by light, rest, and events

## Encounters

### 1. Environmental Hazards
- Collapsing ceilings
- Crumbling walkways
- Ancient traps

### 2. Enemy Types
- Goblins/Orcs (common)
 - Cave troll (mini-boss)
 - The Watcher (scripted event)
 - The Balrog (final boss)

### 3. Non-combat Encounters
- Lost dwarven artifacts
- Ancient murals (lore)
- Gandalf's moments of wisdom

## Data Structures

```rust
// Example Rust structures for Moria

// Tile types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Chasm,
    Stairs,
    Bridge,
    Door,
}

// Room data
pub struct Room {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tiles: Vec<Vec<TileType>>,
    pub connections: Vec<Connection>,
    pub is_illuminated: bool,
    pub has_encounter: bool,
}

// Connection between rooms
pub struct Connection {
    pub target_room: String,
    pub position: (i32, i32),
    pub is_hidden: bool,
}

// Game state for Moria
pub struct MoriaState {
    pub current_room: String,
    pub party_position: (i32, i32),
    pub light_sources: u32,
    pub noise_level: u8,
    pub morale: i8,  // -100 to 100
    pub discovered_rooms: Vec<String>,
}
```

## Implementation Notes
1. **Procedural Generation**:
   - Generate room layouts with hand-crafted key locations
   - Use prefabs for important story rooms
   - Randomize minor paths and dead-ends

2. **Progression**:
   - Linear with optional exploration
   - Locked doors/gates control pacing
   - Environmental storytelling through debris and remnants

3. **Atmosphere**:
   - Dynamic lighting effects
   - Echoing sound design
   - Particle effects for dust and embers
