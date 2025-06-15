# Terminal UI Design (MVP)

## Overview
The MVP will use a classic Dwarf Fortress-style terminal UI, optimized for clarity and quick iteration. The UI is divided into four main areas:
- Map Area
- Status Panel
- Log Panel
- Menu Bar

## Layout
```
+----------------------------------------+
|                  MAP                   |
|                                        |
|                                        |
|                                        |
|                                        |
+------------------+-------------------+
|     STATUS       |      LOG          |
|                  |                   |
|                  |                   |
+------------------+-------------------+
|               MENU BAR                |
+----------------------------------------+
```

## Components

### 1. Map Area
- Displays a scrollable viewport of the world grid
- Shows all entities (hobbits, Ringwraiths) and terrain
- Uses Unicode box-drawing and colored glyphs for clarity
- Player (Frodo) is highlighted

### 2. Status Panel
- Shows current player stats (position, speed, courage, etc.)
- Lists party members and their status (e.g., "Sam: Safe", "Pippin: Fleeing")
- Displays current objective (e.g., "Reach Bree", "Avoid Ringwraiths")

### 3. Log Panel
- Displays recent game events (e.g., "Ringwraith spotted!", "Frodo moves north.")
- System messages (errors, warnings)
- Supports scrolling for longer logs

### 4. Menu Bar
- Shows current mode (e.g., "Exploration", "Paused")
- Displays shortcut hints (e.g., "[Q] Quit [Arrows] Move")
- Context-sensitive help

## Controls (MVP)
- Arrow keys or WASD: Move player
- Q: Quit
- Space: Pause/Unpause
- Tab: Cycle party member (if multi-character control is enabled)
- No mouse support in MVP

## Rendering Technology
- Use `crossterm` for terminal rendering and input
- Double-buffered drawing to minimize flicker
- Unicode and ANSI color support for clarity

## Extensibility
- Panels can be resized or rearranged in future phases
- Support for mouse, tooltips, or advanced menus can be added later

---
This UI provides a familiar, efficient interface for the chase simulation MVP and serves as a foundation for future enhancements.
