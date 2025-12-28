# Phase 2: Advanced Simulation & Depth (Month 2)

**Goal**: Deepening the simulation to support the "Longevity" design. The town must transition from simple survival to a complex, interconnected system where planning for seasons and managing agent relationships becomes critical.

## Week 5: Seasons & Weather
*Focus: Long-term planning loops.*

### 1. Seasonal Logic (Cycle Duration: ~12 In-Game Days)
*   [ ] **Spring**:
    *   *Effect*: +Growth Rate (Farms), +Morale (Hope).
    *   *Visuals*: Green tint, Blooming particles, Rain showers.
*   [ ] **Summer**:
    *   *Effect*: +Movement Speed, -Hydration (requires water).
    *   *Visuals*: Warm/Yellow tint, Heat haze.
*   [ ] **Autumn**:
    *   *Effect*: Harvest Bonus, Wind slowing movement.
    *   *Visuals*: Orange/Brown palette, Falling leaves.
*   [ ] **Winter**:
    *   *Effect*: No Farming, Cold (requires Warmth/Fuel), Snow slows movement.
    *   *Visuals*: White/Blue tint, Snow layer shader on ground/buildings.

### 2. Weather System
*   [ ] **Dynamic Weather State**:
    *   States: `Sunny`, `Rain`, `Storm`, `Snow`, `Fog`.
    *   Transitions based on probability weights per Season.
*   [ ] **Gameplay Impact**:
    *   *Rain*: Waters crops automatically.
    *   *Storm*: Chance to damage buildings (Generate `Debris`).

## Week 6: Resource Chains & Logistics
*   [ ] **Refined Resources**:
    *   *Raw*: `Log`, `StoneChunk`, `Grain`.
    *   *Processed*: `Lumber` (from Logs), `CutStone` (from Chunks), `Flour` (from Grain).
*   [ ] **Processing Buildings**:
    *   *Sawmill*: Converts Log -> Lumber.
    *   *Mason's Yard*: Converts Chunk -> CutStone.
    *   *Windmill*: Converts Grain -> Flour.
*   [ ] **Supply Chain Logic**:
    *   Agents must move goods from `source` -> `processor` -> `storage`.
    *   Introduction of `Hauler` job role specifically for logistics.

## Week 7: Building Upgrades & Town Management
*   [ ] **Upgrade System**:
    *   Buildings can evolve if conditions are met.
    *   *Cost*: Resources + Tech Requirement.
    *   *Example*: `Tent` -> `Shack` -> `Cottage` -> `Manor`.
*   [ ] **Town Board (Prioritization)**:
    *   UI Panel where players can flag resource types or buildings as "High Priority".
    *   Agents with `Job::Laborer` prioritize these tasks over others.
*   [ ] **Maintenance**:
    *   Buildings decay over time (Decay Rate).
    *   Require periodic "Repair" actions using materials.

## Week 8: Emergent Storytelling (The "RimWorld" Element)
*   [ ] **Agent Traits**:
    *   Randomized on spawn relative to Archetype.
    *   *Examples*: `NightOwl` (Bonus work at night), `Glutton` (Eats 2x), `Charismatic` (+Social).
*   [ ] **Social Graph**:
    *   Track `Opinion` between agents (-100 to +100).
    *   *Interactions*: Chat (Opinion+), Argue (Opinion-), Share Meal (Opinion++).
*   [ ] **Memory & Gossip**:
    *   Agents remember key events ("Witnessed Death", "Attended Party").
    *   Memories decay but impact mood while active.
*   [ ] **Feat Tracking**:
    *   Persistent log of "History" for the town viewable in a "Chronicle" UI.
