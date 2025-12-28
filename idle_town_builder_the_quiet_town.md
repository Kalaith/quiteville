# Idle Town Builder: The Quiet Town

## Tagline
A relaxing idle town builder about reviving a small town that grows when you're not watching.

This is not a booming empire game. It's a slow-breathing town, one revival at a time.

## Core Fantasy
You are the caretaker of a small town in a post-apocalyptic world.

The world has ended, but survivors are slowly returning, seeking the simple life. You are helping rebuild this forgotten town from wasteland to paradise.

People don't appear because you clicked "Build House." They appear because the peaceful, self-sufficient life here becomes appealing again.

## How It Marries Idle + Caretaking

**Idle Side (Numbers Flow)**
Resources accumulate over time:
- Energy
- Maintenance
- Attractiveness
- Stability

Systems continue running offline. Progress happens in long arcs, not rapid taps.

**Caretaking Side (Meaningful Input)**
- You choose which systems to restore
- You set priorities, not orders
- You react to slow-emerging problems rather than constant alerts

Idle provides the heartbeat. Caretaking provides the intent. 💡

## The Town Is a System, Not a Grid
Instead of classic tile spam:

The town is divided into Zones (Old Homestead, Village Green, Winding Paths)

Each zone has:
- Condition
- Capacity
- Behavior modifiers

You unlock zones through restoration, not currency dumps.

A zone at 40% behaves fundamentally differently than one at 90%.

## Buildings Are Abstracted (Low Art, High Depth)
You don't place 50 houses. You restore Housing Infrastructure.

Each "building" represents:
- A bundle of structures
- Internal simulation logic
- Long-term consequences

Example: Restoring "Old Homestead A" increases population growth but also increases simple chores, community gatherings, and system strain over time.

This keeps the idle numbers clean while still feeling alive.

## Population: Soft, Autonomous, Emergent
Citizens are not units. They are pressure.

They generate:
- Needs
- Culture
- Problems
- Momentum

As population grows:
- New roles emerge automatically
- Subcultures form
- Some zones become self-sustaining, others fragile

You see this through:
- Reports
- City logs
- Subtle stat shifts

No micromanagement hell. 😌

## The Idle Loop (Minute-to-Hour Scale)

**Short Term (minutes):**
- Resources tick
- Small issues surface
- Minor optimizations

**Mid Term (hours):**
- Zones shift behavior
- New inhabitants arrive
- Systems stabilize or degrade

**Long Term (days):**
- Entire districts transform
- Locked systems awaken
- Rare events occur only if conditions persist

Logging back in feels like opening a letter from the city.

## Progression Without Bloat
Reawakening Milestones

Instead of prestige spam:
- "The Winding Paths Remember Their Way"
- "The Village Green Begins to Flourish"
- "Simple Gatherings Emerge"

Each milestone:
- Adds new idle formulas
- Changes population behavior
- Unlocks problems worth solving

Prestige becomes recontextualization, not reset.

## Failure Is Gentle (But Real)
You don't lose. You stagnate.

- Population plateaus
- Zones go dormant
- Systems stop producing surplus

Recovery is always possible, but it takes patience and planning.

This keeps the game relaxing instead of punitive. 🌙

## Why This Is Very "You"
- System-driven instead of content-driven
- Idle-friendly, works while you're away
- Observational fun, not click fatigue
- Expandable forever with new zones and systems
- Built in layers, perfect for evening dev sessions

Also, this slots cleanly into:
- Rust + Macroquad
- Web-based idle UI
- Or even a hybrid desktop/web build later

## The Vibe
A town that whispers. A game that ambles. Progress measured in serenity, not spectacles. 🌳✨

## Zone Data Structures

### Mental Model: What a Zone Is
A Zone is not a building. It is a bundle of behaviors that:
- Consumes global resources
- Produces global effects
- Modifies population pressure
- Changes how the city behaves over time

Zones don't spam numbers. They bend curves.

### Zone Responsibilities
Each zone answers four questions:
- What does it need to function?
- What does it produce when healthy?
- How does it degrade when neglected?
- How does it change the city's rules?

If a zone doesn't touch at least two core systems, it shouldn't exist.

### Core Zone Data (Conceptual)
```
Zone
├─ identity
│  ├─ id
│  ├─ name
│  └─ category
├─ state
│  ├─ condition        (0.0 – 1.0)
│  ├─ activity         (0.0 – 1.0)
│  └─ dormancy         (bool)
├─ capacity
│  ├─ base_throughput
│  └─ saturation_bias
├─ effects
│  ├─ resource_outputs
│  ├─ resource_costs
│  └─ curve_modifiers
├─ population
│  ├─ attraction
│  ├─ strain
│  └─ decay
├─ progression
│  ├─ reawakening_stage
│  └─ unlocked_behaviors
└─ decay
   ├─ natural_decay_rate
   └─ neglect_thresholds
```

### Key Fields Explained (Why They Exist)

**condition**
Represents physical integrity.
- Affects output multiplicatively
- Recovers slowly
- Decays faster under strain

**activity**
Represents usage and life.
- Driven by population pressure
- High activity increases output and costs
- Can drop even if condition is high

This separation avoids the classic "repair = win" trap.

**base_throughput**
The zone's raw contribution before math.
Think:
- Residential: population attraction
- Market: resource conversion
- Transit: efficiency multipliers

Never scale this linearly with level.

**saturation_bias**
How fast this zone hits diminishing returns.
- High bias: Strong early impact, Plateaus quickly
- Low bias: Slow burn, Long-term stabilizer

### Curve Modifiers (The Secret Sauce)
Zones don't say "+20% Energy." They say things like:
- +0.15 to EnergyFactor numerator
- +2 to Population saturation constant K
- -0.05 Stability decay per tick

This keeps math tame forever.

### Concrete Rust-Like Sketch
```rust
struct ZoneId(u32);

enum ZoneCategory {
    Residential,
    Market,
    Infrastructure,
    Cultural,
    Transit,
    Utility,
}

struct ZoneState {
    condition: f32,   // 0.0 - 1.0
    activity: f32,    // 0.0 - 1.0
    dormant: bool,
}

struct ZoneCapacity {
    base_throughput: f32,
    saturation_bias: f32,
}

struct ResourceDelta {
    energy: f32,
    maintenance: f32,
    stability: f32,
    attractiveness: f32,
}

struct CurveModifier {
    energy_bias: f32,
    maintenance_bias: f32,
    stability_bias: f32,
    population_k_delta: f32,
}

struct PopulationEffect {
    attraction: f32,
    strain: f32,
    decay: f32,
}

struct DecayModel {
    natural_rate: f32,
    neglect_threshold: f32,
}

struct Zone {
    id: ZoneId,
    name: String,
    category: ZoneCategory,

    state: ZoneState,
    capacity: ZoneCapacity,

    output: ResourceDelta,
    upkeep: ResourceDelta,

    curves: CurveModifier,
    population: PopulationEffect,

    reawakening_stage: u8,
    decay: DecayModel,
}
```

### Zone Tick Behavior (High Level)
Each tick:
- Skip if dormant
- Compute effective throughput: throughput = base_throughput × condition × activity × saturation(activity, bias)
- Apply outputs and upkeep
- Adjust condition based on neglect
- Adjust activity based on population pressure
- Possibly trigger reawakening or dormancy

No zone acts alone. Everything feeds the global city state.

### Example Zone: Old Residential Block
- Category: Residential
- Base Throughput: Low
- Saturation Bias: High

Effects:
- + Attractiveness (early)
- + PopulationPressure
- - Maintenance (scales with activity)

Curve Modifiers:
- + Population saturation K
- - Stability decay (small)

Failure Mode: High activity + low maintenance → rapid condition decay

Feels alive. Behaves gently.

### Why This Won't Collapse Later
- Adding new zones doesn't multiply numbers
- Zones compete for attention organically
- Content expansion is horizontal, not vertical
- You can add weird zones without rewriting math

This structure will survive years of tinkering.</content>
<parameter name="filePath">h:\WebHatchery\games\quiteville\idle_city_builder_the_quiet_district.md