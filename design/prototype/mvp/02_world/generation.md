# World Generation

## Overview
This document outlines the procedural generation system for creating the game world.

## Core Concepts

### World Structure
- **Map**: A 2D grid representing the game world
- **Chunks**: Subdivisions of the map for performance
- **Biomes**: Different environmental regions
- **Features**: Special structures and landmarks

### Generation Pipeline
1. **Heightmap**
   - Generate base terrain height
   - Apply noise functions
   - Create major landforms

2. **Climate**
   - Temperature and moisture distribution
   - Biome assignment
   - River and water table

3. **Features**
   - Place settlements
   - Generate dungeons
   - Add points of interest

## Implementation

### Heightmap Generation
```rust
pub struct Heightmap {
    width: usize,
    height: usize,
    values: Vec<f32>,
    noise: NoiseGenerator,
}

impl Heightmap {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        let noise = NoiseGenerator::new(seed);
        let values = vec![0.0; width * height];
        
        let mut hm = Self {
            width,
            height,
            values,
            noise,
        };
        
        hm.generate();
        hm
    }
    
    fn generate(&mut self) {
        // Generate base noise
        for y in 0..self.height {
            for x in 0..self.width {
                let nx = x as f32 / self.width as f32;
                let ny = y as f32 / self.height as f32;
                
                // Combine multiple octaves of noise
                let mut value = 0.0;
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                
                for _ in 0..OCTAVES {
                    value += self.noise.get(nx * frequency, ny * frequency) * amplitude;
                    amplitude *= PERSISTENCE;
                    frequency *= 2.0;
                }
                
                // Store normalized value
                let idx = y * self.width + x;
                self.values[idx] = (value + 1.0) * 0.5; // Normalize to 0..1
            }
        }
    }
}
```

### Biome Assignment
```rust
pub enum Biome {
    Ocean,
    Beach,
    Grassland,
    Forest,
    Mountain,
    Desert,
    Tundra,
    // ...
}

pub struct BiomeMap {
    width: usize,
    height: usize,
    biomes: Vec<Biome>,
    temperature: Vec<f32>,
    moisture: Vec<f32>,
}

impl BiomeMap {
    pub fn from_heightmap(heightmap: &Heightmap, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let noise = NoiseGenerator::new(seed);
        
        let mut temperature = vec![0.0; heightmap.width * heightmap.height];
        let mut moisture = vec![0.0; heightmap.width * heightmap.height];
        let mut biomes = Vec::with_capacity(heightmap.width * heightmap.height);
        
        // Generate temperature and moisture maps
        for y in 0..heightmap.height {
            for x in 0..heightmap.width {
                let idx = y * heightmap.width + x;
                let height = heightmap.values[idx];
                
                // Temperature based on latitude and height
                let lat = y as f32 / heightmap.height as f32;
                temperature[idx] = (1.0 - lat) * (1.0 - height * 0.5);
                
                // Moisture from noise
                let nx = x as f32 / heightmap.width as f32;
                let ny = y as f32 / heightmap.height as f32;
                moisture[idx] = noise.get(nx * 2.0, ny * 2.0) * 0.5 + 0.5;
            }
        }
        
        // Assign biomes
        for i in 0..temperature.len() {
            let t = temperature[i];
            let m = moisture[i];
            let h = heightmap.values[i];
            
            let biome = if h < 0.1 {
                Biome::Ocean
            } else if h < 0.15 {
                Biome::Beach
            } else if t < 0.2 {
                if m < 0.5 { Biome::Tundra } else { Biome::Taiga }
            } else if t > 0.8 {
                if m < 0.3 { Biome::Desert } else { Biome::Savanna }
            } else {
                if m < 0.3 { Biome::Grassland }
                else if m < 0.7 { Biome::Forest }
                else { Biome::Rainforest }
            };
            
            biomes.push(biome);
        }
        
        Self {
            width: heightmap.width,
            height: heightmap.height,
            biomes,
            temperature,
            moisture,
        }
    }
}
```

### Feature Placement
```rust
pub struct WorldGenerator {
    seed: u64,
    rng: StdRng,
    heightmap: Heightmap,
    biome_map: BiomeMap,
    features: Vec<Feature>,
}

impl WorldGenerator {
    pub fn new(width: usize, height: usize, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| SystemTime::now().elapsed()
            .unwrap_or_default()
            .as_secs());
            
        let mut rng = StdRng::seed_from_u64(seed);
        let heightmap = Heightmap::new(width, height, seed);
        let biome_map = BiomeMap::from_heightmap(&heightmap, seed);
        
        Self {
            seed,
            rng,
            heightmap,
            biome_map,
            features: Vec::new(),
        }
    }
    
    pub fn generate_features(&mut self, count: usize) {
        for _ in 0..count {
            // Find suitable location
            let x = self.rng.gen_range(0..self.heightmap.width);
            let y = self.rng.gen_range(0..self.heightmap.height);
            
            // Check if location is suitable
            let idx = y * self.heightmap.width + x;
            let height = self.heightmap.values[idx];
            
            if height < 0.1 || height > 0.8 {
                continue; // Skip water and high mountains
            }
            
            // Create feature based on biome
            let biome = &self.biome_map.biomes[idx];
            let feature = match biome {
                Biome::Forest => self.generate_forest(x, y),
                Biome::Mountain => self.generate_cave(x, y),
                Biome::Grassland => self.generate_settlement(x, y),
                _ => continue,
            };
            
            self.features.push(feature);
        }
    }
    
    fn generate_forest(&mut self, x: usize, y: usize) -> Feature {
        let size = self.rng.gen_range(5..20);
        let density = self.rng.gen_range(0.3..0.8);
        
        Feature::new(FeatureType::Forest, x, y, size, density)
    }
    
    // ... other feature generation methods
}
```

## Performance Considerations

### Memory Usage
- Use chunking for large worlds
- Implement level of detail (LOD)
- Use appropriate data structures

### Generation Speed
- Parallelize independent operations
- Profile and optimize hot paths
- Consider pre-generating common patterns

## Testing

### Unit Tests
- Test individual noise functions
- Verify biome assignments
- Check feature placement rules

### Integration Tests
- Verify world consistency
- Test edge cases
- Validate performance

## Future Extensions
- Dynamic world generation
- World editing tools
- Save/load functionality
- Network synchronization

## Best Practices
- Use deterministic random number generation
- Document generation parameters
- Provide seed-based reproducibility
- Implement validation checks
