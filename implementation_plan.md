# Implementation Plan for Idle Town Builder

## Overview
This plan outlines the implementation of a **greenfield** idle town builder game using Rust and Macroquad. The game focuses on zone-based town revival with idle mechanics, population pressure, and log-based storytelling. We will build this from scratch, prioritizing WebGL compatibility from day one.

## Project Structure
We will establish a clean, modular structure optimized for this specific game design:

- `src/`
  - `main.rs` → Entry point, window setup
  - `zones/` → Zone logic, state, and behavior (NOT per-building)
  - `city/` → District management and zone collections
  - `economy/` → Core resource math (Energy, Maintenance, Attractiveness, Stability)
  - `population/` → Pressure mechanics, saturation curves
  - `simulation/` → The idle loop, time progression, tick system
  - `narrative/` → Event logs, milestones, flavor text
  - `data/` → Static data types, config loaders
  - `ui/` → "Dumb" UI rendering and input handling
  - `save/` → Serialization and offline progress
  - `assets.rs` → Embedded asset management (wasm-compatible)

## Phase 1: Foundation & Core Math (Week 1)

### 1.1 New Project Setup
- Initialize new Rust project (`cargo init`)
- Configure `Cargo.toml` with `macroquad`, `serde`, `serde_json`, `quad-rand`
- Set up `Makefile` or scripts for easy WebGL building (`cargo build --target wasm32-unknown-unknown`)
- Create `src/assets.rs` using `include_str!` for all initial JSON data

### 1.2 Core Data Structures (`state/` & `data/`)
- **Resources**: Struct for the 4 core resources (E, M, A, S)
- **Zone**: Implement the layout from `idle_town_builder_the_quiet_town.md`
- **PopulationPressure**: The central float value driving the game
- **GameState**: The root struct holding it all

### 1.3 The Math Engine (`formulas_and_statistics.md`)
- Implement the "Unbreakable" formulas as pure functions (unit testable)
- **Unit Tests First**: Write tests to verify:
  - `EffectivePopulation` saturation curve
  - `ResourceCost` scaling (P^2)
  - `OfflineGain` logarithmic scaling
- Ensure the math behaves exactly as designed before connecting a UI

## Phase 2: The Simulation Loop (Week 2)

### 2.1 The Tick (`simulation/`)
- Implement `tick(state, delta_time)`
- Differentiate between:
  - **Fast Ticks**: Visual updates (animations)
  - **Game Ticks**: Logic updates (resource accumulation, 1/sec)
- Implement `process_offline_time` logic upon load

### 2.2 Zone Logic (`zones/`)
- Implement `calculate_throughput()`
- Connect Zone outputs to Global Resource state
- Implement `condition` decay and `activity` updates

## Phase 3: "Dumb" UI & Interaction (Week 3)

### 3.1 Basic Rendering
- Create a debug view to visualize the 4 resources and Population Pressure
- Render a list of Zones with their current status (Active/Dormant, Condition %)
- **WebGL Check**: Verify rendering works in browser build

### 3.2 Action System
- Define `Action` enum: `RestoreZone`, `PrioritizeZone`, etc.
- Implement an immediate-mode UI that emits these actions
- Apply actions to `GameState` in the main loop

## Phase 4: Narrative & Polish (Week 4)

### 4.1 The Log System (`narrative/`)
- Create a scrolling text log for game events
- Hook up "Milestone" triggers based on resource/zone thresholds
- Implement the "Reawakening" visual feedback (text only for MVP)

### 4.2 Save/Load
- Implement `serde` for `GameState`
- use `macroquad::miniquad::conf` logic or local storage for web persistence

## VP Milestones (Vertical Slices)

### Slice 1: The Math Check
- **Goal**: Run the app, see numbers go up.
- **Content**: 1 Zone (hardcoded properties), Population 0 -> 10.
- **Verification**: Tweak variables, confirm no spirals.

### Slice 2: The Loop
- **Goal**: Player can "Restore" the zone to increase stats.
- **Content**: "Old Homestead" zone config.
- **Interaction**: Click "Restore" -> Resources needed -> Zone activates.

### Slice 3: The MVP (Web Ready)
- **Goal**: Full game loop with save/load in browser.
- **Content**: 3 Zones, Reawakening Milestone 1.

## Technical Rules
- **WASM First**: No `std::fs`. All assets embedded or loaded async (if needed, but embedded preferred for text).
- **No Global Statics**: Pass `GameState` explicitly.
- **Strict Math**: Do not deviate from the log/sqrt formulas without updating the design doc.
- **Data-Driven Core**: All game balance numbers (Zone outputs, Resource caps, Event triggers) MUST be defined in JSON assets, not Rust code. Hardcoding values is forbidden.