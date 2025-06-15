# Key Entities and Interactions

## 1. Character Entities

### 1.1 Fellowship Members
- **Aragorn**
  - **Type**: Player Character
  - **Interactions**:
    - Can lead the party (morale bonus)
    - Can use healing herbs
    - Can track enemies (Survival skill)

- **Gandalf**
  - **Type**: Support Character
  - **Interactions**:
    - Can light dark areas
    - Can cast spells
    - Can read ancient texts

- **Legolas**
  - **Type**: Ranged Combatant
  - **Interactions**:
    - Can spot distant enemies
    - Can shoot arrows with precision
    - Can move silently

- **Gimli**
  - **Type**: Melee Combatant
  - **Interactions**:
    - Can break weak walls
    - Can carry heavy objects
    - Resistant to fear effects

- **Frodo**
  - **Type**: Stealth Character
  - **Interactions**:
    - Can wear the One Ring (with consequences)
    - Can hide in shadows
    - Can sense Ringwraiths

### 1.2 NPCs
- **Orcs/Goblins**
  - **Behavior**: Patrol, attack on sight
  - **Interactions**: Combat, can be avoided with stealth

- **The Balrog**
  - **Behavior**: Boss enemy, pursues party
  - **Interactions**: Combat, environmental hazards

## 2. World Entities

### 2.1 Interactive Objects
- **Doors**
  - **States**: Locked, Unlocked, Broken
  - **Interactions**: Open, Close, Lockpick, Break

- **Containers**
  - **Types**: Chests, Crates, Barrels
  - **Interactions**: Search, Take, Leave

- **Light Sources**
  - **Types**: Torches, Gandalf's Staff, Ambient
  - **Interactions**: Light, Extinguish, Carry

### 2.2 Environmental Elements
- **Chasms**
  - **Interactions**: Jump (with risk), Find alternative path

- **Collapsed Tunnels**
  - **Interactions**: Clear (STR check), Find detour

- **Ancient Mechanisms**
  - **Interactions**: Activate, Disable, Study

## 3. Item System

### 3.1 Equipment
- **Weapons**
  - **Types**: Swords, Bows, Staffs
  - **Interactions**: Equip, Attack, Throw

- **Armor**
  - **Types**: Light, Medium, Heavy
  - **Interactions**: Wear, Remove, Repair

### 3.2 Consumables
- **Food/Water**
  - **Effects**: Restore HP, Remove hunger

- **Potions**
  - **Types**: Healing, Light, Invisibility

## 4. Interaction System

### 4.1 Basic Interactions
```rust
enum InteractionType {
    Examine,
    Use,
    Take,
    Talk,
    Attack,
    Special,
}

struct Interaction {
    source: Entity,
    target: Entity,
    interaction_type: InteractionType,
    success_chance: f32,
}
```

### 4.2 Context-Sensitive Actions
- **Combat**: Attack, Defend, Use Item
- **Stealth**: Hide, Sneak, Distract
- **Exploration**: Search, Listen, Track

## 5. Event System

### 5.1 Event Types
- **Combat Events**: Attack, Damage, Defeat
- **Dialog Events**: Conversation choices, Reputation changes
- **World Events**: Time passing, Weather changes
- **Story Events**: Cutscenes, Plot advancements

### 5.2 Event Handlers
```rust
trait EventHandler {
    fn handle_event(&mut self, event: &GameEvent) -> Vec<GameEvent>;
}

struct CombatHandler;
impl EventHandler for CombatHandler {
    fn handle_event(&mut self, event: &GameEvent) -> Vec<GameEvent> {
        // Handle combat events
        vec![]
    }
}
```

## 6. State Management

### 6.1 Game States
- **Main Menu**
- **Exploration**
- **Combat**
- **Dialog**
- **Inventory**
- **Pause Menu**

### 6.2 Save States
- **Auto-save**: Triggered by key events
- **Manual Save**: Player-initiated
- **Quick Save**: Single slot for quick saving

## Implementation Notes
1. **ECS Architecture**: Consider using an Entity-Component-System pattern
2. **Event Bus**: For decoupled communication between systems
3. **State Stack**: For managing game states (menu, gameplay, etc.)
4. **Serialization**: For save/load functionality
