# Terrain System

## Overview
This document defines the terrain system that handles the physical properties and rendering of the game world.

## Core Components

### Terrain Types
```rust
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TerrainType {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Forest,
    Mountain,
    Snow,
    // ... other terrain types
}

impl TerrainType {
    pub fn get_properties(&self) -> TerrainProperties {
        match self {
            Self::DeepWater => TerrainProperties {
                walkable: false,
                swim_only: true,
                move_cost: 2.0,
                height: 0.0,
            },
            Self::ShallowWater => TerrainProperties {
                walkable: true,
                swim_only: false,
                move_cost: 1.5,
                height: 0.2,
            },
            // ... other terrain properties
        }
    }
}
```

### Terrain Properties
```rust
#[derive(Clone, Copy, Debug)]
pub struct TerrainProperties {
    /// Can entities walk on this terrain?
    pub walkable: bool,
    /// Is this terrain only traversable by swimming?
    pub swim_only: bool,
    /// Movement cost multiplier
    pub move_cost: f32,
    /// Height value (0.0 - 1.0)
    pub height: f32,
}
```

## Implementation

### Terrain Chunk
```rust
pub struct TerrainChunk {
    pub position: (i32, i32),  // Chunk coordinates
    pub size: usize,           // Width/height in tiles
    pub tiles: Vec<TerrainTile>,
    pub dirty: bool,          // Needs redraw
}

impl TerrainChunk {
    pub fn new(position: (i32, i32), size: usize) -> Self {
        Self {
            position,
            size,
            tiles: vec![TerrainTile::default(); size * size],
            dirty: true,
        }
    }
    
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&TerrainTile> {
        if x < self.size && y < self.size {
            self.tiles.get(y * self.size + x)
        } else {
            None
        }
    }
    
    pub fn set_tile(&mut self, x: usize, y: usize, tile: TerrainTile) -> bool {
        if x < self.size && y < self.size {
            let idx = y * self.size + x;
            if self.tiles[idx] != tile {
                self.tiles[idx] = tile;
                self.dirty = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
```

### Terrain Tile
```rust
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct TerrainTile {
    pub terrain_type: TerrainType,
    pub elevation: f32,        // 0.0 - 1.0
    pub moisture: f32,         // 0.0 - 1.0
    pub temperature: f32,      // 0.0 - 1.0
    pub has_water: bool,
    pub has_road: bool,
    // ... other tile properties
}

impl TerrainTile {
    pub fn new(terrain_type: TerrainType) -> Self {
        let props = terrain_type.get_properties();
        
        Self {
            terrain_type,
            elevation: props.height,
            moisture: 0.0,
            temperature: 0.5,
            has_water: !props.walkable || props.swim_only,
            has_road: false,
        }
    }
    
    pub fn is_walkable(&self) -> bool {
        let props = self.terrain_type.get_properties();
        props.walkable && !self.has_water
    }
    
    pub fn get_movement_cost(&self) -> f32 {
        let mut cost = self.terrain_type.get_properties().move_cost;
        
        // Modify cost based on tile state
        if self.has_water {
            cost *= 1.5;
        }
        
        if self.has_road {
            cost *= 0.5;
        }
        
        cost
    }
}
```

### Terrain Manager
```rust
pub struct TerrainManager {
    chunks: HashMap<(i32, i32), TerrainChunk>,
    chunk_size: usize,
    loaded_chunks: HashSet<(i32, i32)>,
    seed: u64,
}

impl TerrainManager {
    pub fn new(chunk_size: usize, seed: u64) -> Self {
        Self {
            chunks: HashMap::new(),
            chunk_size,
            loaded_chunks: HashSet::new(),
            seed,
        }
    }
    
    pub fn get_chunk(&mut self, chunk_x: i32, chunk_y: i32) -> &mut TerrainChunk {
        let chunk_key = (chunk_x, chunk_y);
        
        if !self.chunks.contains_key(&chunk_key) {
            // Generate or load chunk
            let mut chunk = TerrainChunk::new(chunk_key, self.chunk_size);
            self.generate_chunk(&mut chunk);
            self.chunks.insert(chunk_key, chunk);
        }
        
        self.loaded_chunks.insert(chunk_key);
        self.chunks.get_mut(&chunk_key).unwrap()
    }
    
    fn generate_chunk(&self, chunk: &mut TerrainChunk) {
        let (cx, cy) = chunk.position;
        let size = chunk.size as i32;
        let mut rng = StdRng::seed_from_u64(
            self.seed.wrapping_add((cx as u64) << 32).wrapping_add(cy as u64)
        );
        
        // Generate noise-based terrain
        for y in 0..size {
            for x in 0..size {
                let wx = (cx * size + x) as f32 / 100.0;
                let wy = (cy * size + y) as f32 / 100.0;
                
                // Sample noise functions
                let elevation = self.sample_elevation(wx, wy);
                let moisture = self.sample_moisture(wx, wy);
                let temperature = self.sample_temperature(wx, wy);
                
                // Determine terrain type
                let terrain_type = self.determine_terrain_type(elevation, moisture, temperature);
                
                // Create and set tile
                let tile = TerrainTile {
                    terrain_type,
                    elevation,
                    moisture,
                    temperature,
                    has_water: elevation < 0.3 || rng.gen_bool(0.1) as bool,
                    has_road: false,
                };
                
                chunk.set_tile(x as usize, y as usize, tile);
            }
        }
    }
    
    // ... noise sampling and helper methods
}
```

## Rendering

### Terrain Renderer
```rust
pub struct TerrainRenderer {
    shader: ShaderProgram,
    vertex_array: VertexArray,
    texture_atlas: TextureAtlas,
}

impl TerrainRenderer {
    pub fn new() -> Result<Self> {
        // Initialize OpenGL resources
        let shader = ShaderProgram::from_files(
            "shaders/terrain.vert",
            "shaders/terrain.frag"
        )?;
        
        let vertex_array = VertexArray::new()?;
        let texture_atlas = TextureAtlas::new("textures/terrain.png", 16, 16)?;
        
        Ok(Self {
            shader,
            vertex_array,
            texture_atlas,
        })
    }
    
    pub fn render_chunk(
        &mut self,
        chunk: &TerrainChunk,
        camera: &Camera,
        lights: &[Light],
    ) -> Result<()> {
        if !chunk.dirty {
            return Ok(());
        }
        
        // Update vertex buffer if needed
        self.update_chunk_geometry(chunk)?;
        
        // Set up shader uniforms
        self.shader.bind();
        self.shader.set_uniform_mat4("u_view_proj", &camera.view_proj_matrix());
        
        // Set up lighting
        self.shader.set_uniform_int("u_light_count", lights.len() as i32);
        for (i, light) in lights.iter().enumerate() {
            self.shader.set_uniform_vec3(
                &format!("u_lights[{}].position", i),
                &light.position
            );
            self.shader.set_uniform_vec3(
                &format!("u_lights[{}].color", i),
                &light.color
            );
        }
        
        // Bind textures
        self.texture_atlas.bind(0);
        
        // Draw
        self.vertex_array.bind();
        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                (chunk.size * chunk.size * 6) as i32
            );
        }
        
        chunk.dirty = false;
        Ok(())
    }
    
    fn update_chunk_geometry(&mut self, chunk: &TerrainChunk) -> Result<()> {
        // Update vertex buffer with chunk geometry
        // This would include position, normal, texture coordinates, etc.
        // ...
        
        Ok(())
    }
}
```

## Performance Optimization

### Level of Detail (LOD)
```rust
pub enum TerrainLOD {
    High,    // Full detail
    Medium,  // Reduced geometry
    Low,     // Minimal geometry
}

impl TerrainLOD {
    pub fn get_render_distance(&self) -> f32 {
        match self {
            Self::High => 100.0,
            Self::Medium => 300.0,
            Self::Low => 1000.0,
        }
    }
    
    pub fn get_tessellation_level(&self) -> usize {
        match self {
            Self::High => 1,
            Self::Medium => 2,
            Self::Low => 4,
        }
    }
}
```

### Chunk Management
- Dynamic loading/unloading based on camera position
- Frustum culling
- Occlusion culling
- Background loading

## Physics Integration

### Collision Detection
- Terrain height sampling
- Slope-based movement restrictions
- Water physics
- Collision response

### Pathfinding
- Navigation mesh generation
- A* pathfinding with terrain costs
- Dynamic obstacle avoidance
- Path smoothing

## Editor Support
- Terrain sculpting tools
- Texture painting
- Object placement
- Terrain material editing

## Serialization
- Save/load terrain data
- Delta compression for network sync
- Versioning support

## Testing
- Unit tests for terrain generation
- Performance profiling
- Visual validation tools
- Automated terrain validation

## Future Extensions
- Procedural texturing
- Deformable terrain
- Weather effects
- Erosion simulation
