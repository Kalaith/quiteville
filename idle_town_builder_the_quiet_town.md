# Idle Town Builder: The Quiet Town

## Tagline
A relaxing idle town builder about reviving a small town that grows when you're not watching. A living 2D world that breathes on its own.

## Core Fantasy
You are the caretaker of a small town in a post-apocalyptic world.

The world has ended, but survivors are slowly returning, seeking the simple life. You are helping rebuild this forgotten town from wasteland to paradise.

People don't appear because you clicked "Build House." They appear because the peaceful, self-sufficient life here becomes appealing again. They arrive, they build, they live.

## Visual Experience: A Living World
The game is played on a **Top-Down 2D Tilemap**. You are not just looking at a spreadsheet; you are watching a simulation.

- **The Map**: A sprawling, pre-designed landscape of ruins, forests, and overgrown paths.
- **The Ruins**: Old Homesteads, Village Greens, and Wells lie dormant, waiting for you to clear the debris and bring them back to life.
- **The Villagers**: Tiny autonomous agents that wander, work, sleep, and socialize. You watch their daily lives unfold.

## How It Marries Idle + Caretaking + Simulation

**Idle Side (The Economy)**
Resources accumulate over time:
- **Materials**: Scavenged and produced by the town.
- **Attractiveness**: The pull that brings new people in.
- **Stability**: The glue that keeps them staying.

Systems continue running offline. Progress happens in long arcs.

**Caretaking Side (Your Input)**
- You **Restore** ancient ruins rather than placing generic buildings.
- You **Clear** debris to open new paths.
- You **Prioritize** needs (e.g., "We need more housing before we rebuild the market").

**Simulation Side (The Agents)**
Citizens are not just numbers. They are **Agents**.
- **Hunger**: They need food markets.
- **Energy**: They need homes to sleep in.
- **Social**: They need community spaces (Wells, Parks).

If you neglect their needs, they might leave, or the town will stagnate. If you nurture them, they thrive.

## The Town Is a System
The town is divided into **Zones** that occupy physical space on the map.

**Example: The Old Homestead**
- **State**: Starts as "Overgrown Ruin".
- **Action**: You spend Materials to "Restore" it.
- **Result**: It becomes a functioning house. Villagers move in.
- **Live Effect**: You see villagers walking to it at night to sleep.

**Example: The Village Green**
- **State**: A muddy patch of weeds.
- **Action**: You restore it.
- **Result**: It becomes a gathering spot.
- **Live Effect**: Villagers congregate here during the day to fulfill Social needs, boosting Town Stability.

## Progression: Reawakening
You don't just "level up." You reawaken sections of the map.
1.  **Start**: A single campfire in the woods.
2.  **Phase 1**: Restore the central Homesteads.
3.  **Phase 2**: Clear the road to the Old Well.
4.  **Phase 3**: Rebuild the Bridge to the outer farmlands.

Each expansion is visually rewarding—you see the lights turn on, the paths clear, and the life return.

## Controls & UI
- **Camera**: WASD or Click-and-Drag to explore the world. Zoom in to see pixel-art details, Zoom out to see the whole town.
- **Interaction**: Click on ruins to see their requirements. Click on villagers to see what they are thinking ("Going home to sleep", "Looking for food").
- **UI**: Minimalist floating panels that don't obscure the view. The focus is on the world.

## The Vibe
A town that whispers. A game that ambles. Progress measured in serenity, not spectacles. 
Watch your little people live their lives. 🌳✨

## Technical Pillars (Rust + Macroquad)
- **ECS-Lite Simulation**: High performance agent simulation (Needs, Pathfinding, State Machines).
- **Data-Driven Designs**: All balance values (Zone Outputs, Decay Rates, Costs) are loaded from JSON for easy tweaking.
- **Offline Calculation**: The game simulates "what happened while you were gone" so you never lose progress.