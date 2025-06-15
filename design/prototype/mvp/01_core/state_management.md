# State Management

## Overview
This document outlines the state management system for handling different game states and transitions.

## State Stack

### Core States
- `Splash`: Initial loading screen
- `MainMenu`: Main menu interface
- `Game`: Main gameplay state
- `Pause`: Pause menu
- `GameOver`: End game screen

### State Definition
```rust
pub enum RunState {
    Splash { display_time: f32 },
    MainMenu { menu_selection: usize },
    Game { paused: bool },
    Pause { previous_state: Box<RunState> },
    GameOver { score: u32 },
}

pub struct StateStack {
    states: Vec<RunState>,
}

impl StateStack {
    pub fn new(initial_state: RunState) -> Self {
        Self {
            states: vec![initial_state],
        }
    }

    pub fn push(&mut self, state: RunState) {
        self.states.push(state);
    }

    pub fn pop(&mut self) -> Option<RunState> {
        if self.states.len() > 1 {
            self.states.pop()
        } else {
            None
        }
    }

    pub fn current(&self) -> Option<&RunState> {
        self.states.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut RunState> {
        self.states.last_mut()
    }
}
```

## State Transitions

### Transition Types
- **Push**: Add a new state on top of the stack
- **Pop**: Remove the current state and return to previous
- **Switch**: Replace the current state with a new one
- **Quit**: Exit the game

### Transition Definition
```rust
pub enum StateTransition {
    Push(RunState),
    Pop,
    Switch(RunState),
    Quit,
}
```

## State Management System

### State Resources
```rust
pub struct StateManager {
    stack: StateStack,
    transitions: Vec<StateTransition>,
}

impl StateManager {
    pub fn new(initial_state: RunState) -> Self {
        Self {
            stack: StateStack::new(initial_state),
            transitions: Vec::new(),
        }
    }

    pub fn push_transition(&mut self, transition: StateTransition) {
        self.transitions.push(transition);
    }

    pub fn process_transitions(&mut self) -> bool {
        let mut should_quit = false;
        
        for transition in self.transitions.drain(..) {
            match transition {
                StateTransition::Push(state) => {
                    self.stack.push(state);
                }
                StateTransition::Pop => {
                    self.stack.pop();
                }
                StateTransition::Switch(state) => {
                    self.stack.pop();
                    self.stack.push(state);
                }
                StateTransition::Quit => {
                    should_quit = true;
                    break;
                }
            }
        }
        
        should_quit
    }
}
```

## State-Specific Logic

### State Trait
```rust
pub trait State {
    fn on_enter(&mut self, world: &mut World) {}
    fn on_exit(&mut self, world: &mut World) {}
    fn update(&mut self, world: &mut World) -> StateTransition;
    fn render(&self, world: &World) -> Result<()>;
    fn handle_event(&mut self, world: &mut World, event: &Event) -> StateTransition;
}
```

### Example State Implementation
```rust
pub struct GameState {
    // Game-specific state
}

impl State for GameState {
    fn on_enter(&mut self, world: &mut World) {
        // Initialize game state
    }

    fn update(&mut self, world: &mut World) -> StateTransition {
        // Update game logic
        StateTransition::None
    }

    fn render(&self, world: &World) -> Result<()> {
        // Render game
        Ok(())
    }

    fn handle_event(&mut self, world: &mut World, event: &Event) -> StateTransition {
        match event {
            Event::KeyPress(KeyCode::Escape) => StateTransition::Push(RunState::Pause),
            _ => StateTransition::None,
        }
    }
}
```

## Integration with ECS

### State Resources
```rust
pub struct StateResources {
    pub state_manager: StateManager,
    pub current_state: Option<Box<dyn State>>,
}

impl Default for StateResources {
    fn default() -> Self {
        Self {
            state_manager: StateManager::new(RunState::Splash { display_time: 0.0 }),
            current_state: None,
        }
    }
}
```

### State System
```rust
pub struct StateSystem;

impl<'a> System<'a> for StateSystem {
    type SystemData = (
        Write<'a, StateResources>,
        WriteExpect<'a, World>,
        Read<'a, EventChannel<Event>>,
    );

    fn run(&mut self, (mut state_res, mut world, events): Self::SystemData) {
        // Process state transitions
        if state_res.state_manager.process_transitions() {
            // Handle quit
            return;
        }

        // Update current state
        if let Some(state) = &mut state_res.current_state {
            // Process events
            for event in events.read() {
                let transition = state.handle_event(&mut world, event);
                if !matches!(transition, StateTransition::None) {
                    state_res.state_manager.push_transition(transition);
                }
            }

            // Update state
            let transition = state.update(&mut world);
            if !matches!(transition, StateTransition::None) {
                state_res.state_manager.push_transition(transition);
            }
        }
    }
}
```

## Best Practices

### State Design
- Keep states focused and single-purpose
- Minimize state-specific code in the main loop
- Use the state stack for hierarchical state management
- Clean up resources when exiting states

### Performance
- Lazy-initialize heavy resources
- Cache frequently accessed data
- Consider state pooling for frequently used states

### Testing
- Test state transitions
- Verify resource cleanup
- Test event handling in different states

## Future Extensions
- State history and time travel
- Network synchronization
- Save/load state functionality
- Replay system
