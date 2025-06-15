# Character System

## Overview
This document details the character simulation system, including attributes, needs, decision-making, and interactions.

## Core Components

### 1. Attributes
- **Physical**
  - Strength (STR): Physical power and lifting capacity
  - Dexterity (DEX): Agility and fine motor control
  - Endurance (END): Stamina and health
  - Wits (WIT): Problem-solving and perception

### 2. Needs System (0-100%)
- **Energy**: Depletes with activity, restored by sleeping
- **Hunger**: Increases over time, reduced by eating
- **Social**: Decreases when isolated, improved by socializing
- **Comfort**: Affected by environment and activities

### 3. Decision Making
- **Goal-Oriented Action Planning**
  - Current needs influence priorities
  - Environment provides context
  - Past experiences affect choices

### 4. Skills and Progression
- Skills improve with use
- Learning rate affected by attributes
- Specialization paths available

## Data Structures
```typescript
interface Character {
  id: string;
  name: string;
  attributes: {
    strength: number;    // 1-10
    dexterity: number;   // 1-10
    endurance: number;   // 1-10
    wits: number;        // 1-10
  };
  needs: {
    energy: number;     // 0-100
    hunger: number;      // 0-100
    social: number;      // 0-100
    comfort: number;     // 0-100
  };
  skills: Map<string, number>;  // skillName -> level (0-100)
  currentGoal?: Goal;
  inventory: Item[];
  relationships: Map<string, number>;  // characterId -> relationshipScore
}
```

## Related Documents
- [World State](../world/README.md)
- [Game Loop](../game_loop/README.md)
- [Technical Architecture](../architecture/README.md)
