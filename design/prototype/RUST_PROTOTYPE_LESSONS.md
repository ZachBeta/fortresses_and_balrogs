# Lessons Learned from Rust Prototype - Iteration 1

This document captures the key challenges, lessons learned, and useful concepts encountered during the initial development phase of the Rust prototype for the Fortresses & Balrogs game. This serves as a reference for future development iterations.

## Challenges Encountered

1.  **ECS (Specs) Integration:**
    *   **Issue:** Initial difficulty getting `Player`, `BlocksMovement`, and `BlocksSight` to correctly implement the `specs::Component` trait.
    *   **Root Cause:** Incorrect `Cargo.toml` setup (missing explicit `specs-derive` dependency initially) and missing `use specs_derive::Component;` in the components module, preventing the `#[derive(Component)]` macro from working as expected.
    *   **Impact:** Numerous `E0277` (trait bound not satisfied) and `E0599` (method not found) errors.

2.  **Module Visibility & Imports (Rust Build System):**
    *   **Issue:** The `ui` module and its `Ui` struct were not accessible from `main.rs`.
    *   **Root Cause:** The `ui` module was not declared as `pub mod ui;` in `src/lib.rs`.
    *   **Impact:** `E0432` (unresolved import) errors.

3.  **Type Mismatches (State Management):**
    *   **Issue:** Assigning `RunState` enum variants to a variable expecting `RunStateStack` in `src/main.rs`.
    *   **Root Cause:** Likely an evolving design for game state management where a more complex stack-based approach (`RunStateStack`) was intended or partially implemented, but assignments were still using the simpler `RunState` enum directly.
    *   **Impact:** `E0308` (mismatched types) errors.

4.  **Borrow Checker with `crossterm` (UI & I/O):**
    *   **Issue:** A significant number of `E0500` (closure requires unique access but already borrowed) and `E0524` (two closures require unique access at the same time) errors in `src/ui/panel.rs` and `src/ui/widget.rs`.
    *   **Root Cause:** Complex chaining of `crossterm` queue operations on `stdout` (e.g., `stdout.queue(...).and_then(|_| stdout.queue(...))`) led to multiple mutable borrows of `stdout` active simultaneously within closures.
    *   **Impact:** Prevented UI rendering logic from compiling.

5.  **Unused Code Warnings:**
    *   **Issue:** Warnings for unused fields (e.g., `VisibleEntities::entities`) and methods in `src/resources/viewshed.rs`.
    *   **Impact:** While not breaking compilation, these indicate incomplete features or dead code that could be cleaned up.

## Key Lessons Learned

1.  **Dependency Configuration is Crucial:**
    *   Always double-check `Cargo.toml` for correct crate versions, feature flags (e.g., `specs`'s `"specs-derive"` feature vs. the separate `specs-derive` crate), and ensure all necessary companion crates are included.

2.  **Master Rust's Module System:**
    *   Careful management of `pub` visibility for modules and items is essential for creating a well-structured library and application. Understand how items are exposed and imported across module boundaries.

3.  **Embrace Rust's Type System:**
    *   Type mismatches are caught early, forcing clarity in data structures and system design (e.g., `RunState` vs. `RunStateStack`). This is a strength, even if it means rethinking initial designs.

4.  **Confront the Borrow Checker Systematically:**
    *   I/O operations and libraries interacting with mutable external state (like `crossterm` with `stdout`) require a solid grasp of Rust's ownership and borrowing rules.
    *   When facing borrow checker errors in chained operations or closures: simplify the chain, explicitly manage lifetimes, or refactor to pass ownership or re-borrow within smaller scopes. Avoid deeply nested closures that all try to mutate the same resource.

5.  **Iterative `cargo check` and Focused Debugging:**
    *   Frequent use of `cargo check` is good, but if a large number of errors appear, try to identify and fix one category of error at a time (e.g., all trait bound errors, then all import errors) rather than jumping between them. This can make the debugging process less overwhelming.

6.  **Prototyping Reveals Real Challenges:**
    *   This prototype iteration, even with its compilation issues, has been highly valuable in highlighting specific areas (ECS setup, `crossterm` usage) that require more careful design or learning.

## Useful Rust Concepts Encountered/Reinforced

*   **Entity Component System (ECS):** `specs` library (Components, Systems, World, Resources).
*   **Procedural Macros:** `#[derive(Component)]` via `specs_derive`.
*   **Module System:** `mod`, `pub mod`, `use`, crate structure (`lib.rs` vs. `main.rs`).
*   **Ownership & Borrowing:** Particularly with mutable references (`&mut`) in closures and chained method calls.
*   **Error Handling:** `Result<T, E>`, `io::Result`, the use of `?` operator.
*   **Traits:** `Component`, `Default`, `Debug`, `Drop`.
*   **Terminal UI:** `crossterm` for raw mode, alternate screen, event handling, and direct terminal manipulation.
*   **State Management:** The distinction and potential need for simple enums (`RunState`) versus more complex structures (`RunStateStack`).

## Next Steps for Prototype (When Resumed)

*   Address the `RunState` vs. `RunStateStack` type mismatch in `main.rs` by clarifying the intended state management approach.
*   Systematically refactor the `crossterm` usage in `src/ui/panel.rs` and `src/ui/widget.rs` to resolve borrow checker issues. This might involve breaking down long chains of `queue` calls or changing how `stdout` is passed and used.
*   Address the warnings for unused code.
*   Re-verify `ui::Ui` import and usage in `main.rs` after other fixes.
