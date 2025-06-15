# Fortresses & Balrogs MVP - High Level Implementation Summary

## Project Goal
Simulate the flight of Frodo and the hobbits from the Shire to Bree, pursued by Ringwraiths, using a Rust-based ECS architecture. The MVP focuses on pathfinding, pursuit, and evasion, with a classic terminal UI and clear win/loss conditions.

## Core Systems
- **Game Loop:** Fixed timestep, clear separation of input, update, and rendering.
- **ECS Architecture:** Modular components and systems (using `specs`).
- **State Management:** Stack-based, supports gameplay, pause, and future states.
- **World Generation:** Procedural, surface terrain only (no dungeons/mines).
- **Pathfinding:** Basic grid-based A* for all entities.
- **Entity System:** Minimal components (Position, Renderable, Player, Creature, Health), prefab templates for hobbits and Ringwraiths.
- **AI:** Simple pursuit for Ringwraiths, evasion for hobbits.
- **UI:** Dwarf Fortress-style terminal layout (map, status, log, menu), powered by `crossterm`.
- **Gameplay Logic:** Win if all hobbits reach Bree; lose if any are caught.

## Implementation Phases
1. **Documentation:** Complete recursive design docs (done).
2. **Project Bootstrap:** Set up Rust project, dependencies, and initial file structure.
3. **Core Systems:** Implement ECS, game loop, and state management.
4. **World & Pathfinding:** Terrain generation and pathfinding grid.
5. **Entities & AI:** Define components, prefabs, and basic chase/evade logic.
6. **UI:** Terminal rendering and input handling.
7. **Gameplay Logic:** Implement win/loss checks and endgame states.
8. **Testing & Polish:** Playtest, debug, and refine user experience.

## MVP Scope
- No combat, inventory, or underground levels in MVP.
- All four hobbits must reach Bree for victory.
- Ringwraiths pursue using A*; hobbits evade and pathfind to Bree.
- Modular, extensible codebase for future expansion.

## Key Dependencies
- `specs` (ECS), `crossterm` (UI), `serde` (serialization), `rand` (randomness), `tracing` (logging).

## Next Steps
- Bootstrap Rust project and begin implementation following this plan.

---
This summary can be used as a reference for onboarding, implementation, or status reporting throughout MVP development.
