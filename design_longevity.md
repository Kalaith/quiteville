# Project Eternity: 6-Month Longevity Design
## "The Spirit of Restoration"

To justify 6 months of real-time engagement in a game that can be mechanically "maxed" in 6 hours, we must shift the goal from **"Completion"** to **"Collection & Legacy"**.

### Core Philosophy: "You don't build *a* town. You rebuild *the world*."

The current town (Quiteville) is not the entire game. It is merely **Chapter 1**.

### The 4 Layers of Time

#### Layer 1: The Settlement (Hours to Days)
*   **Current Gameplay**: Restore a single grid of ruins.
*   **Goal**: Reach "Self-Sufficiency" (Population Cap, Happiness Cap).
*   **Twist**: Once a town is "Finished", you **cannot play it heavily anymore**. It becomes a "completed page" in your Chronicle.

#### Layer 2: The Region (Weeks)
*   **Mechanic**: "The Horizon". Once Quiteville is finished, the camera zooms out. You see it as one node on a map of a desolate continent.
*   **Action**: You select a neighboring tile (Mountain, Coast, Forest) to start a **NEW** town.
*   **Interconnectivity**: Your previous town (Quiteville) now acts as a passive provider. It sends trade caravans to your new town.
    *   *Example*: quiteville produces ample food. Your new "Mountain Outpost" has no farms but rich mines. You *must* link them to survive.
*   **Variety**: Each biome has strict constraints (No farming in mountains, no wood in deserts), forcing you to rely on your legacy towns.

#### Layer 3: The Dynasty (Months)
*   **Mechanic**: "Great Projects".
*   Some ruins are too big for one town. A separate screen shows a massive "Wonder" (e.g., The Cloud Spire).
*   **Goal**: To restore this Wonder, you need **1,000,000 Stone**.
*   **Loop**: You need to build and complete 10 different mining towns over the course of months to generate the passive income required to build the Wonder.
*   **Legacy**: Named Villagers from your first town appear as "Ancestors" or "Legends" in later towns, giving small bonuses.

#### Layer 4: The Seasons (Real Time)
*   **Mechanic**: Slow-mo Sync.
*   The game world reflects real-world seasons.
*   **Winter (Dec-Feb)**: Farming yields drop 90%. You must have spent the Autumn (Sept-Nov) building massive granary towns to stockpile for the entire region.
*   **Justification**: "A game about preparing for the future."

---

### Implementation Roadmap

#### Phase 6: The Horizon (Procedural Map)
1.  **World Map**: A new layer above the Town Map. A grid of Hexes.
2.  **Town State**: Save/Load system that allows "Archiving" a save as a read-only resource generator.
3.  **New Biomes**: Create `ZoneTemplates` that are biome-specific (e.g., "Fishing Hut" only in Coastal towns).

#### Phase 7: Prestige (The Archive)
1.  **Chronicle UI**: A book showing your completed towns.
2.  **Global Resources**: "Legacy Points" earned from completed towns, used to unlock global tech upgrades (e.g., "Better Roads" = faster inter-town trade).

### Immediate Action Item
To test this, we can implement **"Town Completion"**:
*   When all regions are 100% restored, show a "Victory" screen.
*   Offer a button: **"Depart & Start Anew"**.
*   This resets the map (new seed/layout) but keeps a permanent **"+10% Global Production Speed"** multiplier.
*   This makes the player chase infinite multipliers (Incrementalist Approach).
