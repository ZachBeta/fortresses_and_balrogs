# World Generation Scope

## 1. Core Requirements

### 1.1 Map Generation
- **Procedural Elements**:
  - Room shapes and sizes
  - Corridor connections
  - Environmental features (pits, bridges, etc.)
- **Handcrafted Elements**:
  - Key story locations (Doors of Durin, Chamber of Mazarbul, Bridge of Khazad-dûm)
  - Critical path through Moria
  - Scripted event locations

### 1.2 Level of Detail
- **MVP (Minimal Viable Product)**:
  - Simple rectangular rooms
  - Straight corridors
  - Basic terrain types (floor, wall, chasm)
  - Key items and NPCs placed manually

- **Future Expansion**:
  - Complex room shapes
  - Multi-level structures
  - Dynamic lighting and shadow casting
  - Weather/atmospheric effects

## 2. Technical Implementation

### 2.1 Data Structures
```rust
// MVP Implementation
pub struct Tile {
    pub tile_type: TileType,
    pub is_explored: bool,
    pub is_visible: bool,
    pub light_level: u8,
}

pub enum TileType {
    Floor,
    Wall,
    Chasm,
    Stairs,
    Door(DoorState),
}

pub enum DoorState {
    Open,
    Closed,
    Locked,
    Broken,
}

pub struct Room {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub room_type: RoomType,
}

pub enum RoomType {
    Standard,
    Entrance,
    Boss,
    Treasure,
    Puzzle,
}
```

### 2.2 Generation Algorithm (MVP)
1. Place key story rooms manually
2. Generate connecting corridors
3. Fill remaining space with generic rooms
4. Add environmental features
5. Place enemies and items

## 3. Content Scope

### 3.1 Included in MVP
- **Rooms**: 10-15 handcrafted rooms
- **Corridors**: Simple straight connections
- **Lighting**: Basic light/dark areas
- **Entities**:
  - Fellowship members
  - Basic orc enemies
  - The Balrog (simplified)

### 3.2 Post-MVP Features
- **Procedural Dungeon Generation**
- **Complex AI Behaviors**
- **Advanced Lighting System**
- **Environmental Hazards**
- **Dynamic Events**

## 4. Performance Considerations
- **View Distance**: 10-15 tiles (adjustable)
- **Entity Count**: Max 20 active NPCs
- **Save File Size**: Target < 1MB for game state

## 5. Future Expansion
- **New Areas**:
  - Dwarrowdelf
  - The Endless Stair
  - The Underdeeps
- **Procedural Quests**
- **Dynamic World State**

## 6. Technical Dependencies
- **Rust Crates**:
  - `bracket-lib` for roguelike features
  - `serde` for save/load
  - `specs` or `legion` for ECS (if needed)

## 7. Scope Management

### 7.1 MVP Boundaries
- Fixed map size (e.g., 100x100 tiles)
- Limited enemy variety
- Basic AI (patrol, chase, attack)
- Simple inventory system

### 7.2 Cut Features (for now)
- Day/night cycle
- Weather system
- Complex physics
- Advanced sound propagation

## 8. Development Phases

1. **Prototype**
   - Basic map rendering
   - Player movement
   - Simple collision

2. **MVP**
   - Core gameplay loop
   - Basic combat
   - Simple AI
   - Win/lose conditions

3. **Polish**
   - UI improvements
   - Balance tuning
   - Bug fixes

4. **Expansion**
   - Additional content
   - Advanced features
   - Mod support
