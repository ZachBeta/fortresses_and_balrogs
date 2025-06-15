# Gameplay Logic: Win/Loss Conditions (MVP)

## Overview
This document defines the win and loss conditions for the chase scenario MVP, matching the story goal: all four hobbits must reach Bree safely.

## Win Condition
- **Victory:** The player wins if all four hobbits (Frodo, Sam, Merry, Pippin) reach Bree (the designated map location) without being caught by a Ringwraith.
- The game ends immediately in victory when the last hobbit enters Bree.
- Display a victory message/log entry (e.g., "All hobbits have reached Bree! You win.")

## Loss Condition
- **Defeat:** The player loses if any hobbit is caught by a Ringwraith before reaching Bree.
- The game ends immediately in defeat when this occurs.
- Display a defeat message/log entry (e.g., "Pippin was caught by a Ringwraith! The quest fails.")

## Implementation Notes
- Each tick, check the positions of all hobbits:
  - If all are at Bree, trigger victory.
  - If any share a tile with a Ringwraith (or are within a defined capture radius), trigger defeat.
- The game loop should halt on win or loss, displaying the appropriate message and allowing the player to quit or restart.

## No Intermediate States (MVP)
- No partial wins (e.g., some hobbits survive).
- No party splitting, hiding, or alternative escape routes for MVP.
- All-or-nothing: all hobbits must survive to Bree.

## Future Extensions
- Partial success (e.g., some hobbits reach Bree, others are lost)
- Stealth/hiding mechanics
- Alternative win conditions (e.g., rescue by Aragorn)
- Dynamic events (e.g., roadblocks, weather)

---
This simple win/loss logic keeps the MVP focused and true to the story, with clear, testable conditions for success and failure.
