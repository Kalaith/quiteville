# Comprehensive Testing Plan (Phases 1-6)

This document outlines the testing criteria for the entire game development roadmap, from the foundation to the final release.

## Phase 1: Foundation & Core Loop

### Construction & Environment
- [ ] **Construction Sites**: Verify blueprints create `ConstructionSite` entities.
- [ ] **Build Process**: Confirm agents carry materials to sites and progress updates visually (scaffolding).
- [ ] **Debris Clearing**: Test selecting debris, assigning agents, and receiving rewards (resources/space).
- [ ] **Zone Types**: Verify all basic zones built correctly:
    - Housing: Tent, Shack, Cottage, Brick House.
    - Service: Well, Outhouse, Campfire, Market Stall.
    - Production: Scavenger Hut, Woodcutter, Quarry.

### Agent Simulation
- [ ] **Jobs**: Verify agents accept jobs (Laborer, Farmer, Cook, etc.) and go to their assigned workplace.
- [ ] **Needs**: Check Hunger and Energy decay and restoration (Eating, Sleeping).
- [ ] **Day/Night Cycle**: Confirm agents sleep at night and work during the day.
- [ ] **Pathfinding**: Ensure agents navigate between home, work, and service buildings.

### UI/UX
- [ ] **Tooltips**: Hover over entities/UI to see details.
- [ ] **Resource Bar**: Verify income/expense tracking updates correctly.
- [ ] **Alerts**: Click on alerts (Homeless, Starving) to snap camera to target.
- [ ] **Event Log**: Check for messages (Death, Unlocks).

### Tech & Economy
- [ ] **Milestone Unlocks**: Verify actions trigger unlocks (e.g., Harvest 100 Wheat -> Bakery).
- [ ] **Memory Fragments**: Confirm drop rate from debris and usage at Research Bench.
- [ ] **Balance**: Ensure no soft-locks (e.g., running out of wood with no way to get more).

---

## Phase 2: Advanced Simulation

### Seasons & Weather
- [ ] **Season Cycle**: Verify 12-day transition (Spring -> Summer -> Autumn -> Winter).
- [ ] **Seasonal Effects**:
    - Winter: No farming, cold penalty.
    - Summer: Thirst increase.
- [ ] **Weather Events**:
    - Rain: Waters crops.
    - Storm: Damages buildings (check condition decay).
    - Snow: Visual shader and movement slowdown.

### Resource Chains
- [ ] **Processing**: Verify raw -> processed chain (Log -> Lumber, Grain -> Flour).
- [ ] **hauler Logic**: Confirm haulers move goods between stockpiles and processors.

### Town Management
- [ ] **Upgrades**: Test upgrading Tent -> Shack using resources.
- [ ] **Maintenance**: Verify buildings decay and require repair.
- [ ] **Prioritization**: Check if "High Priority" tasks are done first.

### Social Simulation
- [ ] **Traits**: Verify agents spawn with traits (NightOwl, Glutton).
- [ ] **Social Graph**: distinct interactions (Chat, Argue) affecting Opinion.
- [ ] **Chronicle**: Check history logging for town events.

---

## Phase 3: Region & Expansion

### World Map
- [ ] **Map Switch**: Toggle between Town View and Region View.
- [ ] **State Persistence**: Verify town simulation pauses/resumes correctly.
- [ ] **Procedural Gen**: Generate new region maps with different seeds.

### Multi-Town Management
- [ ] **Settling**: Send colonists to a new node and start a new town.
- [ ] **Proxy Simulation**: Verify archived towns continue to produce/consume resources daily.
- [ ] **Events**: Trigger global events affecting inactive towns.

### Trade
- [ ] **Caravans**: Send resources between towns. Verify arrival and stockpile update.
- [ ] **Routes**: Ensure travel time scales with road upgrades.

---

## Phase 4: Legacy & Dynasty

### Prestige
- [ ] **Chronicle Book**: View history of all past towns.
- [ ] **Legacy Points**: Verify points accumulation from completed towns.

### Wonders
- [ ] **Construction**: Multi-stage building of Wonder sites.
- [ ] **Visuals**: Map changes upon completion.

### Ancestors
- [ ] **Retirement**: Retire a villager to the Hall of Heroes.
- [ ] **Ancestral Spirits**: Verify buff spawning in new towns.

---

## Phase 5: Visual Polish

### Audio & Visuals
- [ ] **Particles**: Smoke from chimneys, weather effects (rain, snow).
- [ ] **UI Polish**: Verify animations and thematic textures.

### Narrative
- [ ] **Tutorial**: intro sequence, Guide character dialogue, contextual hints.
- [ ] **Intro**: Cinematic pan.

---

## Phase 6: Release Readiness

### Content
- [ ] **Biomes**: Test Desert, Tundra, Swamp generation.
- [ ] **Building Variety**: Verify all 50+ structures function.
- [ ] **Events**: Trigger random narrative events.

### Achievements & Stats
- [ ] **Achievements**: Verify triggers for "First House", "Utopia", etc.
- [ ] **Lifetime Stats**: Check persistence of "Total Bricks Laid" across saves.
