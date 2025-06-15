# Game Loop System

## Overview
This document describes the core game loop and simulation timing that drives the game's progression.

## Core Loop

### 1. Main Loop Phases
1. **Input Phase**
   - Process player commands
   - Queue actions

2. **Simulation Phase**
   - Update character AI and needs
   - Process world events
   - Handle interactions
   - Update world state

3. **Render Phase**
   - Update visual representation
   - Handle animations
   - Update UI

### 2. Time System
- **Game Time**
  - 1 real second = 1 game minute (configurable)
  - 60 minutes per hour
  - 24 hours per day
  - 7-day week cycle
  - 4 seasons

- **Simulation Ticks**
  - 1 tick = 1 game second
  - 60 ticks per game minute
  - Some systems update every tick, others less frequently

### 3. Event System
- **Event Queue**
  - Time-delayed events
  - Conditional triggers
  - Recurring events

- **Event Types**
  - Character actions
  - World events (weather, disasters)
  - Story events
  - System events

## Data Structures
```typescript
interface GameState {
  time: {
    year: number;
    season: Season;
    day: number;
    hour: number;
    minute: number;
    tick: number;
  };
  characters: Character[];
  world: World;
  eventQueue: GameEvent[];
  player: {
    controlledCharacter?: string;  // Character ID
    camera: {
      x: number;
      y: number;
      zoom: number;
    };
  };
}

interface GameEvent {
  id: string;
  type: string;
  scheduledTime: number;  // game time in ticks
  data: any;
  repeatInterval?: number;  // in ticks, if repeating
  callback: (gameState: GameState) => void;
}
```

## Performance Considerations
- **Time Scaling**
  - Variable time speed
  - Pause functionality
  - Fast-forward options

- **Optimization**
  - Spatial partitioning
  - Level of detail (LOD)
  - Background processing

## Related Documents
- [Character System](../character/README.md)
- [World System](../world/README.md)
