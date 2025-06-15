# Entity Prefabs (MVP: Shire to Bree Chase)

## Overview
This document defines the prefab templates for key entities in the chase scenario, based on FELLOWSHIP_STATS.md but simplified for our simulation needs. Each prefab lists initial components, starting stats, and relevant gameplay traits.

## Hobbits (Player Characters)

### Frodo Baggins
- **Position**: Starting at SHIRE_COORDS
- **Renderable**: `@` (white fg, green bg)
- **Player**: Yes
- **Stats**:
  - Speed: 5
  - Stealth: 15
  - Perception: 12
  - Courage: 14

### Samwise Gamgee
- **Position**: Adjacent to Frodo
- **Renderable**: `@` (yellow fg, green bg)
- **Player**: Yes (or AI-controlled companion)
- **Stats**:
  - Speed: 5
  - Stealth: 13
  - Perception: 11
  - Courage: 16

### Merry Brandybuck
- **Position**: Adjacent to Frodo
- **Renderable**: `@` (cyan fg, green bg)
- **Player**: No (AI-controlled)
- **Stats**:
  - Speed: 5
  - Stealth: 14
  - Perception: 12
  - Courage: 13

### Pippin Took
- **Position**: Adjacent to Frodo
- **Renderable**: `@` (magenta fg, green bg)
- **Player**: No (AI-controlled)
- **Stats**:
  - Speed: 5
  - Stealth: 12
  - Perception: 10
  - Courage: 12

## Ringwraiths (Nazgûl)

### Ringwraith
- **Position**: Spawn at strategic map edges
- **Renderable**: `N` (black fg, gray bg)
- **Creature**: Yes
- **Ringwraith**: Yes (special marker component)
- **Stats**:
  - Speed: 6 (faster than hobbits)
  - Stealth: 10
  - Perception: 18 (very high, can sense hobbits at distance)
  - Fear Aura: Reduces nearby hobbits' courage

## Notes
- All stats are on a 1-20 scale, as in FELLOWSHIP_STATS.md.
- Courage may be used for future fear/stealth mechanics.
- Additional fields (inventory, health, etc.) can be added in later phases.
- Prefabs can be extended for new scenarios (e.g., Aragorn, Weathertop, etc.).

---
This prefab set supports the chase scenario: hobbits escaping from the Shire to Bree, Ringwraiths in pursuit, with basic stats for pathfinding, evasion, and pursuit.
