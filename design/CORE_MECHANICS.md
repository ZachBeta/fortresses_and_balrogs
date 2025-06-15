# Core Game Mechanics

## 1. Turn-Based Movement
- **Action Points (AP)**: Each character has a pool of AP per turn
- **Movement Costs**:
  - Walk: 1 AP per tile
  - Run: 2 AP per tile (noisy)
  - Cautious Move: 2 AP per tile (stealthy)
- **Facing**: Direction affects awareness and combat

## 2. Party Management
- **Formation System**:
  - Single File (stealthy, +2 to stealth checks)
  - Double File (balanced, no bonuses)
  - Loose (fast, -2 to stealth, +10% movement speed)
- **Morale System**:
  - Affected by light levels, rest, and events
  - Impacts skill checks and combat effectiveness

## 3. Light and Vision
- **Light Sources**:
  - Gandalf's Staff (20ft radius, unlimited)
  - Torches (30ft radius, 1 hour duration)
  - Ambient Light (5ft radius in some areas)
- **Vision Types**:
  - Normal Vision (full details)
  - Low Light (disadvantage on perception)
  - Darkness (heavily obscured, disadvantage on attacks)

## 4. Combat System
- **Initiative**: DEX-based turn order
- **Action Economy**:
  - Move Action (up to half speed)
  - Standard Action (attack, cast, use item)
  - Bonus Action (quick actions)
  - Reaction (opportunity attacks, readied actions)
- **Combat Modifiers**:
  - Advantage/Disadvantage system
  - Flanking bonuses
  - Cover mechanics

## 5. Skill System
### Core Skills:
- **Stealth** (DEX): Moving silently, hiding
- **Perception** (WIS): Spotting hidden objects/enemies
- **Lore** (INT): Knowledge checks
- **Persuasion** (CHA): Social interactions
- **Athletics** (STR): Climbing, jumping, swimming

### Skill Checks:
```rust
fn skill_check(character: &Character, skill: Skill, difficulty: i32) -> bool {
    let modifier = match skill {
        Skill::Stealth => character.dexterity,
        Skill::Perception => character.wisdom,
        // ... other skills
    };
    let roll = d20() + modifier;
    roll >= difficulty
}
```

## 6. Inventory System
- **Encumbrance**: Affects movement speed
- **Quick Items**: 4 slots for easy access
- **Container System**: Bags, chests, etc.
- **Item Interaction**:
  - Use
  - Examine
  - Drop
  - Combine

## 7. Dialogue System
- **Branching Conversations**
- **Persuasion Checks**
- **Reputation System**
- **Information Gathering**

## 8. Environmental Interaction
- **Searching**: Find hidden objects/paths
- **Climbing**: Scale surfaces (Athletics check)
- **Lockpicking**: Open locked doors/chests
- **Puzzles**: Environmental challenges

## 9. Time System
- **Turns**: 6 seconds each
- **Rests**:
  - Short Rest (1 hour)
  - Long Rest (8 hours)
- **Light Source Duration**
- **Event Triggers**

## 10. Save System
- **Save Slots**: Multiple save files
- **Auto-save**: Key story moments
- **Quick Save/Load**

## Implementation Notes
1. **ECS Architecture**: Consider using an Entity-Component-System architecture
2. **Event Bus**: For handling game events
3. **State Machine**: For game/character states
4. **Serialization**: For save games
