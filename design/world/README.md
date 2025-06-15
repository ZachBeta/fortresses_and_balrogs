# World System

## Overview
This document outlines the world simulation system, including terrain, resources, and environmental interactions.

## Core Components

### 1. Terrain System
- **Tile Types**
  - Grassland (buildable, walkable)
  - Water (blocks movement, can be fished)
  - Forest (provides wood, blocks view)
  - Mountain (blocks movement, provides stone)
  - Roads (increases movement speed)

- **Elevation**
  - Affects movement cost
  - Influences water flow
  - Impacts construction options

### 2. Resource System
- **Natural Resources**
  - Wood (from trees)
  - Stone (from rocks/mountains)
  - Food (from plants/animals)
  - Water (from rivers/lakes)

- **Resource Nodes**
  - Finite but respawning nodes
  - Quality levels
  - Seasonal variations

### 3. Structures
- **Types**
  - Residential (houses, shelters)
  - Storage (warehouses, stockpiles)
  - Production (workshops, farms)
  - Infrastructure (roads, bridges)

- **Construction**
  - Material requirements
  - Build time
  - Skill requirements

## Data Structures
```typescript
interface WorldTile {
  x: number;
  y: number;
  elevation: number;
  terrainType: TerrainType;
  resource?: {
    type: ResourceType;
    amount: number;
    quality: number;
  };
  structure?: Structure;
  objects: WorldObject[];
}

interface Structure {
  id: string;
  type: StructureType;
  integrity: number;  // 0-100%
  inventory: Item[];
  assignedTo?: string;  // Character ID
  constructionProgress?: number;  // 0-100%
}
```

## World Generation
1. Heightmap generation
2. Biome placement
3. Resource distribution
4. Initial settlement placement
5. Road/path generation

## Related Documents
- [Character System](../character/README.md)
- [Game Loop](../game_loop/README.md)
