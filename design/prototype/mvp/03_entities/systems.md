# Entity Systems (MVP: Shire to Bree Chase)

## Overview
This document describes the core entity systems needed for the MVP chase simulation: movement, rendering, and basic AI for pursuit/evasion.

## 1. Movement System
Moves entities along their path (computed by pathfinding) each tick.

```rust
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Path>,
        Entities<'a>,
    );

    fn run(&mut self, (mut positions, mut paths, entities): Self::SystemData) {
        for (pos, path, _entity) in (&mut positions, &mut paths, &entities).join() {
            if let Some(next) = path.next_step() {
                pos.x = next.0;
                pos.y = next.1;
                path.advance();
            }
        }
    }
}
```

## 2. Rendering System
Renders all entities with a `Renderable` component to the terminal UI.

```rust
pub struct RenderingSystem<'a> {
    pub ui: &'a mut TerminalUI,
}

impl<'a> System<'a> for RenderingSystem<'_> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        Entities<'a>,
    );

    fn run(&mut self, (positions, renderables, entities): Self::SystemData) {
        self.ui.clear();
        for (pos, rend, _entity) in (&positions, &renderables, &entities).join() {
            self.ui.draw_tile(pos.x, pos.y, rend.glyph, rend.fg, rend.bg);
        }
        self.ui.flush();
    }
}
```

## 3. Basic AI System
Implements simple AI:
- **Ringwraiths**: Pursue the nearest hobbit (Frodo or companions) using A* pathfinding.
- **Hobbits**: Move toward Bree, avoid Ringwraiths if nearby (flee if within threat radius).

```rust
pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Path>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Creature>,
        ReadStorage<'a, Ringwraith>,
        ReadStorage<'a, Hobbit>,
        Read<'a, NavGrid>, // Pathfinding grid
        Entities<'a>,
    );

    fn run(&mut self, (mut positions, mut paths, players, creatures, ringwraiths, hobbits, nav, entities): Self::SystemData) {
        // Ringwraiths chase nearest hobbit
        for (wraith_pos, _wraith, entity) in (&positions, &ringwraiths, &entities).join() {
            if let Some((closest_hobbit, dist)) = find_closest_hobbit(wraith_pos, &positions, &hobbits) {
                if let Some(path) = nav.astar((wraith_pos.x, wraith_pos.y), (closest_hobbit.x, closest_hobbit.y)) {
                    if let Some(mut path_comp) = paths.get_mut(entity) {
                        path_comp.set_path(path);
                    }
                }
            }
        }
        // Hobbits move toward Bree; flee if Ringwraith is near
        for (hobbit_pos, _hobbit, entity) in (&positions, &hobbits, &entities).join() {
            let threat = find_nearest_ringwraith(hobbit_pos, &positions, &ringwraiths);
            let target = if let Some((wraith_pos, dist)) = threat {
                if dist < 5 {
                    // Flee away from Ringwraith
                    let dx = hobbit_pos.x as isize - wraith_pos.x as isize;
                    let dy = hobbit_pos.y as isize - wraith_pos.y as isize;
                    (hobbit_pos.x.wrapping_add(dx.signum() as usize), hobbit_pos.y.wrapping_add(dy.signum() as usize))
                } else {
                    // Head toward Bree
                    BREE_COORDS
                }
            } else {
                BREE_COORDS
            };
            if let Some(path) = nav.astar((hobbit_pos.x, hobbit_pos.y), target) {
                if let Some(mut path_comp) = paths.get_mut(entity) {
                    path_comp.set_path(path);
                }
            }
        }
    }
}
```

## 4. System Scheduling
- Order: AI → Pathfinding → Movement → Rendering
- All systems run each tick in the main game loop

## 5. Future Extensions
- Add combat (at Weathertop or later)
- Add stealth/hiding mechanics
- Add fatigue, inventory, or other entity systems

---
This set of systems supports the chase scenario for the MVP: hobbits racing to Bree, Ringwraiths in pursuit, with basic evasion and pursuit AI.
