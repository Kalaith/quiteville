# Phase 1: Foundation & Core Loop Refinement (Month 1)

**Goal**: Solidifying the current MVP into a robust, bug-free, and feature-complete "Single Town" experience. Before expanding to the region, the core town gameplay must be engaging for at least 1-2 hours of active play.

## Week 1: Build System V2 - Dynamic Construction & Environment
*Focus: moving from instant gratification to meaningful progression.*

### 1. Dynamic Construction
*   [ ] **Construction Site Entity**:
    *   Placing a blueprint creates a `ConstructionSite` entity instead of the finished building.
    *   Site requires **Materials** (Wood, Stone) and **Labor** (Work units).
*   [ ] **Agent Integration**:
    *   Agents must physically travel to the site to "build" (adding progress).
    *   Visual feedback: Scaffolding mesh that changes stages (e.g., 0% -> 50% -> 100%).
*   [ ] **Component Updates**:
    *   New `ConstructionProgress` component: `{ current_work: f32, max_work: f32, materials_deposited: Vec<Resource> }`.

### 2. Expanded Zone Types (~15 Types)
*   [ ] **Housing**:
    *   *Tent* (Starter, cheap, low comfort)
    *   *Shack* (Early game, wood)
    *   *Cottage* (Mid game, stone/wood)
    *   *Brick House* (Late Phase 1, high comfort)
*   [ ] **Service**:
    *   *Well* (Water source)
    *   *Outhouse* (Hygiene)
    *   *Campfire* (Social/Warmth)
    *   *Market Stall* (Distribution)
*   [ ] **Production**:
    *   *Scavenger Hut* (Debris clearing speedup)
    *   *Woodcutter's Block* (Wood)
    *   *Stone Quarry* (Stone)

### 3. Debris & Clearing (Fog of War)
*   [ ] **World Generation**: Map, or at least the expansion zones, should be filled with `Debris` (Ruins, Rubble, Overgrowth).
*   [ ] **Clearing Mechanic**:
    *   Select debris -> Assign "Clear" order.
    *   Agents perform work to remove it.
    *   **Reward**: Resources (Scrap), Map Space, or `MemoryFragments`.

## Week 2: Agent AI V2 - The Living Town
*Focus: making agents feel like purpose-driven inhabitants.*

### 1. Jobs System
*   [ ] **Role Assignment**:
    *   Agents are no longer generic wanderers. They have a `Job` variant.
    *   *Roles*: Laborer (Default), Farmer, Cook, Scavenger, Builder.
*   [ ] **Workplaces**:
    *   Buildings have `Workplace` component with `max_slots`.
    *   Agents are linked to a specific building ID for their shift.

### 2. Day/Night Cycle & Schedule
*   [ ] **Global Time**: Implement 24-hour clock (e.g., 1 game hour = 1 real minute).
*   [ ] **Daily Schedule State Machine**:
    *   *Morning*: Wake up, Hygiene, Breakfast.
    *   *Work*: Go to workplace, execute production logic.
    *   *Evening*: Socialize, Recreation, Dinner.
    *   *Night*: Sleep (Find Bed -> Find Floor -> Debuff).
*   [ ] **Visuals**: Day/Night lighting cycle (Sun orbit).

### 3. Needs Tuning
*   [ ] **Balance Pass**:
    *   Hunger should allow for ~1 day survival without food before health impact.
    *   Energy decays during work, requires Sleep to restore.
    *   New Need: **Spirit/Hope** (Impacted by "Aesthetics" and "Social").

## Week 3: UI/UX Refinement
*Focus: Clarity and feedback.*

### 1. Tooltips & Inspect
*   [ ] **Global Hover**:
    *   Mouse over entities -> Show Info Panel (Stats, Contents, Owner).
    *   Mouse over UI -> Show Explainer (Cost, Effect).
*   [ ] **Resource Tracking**: Top bar breakdown of income/expense per minute.

### 2. Notifications & Feedback
*   [ ] **World Space Floating Text**:
    *   "+1 Wood" (Green), "-10 Health" (Red).
*   [ ] **Event Log**:
    *   Scrollable text box: "Agent Smith has died of starvation.", "New research unlocked: Farming."

### 3. Alerts System
*   [ ] **Critical Indicators**:
    *   Icons for "Homeless", "Starving", "Building Idle (No Input)".
    *   Clicking alert snaps camera to relevant entity.

## Week 4: Tech Tree & Discovery
*Focus: Progression and unlock pacing.*

### 1. Discovery System (Action-Based Unlocks)
*   [ ] **Logic**: Techs are unlocked by completing in-game *Milestones* rather than just spending points.
    *   *Trigger*: "Harvest 100 Wheat" -> Unlocks *Bakery*.
    *   *Trigger*: "Clear Ancient Ruin" -> Unlocks *Stone Masonry*.
*   [ ] **Data Structure**: `UnlockCondition` enum supporting complex triggers.

### 2. Memory Fragments & Artifacts
*   [ ] **Loot Table**: Clearing Debris has chance to drop `MemoryFragments`.
*   [ ] **Research Bench**: Constructible building where Fragments are spent to unlock passive bonuses ("Lost Tech").

### 3. Economy Balancing
*   [ ] **Spreadsheet Model**:
    *   Define input/output rates for all buildings.
    *   Target a smooth difficulty curve (survivable but demanding).
*   [ ] **Configuration**: Move all balance constants (costs, rates) to hot-loadable JSON/TOML files.
