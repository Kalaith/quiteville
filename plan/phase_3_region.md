# Phase 3: The Horizon (Region Map) (Month 3)

**Goal**: Implementing the "Layer 2" region gameplay. Breaking out of the single map to a world of interconnected towns, enabling the specific "Legacy" gameplay loop where players settle, build, and move on.

## Week 9: World Map Architecture
*Focus: The meta-layer foundation.*

### 1. Overworld Scene
*   [ ] **Scene Switcher**:
    *   Implement state transition `TownView` <-> `RegionView`.
    *   Maintain memory state of the active town when switching out.
*   [ ] **Node Map Data Structure**:
    *   Graph-based map: Nodes (Town Sites, Points of Interest) and Edges (Routes).
    *   Visuals: 3D or 2D "Paper Map" aesthetic showing the region.

### 2. Multi-Town Save Architecture
*   [ ] **Save File Refactor**:
    *   `GlobalSave`: Stores Region Map state + List of `TownData` summaries.
    *   `TownSave`: Individual files (or blobs) for each specific town layout.
*   [ ] **Loading Logic**:
    *   Only load the *Active* town's full entity list into ECS.
    *   Inactive towns exist only as `SimulationProxy` structs (see Week 11).

## Week 10: Procedural Generation (The Horizon)
*   [ ] **Biome System**:
    *   *Forest*: +Wood, -Stone, Temperate weather.
    *   *Desert*: +Stone, -Food, Hot weather.
    *   *Coast*: +Fish/Trade, High humidity.
*   [ ] **Map Generation Algorithm**:
    *   Voronoi or Perlin Noise based generation for the Region Map.
    *   Seed-based generation for consistency.
*   [ ] **Site Attributes**:
    *   Each node has `ResourcePotentials` (e.g., "Rich Iron Deposit").

## Week 11: Town States & Archiving (The Legacy)
*   [ ] **Abstract Simulation (The "Freeze")**:
    *   When leaving a town, convert it to a `ProductionNode`.
    *   Calculate: `NetOutput = Production - Consumption`.
*   [ ] **Simulation Proxy**:
    *   Inactive towns update once per day (not per tick).
    *   Logic: `Stockpile += NetOutput`.
*   [ ] **Events**:
    *   Even inactive towns can trigger low-fidelity events ("Flood in Town A requires 100 Wood").

## Week 12: Inter-Town Trade & Caravans
*   [ ] **Trade Routes**:
    *   Player defines: "Send 50 Wood/Day from Town A to Town B".
    *   Requirement: Route must be discovered/safe.
*   [ ] **Caravan Unit**:
    *   Visual representation on World Map moving between nodes.
    *   Speed determined by Infrastructure upgrades (Dirt Road -> Paved Road).
*   [ ] **Global Stockpile (The Capital)**:
    *   Designating one town as the "Capital" where "Great Projects" are built using regional resources.
