# Implementation Plan for Idle Town Builder in Rust/Macroquad

## Overview
This plan outlines the implementation of the idle town builder game using Rust and Macroquad, following the project structure defined in AGENTS.md. The game focuses on zone-based town revival with idle mechanics, population pressure, and log-based storytelling.

## Project Structure Alignment
We'll adapt the existing Quiteville structure to fit the idle town builder concept:

- `building/` → Repurposed for `zones/` (zone management, restoration)
- `city/` → Districts and zone collections
- `consequences/` → Population effects and zone interactions
- `data/` → Config loaders for zones, resources, milestones
- `economy/` → Resource management (Energy, Maintenance, Attractiveness, Stability)
- `narrative/` → Event logs, milestones, storytelling
- `save/` → Game state persistence
- `simulation/` → Idle loop, time progression, decay
- `state/` → Game state, views
- `tenant/` → Repurposed for `population/` (population pressure, emergent behaviors)
- `ui/` → Rendering (read-only, action emission)
- `util/` → Cross-platform utilities

## Phase 1: Core Setup and Data Structures (Week 1)

### 1.1 Initialize Project Structure
- Create the src/ module directories as listed above
- Set up Cargo.toml with Macroquad dependencies
- Create basic main.rs with Macroquad window setup
- Implement WASM-compatible file loading for assets (include_str! for JSON)

### 1.2 Define Core Data Structures (state/)
- **Resources**: Struct for the 4 core resources (Energy, Maintenance, Attractiveness, Stability)
- **Zone**: Implement the zone data structure from the design document
  - ZoneId, ZoneCategory, ZoneState, ZoneCapacity, ResourceDelta, CurveModifier, PopulationEffect, DecayModel
- **PopulationPressure**: Float representing population level and growth
- **GameState**: Main state struct containing resources, zones, population, time
- **GameView**: Read-only view for UI consumption

### 1.3 Data Loading (data/)
- Create JSON configs for zones (assets/zones.json)
- Implement config loaders using include_str! for WASM compatibility
- Define zone templates with base stats, effects, and modifiers

## Phase 2: Simulation Engine (Week 2)

### 2.1 Time and Tick System (simulation/)
- Implement game tick loop with configurable time scales
- Add time progression tracking (real time vs game time)
- Create tick function that updates all systems

### 2.2 Resource Management (economy/)
- Implement resource accumulation and consumption
- Add resource caps and decay mechanics
- Create functions for resource deltas from zones

### 2.3 Zone Simulation (zones/ - repurposed from building/)
- Implement zone tick behavior:
  - Calculate effective throughput
  - Apply resource outputs/costs
  - Update condition and activity based on neglect/population
  - Handle dormancy and reawakening triggers

### 2.4 Population System (population/ - repurposed from tenant/)
- Implement population pressure growth/decay
- Add population effects on zone activity
- Create emergent behaviors based on population levels

## Phase 3: UI and User Interaction (Week 3)

### 3.1 Basic UI Framework (ui/)
- Create main UI layout with resource displays
- Implement zone status panels
- Add log display area for narrative events
- Use "dumb" UI pattern: read state, emit actions

### 3.2 UI Actions and State Updates
- Define UiAction enum for user interactions (e.g., restore zone, view logs)
- Implement action handling in main loop
- Update game state based on actions

### 3.3 Visual Feedback
- Display resource bars/graphs
- Show zone condition/activity meters
- Render population pressure indicators

## Phase 4: Narrative and Events (Week 4)

### 4.1 Event Logging (narrative/)
- Implement event system for game happenings
- Create log entries for resource changes, zone events, population shifts
- Add narrative flavor text for immersion

### 4.2 Milestones and Progression
- Define reawakening milestones in JSON
- Implement milestone checking and unlocking
- Add new behaviors/formulas when milestones are reached

## Phase 5: Persistence and Polish (Week 5)

### 5.1 Save/Load System (save/)
- Implement serialization of game state
- Add save/load functionality with JSON storage
- Handle offline progression calculations

### 5.2 Testing and Balancing (util/)
- Create unit tests for simulation logic
- Add balance testing utilities
- Implement debug tools for resource/zone inspection

### 5.3 Final Polish
- Add sound effects and basic animations
- Implement settings for time scaling
- Create initial zone configs and balance numbers

## MVP Milestones

### Very Realistic MVP (Version 0.1)
- [ ] One zone with basic restoration
- [ ] 4 core resources with accumulation/decay
- [ ] Population pressure affecting zone activity
- [ ] Time progression with tick-based updates
- [ ] Basic log display for events

### Ultra-Realistic MVP (2-4 weeks)
- [ ] 1 district with 3 zones
- [ ] Full resource system with caps
- [ ] Population pressure driving zone behaviors
- [ ] Offline progression with save/load
- [ ] Log-based narrative events
- [ ] Reawakening milestones with unlocks

## Technical Considerations

### WASM Compatibility
- Use include_str! for all asset loading
- Avoid std::fs in favor of embedded assets
- Test WebGL builds regularly

### Performance
- Keep simulation lightweight for idle gameplay
- Use efficient data structures for zone updates
- Minimize allocations in hot paths

### Data-Driven Design
- Store all balance numbers in JSON configs
- Allow easy tweaking without code changes
- Version configs for save compatibility

### Code Standards
- Follow readability guidelines from CODE_STANDARDS.md
- Keep modules focused and responsibilities clear
- Target 200-400 lines per file

## Asset Requirements
- Create JSON configs for initial zones and milestones
- Design simple UI textures (64x64 icons for resources/zones)
- Prepare placeholder graphics for zones

## Testing Strategy
- Unit tests for simulation math
- Integration tests for full game ticks
- Manual testing for UI responsiveness
- WASM testing for WebGL builds

This plan provides a structured path to implement the idle town builder while maintaining the established project architecture and coding standards.