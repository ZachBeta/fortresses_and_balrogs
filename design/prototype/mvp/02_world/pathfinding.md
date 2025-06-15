# Pathfinding System (MVP)

## Overview
This document describes the basic grid-based A* pathfinding system for the MVP phase. The goal is to enable entities to find the shortest walkable path on a 2D tile grid, without support for dynamic obstacles, multiple movement types, or hierarchical navigation.

## Problem Statement
- Find the shortest path between two points on a grid, avoiding impassable tiles.
- The grid is generated from the terrain system and uses a boolean walkable flag per tile.

## Data Structures

### Pathfinding Grid
- 2D array or flat vector of bool (walkable/unwalkable)
- Same dimensions as the world terrain grid

### Position
- (x, y) coordinates (usize)

### Path
- Vec<(usize, usize)> representing the sequence of tiles from start to goal (inclusive)

## Algorithm

### A* Search (Pseudocode)
```rust
use std::collections::{BinaryHeap, HashMap};

fn astar(
    grid: &Vec<Vec<bool>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.insert(start, 0);
    f_score.insert(start, manhattan(start, goal));
    open_set.push(Node { pos: start, f_score: f_score[&start] });

    while let Some(current) = open_set.pop() {
        if current.pos == goal {
            return Some(reconstruct_path(came_from, current.pos));
        }
        for neighbor in neighbors(grid, current.pos) {
            let tentative_g = g_score[&current.pos] + 1;
            if tentative_g < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current.pos);
                g_score.insert(neighbor, tentative_g);
                let f = tentative_g + manhattan(neighbor, goal);
                f_score.insert(neighbor, f);
                open_set.push(Node { pos: neighbor, f_score: f });
            }
        }
    }
    None
}
```
- `neighbors` returns orthogonal and diagonal adjacent walkable tiles.
- `manhattan` is the Manhattan distance heuristic.

## Integration
- System exposes a function to request a path from (x1, y1) to (x2, y2).
- Entities store their current path as a Vec<(usize, usize)>.
- Movement system advances entities along their path each tick.

## Limitations (MVP)
- No dynamic obstacle avoidance (static grid only)
- No support for multiple movement types
- No path smoothing or hierarchical search

## Future Extensions
- Dynamic obstacle updates
- Hierarchical pathfinding for large worlds
- Support for flying/swimming/climbing
- Path smoothing

## Testing
- Unit test: Ensure shortest path is found in simple maps
- Edge cases: No path, start == goal, blocked goal

---
This basic system provides reliable, fast pathfinding for the MVP, and can be extended in later phases.
