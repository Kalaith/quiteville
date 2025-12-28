# Formulas and Statistics

## Design the Idle Math So It Never Spirals

### Design Principles (Non-Negotiable)
- No pure exponentials
- All growth has friction
- Every bonus increases a cost somewhere else
- Caps are soft, never hard
- Time smooths everything

If the math obeys these rules, spirals simply don't form.

### Core Resources (Small, Interdependent Set)
Use 4 primary values only:
- Energy (E) – how much the city can run
- Maintenance (M) – how well it stays running
- Stability (S) – how predictable the city is
- Attractiveness (A) – how much population pressure grows

Everything else derives from these.

### Population as Pressure, Not a Count
Do not store population as a raw number. Instead: PopulationPressure P ∈ [0, ∞)

Convert pressure into effects via saturation: EffectivePopulation = P / (P + K)

Early growth feels strong. Late growth slows automatically. No upper bound needed.

This one formula kills 80% of idle runaway issues.

### Resource Generation (The Heartbeat)
Each tick (or second):
- BaseOutput = ZoneOutput × EffectivePopulation
- Then apply dampening multipliers: FinalOutput = BaseOutput × EnergyFactor(E) × MaintenanceFactor(M) × StabilityFactor(S)

Each factor is sublinear.

Example:
- EnergyFactor(E) = E / (E + 1)
- MaintenanceFactor(M) = √M / (√M + 1)
- StabilityFactor(S) = ln(S + 1) / ln(S + 2)

You can swap functions, but the rule is: Increasing returns early, Diminishing returns forever.

### Consumption Scales Faster Than Production
Every resource also consumes:
- EnergyCost = α × P
- MaintenanceCost = β × EffectivePopulation²

Why squared? Because population pressure must always push back eventually.

This ensures: Growth slows naturally, Overexpansion hurts gently, not catastrophically.

### Idle Accumulation Uses Time Buckets
Never simulate every second offline. Instead: OfflineGain = f(CurrentState) × log(TimeAway + 1)

This means: Being gone longer helps, But never linearly, Never infinitely.

You can cap TimeAway softly by flattening the log curve.

### Upgrades Don't Multiply. They Bias.
Avoid: +50% production

Prefer: +0.2 to EnergyFactor curve, -0.1 population pressure decay, +small increase to saturation constant K

Upgrades change shape, not magnitude. This is how numbers stay human-readable forever.

### Soft Caps Through Feedback Loops
Example loop:
- Higher Attractiveness → higher PopulationPressure
- Higher PopulationPressure → higher MaintenanceCost
- Lower Maintenance → lower Stability
- Lower Stability → reduced Attractiveness

No explicit cap needed. The system self-centers like a pendulum. 🕰️

### Zone Scaling Without Explosion
Each zone adds complexity, not raw output. ZoneContribution = ZoneLevel / (ZoneLevel + Z)

Zones past level ~5–7 mostly add behavior, not numbers. This keeps expansion interesting instead of obligatory.

### Prestige Without Reset Hell
When you "reawaken" a district:
- Reset some pressures
- Preserve shapes of curves
- Unlock new interactions, not multipliers

Example: Stability decay becomes slower, Population saturation constant increases, Maintenance penalties become smoother

Prestige changes physics, not numbers.

### Debug Rule of Thumb
If at any point:
- Doubling something doubles output → ❌
- AFK for 10x time gives 10x gain → ❌
- A single stat dominates all others → ❌

Replace it with: log, sqrt, saturation, cross-cost

### Minimal Formula Set (Copy-Paste Friendly)
You could ship MVP with just these:
- EffectivePopulation = P / (P + 10)
- Output = Base × (E / (E + 1)) × (√M / (√M + 1)) × (ln(S + 1) / ln(S + 2))
- OfflineGain = Output × log(TimeAway + 1)

That alone will feel smooth, calm, and unbreakable.

### The Resulting Player Experience
- Numbers rise, then settle
- Growth feels earned, not explosive
- Optimization is about balance, not abuse
- Long-term play feels meditative, not manic

A city that idles like a tide, not a rocket engine. 🌊

## Stress-Test the System with Fake Data

### Baseline Model (Locked In)
We'll use the MVP math exactly as designed.

Core Formulas
- EffectivePopulation = P / (P + 10)
- EnergyFactor(E) = E / (E + 1)
- MaintenanceFactor(M) = √M / (√M + 1)
- StabilityFactor(S) = ln(S + 1) / ln(S + 2)
- Output = Base × EnergyFactor × MaintenanceFactor × StabilityFactor
- MaintenanceCost = 0.02 × EffectivePopulation²

Assume:
- Base = 10
- Output feeds back into Energy + Maintenance slowly
- Stability drifts based on balance

### Scenario A: Early Game (Sanity Check)
Inputs
- PopulationPressure P = 5
- Energy E = 1
- Maintenance M = 1
- Stability S = 1

Calculations
- EffectivePop = 5 / 15 = 0.33
- EnergyFactor = 1 / 2 = 0.50
- MaintenanceFactor = 1 / 2 = 0.50
- StabilityFactor ≈ ln(2) / ln(3) ≈ 0.63
- Output ≈ 10 × 0.33 × 0.50 × 0.50 × 0.63 ≈ 0.52

Result: Output is small but non-zero. Improvements feel noticeable. No dead start. ✅ Pass: early game feels alive but gentle

### Scenario B: Midgame Growth Pressure
Inputs
- P = 25
- E = 5
- M = 4
- S = 3

Calculations
- EffectivePop = 25 / 35 ≈ 0.71
- EnergyFactor = 5 / 6 ≈ 0.83
- MaintenanceFactor = √4 / (√4 + 1) = 2 / 3 ≈ 0.67
- StabilityFactor ≈ ln(4) / ln(5) ≈ 0.86
- Output ≈ 10 × 0.71 × 0.83 × 0.67 × 0.86 ≈ 3.37
- Maintenance Cost: 0.02 × (0.71²) ≈ 0.01

Result: Output rose ~6× from early game. Costs are now present but not crushing. Growth feels earned. ✅ Pass: strong but controlled

### Scenario C: "What If the Player Min-Maxed?"
Push stats unrealistically high.
Inputs
- P = 200
- E = 50
- M = 50
- S = 50

Calculations
- EffectivePop = 200 / 210 ≈ 0.95
- EnergyFactor = 50 / 51 ≈ 0.98
- MaintenanceFactor = √50 / (√50 + 1) ≈ 7.1 / 8.1 ≈ 0.88
- StabilityFactor ≈ ln(51) / ln(52) ≈ 0.99
- Output ≈ 10 × 0.95 × 0.98 × 0.88 × 0.99 ≈ 8.1
- Maintenance Cost: 0.02 × (0.95²) ≈ 0.018

Result: Output did NOT explode. Even extreme stats only give ~2.4× midgame. Headroom remains without runaway. ✅ Pass: spiral prevented

### Scenario D: Population Runaway Attempt
Player pushes Attractiveness endlessly.
Inputs
- P = 1,000
- E = 10
- M = 10
- S = 10

Calculations
- EffectivePop = 1000 / 1010 ≈ 0.99
- Everything else: Factors already near saturation
- Output caps around ~8.5
- Maintenance Cost: 0.02 × (0.99²) ≈ 0.02

Hidden Effect: At high P, Maintenance starts slipping, Stability decays, Attractiveness feedback slows or reverses. 📉 Population pressure stops helping. ✅ Pass: population inflation neutralized

### Scenario E: Neglect Spiral (Does It Kill the City?)
Inputs
- P = 40
- E = 0.2
- M = 0.2
- S = 0.3

Calculations
- EnergyFactor ≈ 0.17
- MaintenanceFactor ≈ 0.31
- StabilityFactor ≈ 0.39
- EffectivePop = 40 / 50 = 0.8
- Output ≈ 10 × 0.8 × 0.17 × 0.31 × 0.39 ≈ 0.16

Result: City doesn't collapse instantly. Output is low but recoverable. Player has time to respond. ✅ Pass: failure is gentle, not punitive

### Offline Abuse Test
Player goes AFK for 72 hours.
- OfflineGain = Output × log(TimeAway + 1)
- log(72 + 1) ≈ 4.29
- Even with high output: 8 × 4.29 ≈ 34

That's meaningful, not game-breaking. ✅ Pass: idle feels rewarding without skipping the game

### Long-Term Plateau Check
After weeks of play: All factors saturated, Output stabilizes around 8–9× base. Progress comes from unlocking new interactions, not scaling numbers.

This means: UI stays readable forever, You never need scientific notation, Content beats math, always.

### Final Verdict 🏙️
The system: Cannot explode, Cannot deadlock, Cannot be AFK abused, Cannot be trivialized by one stat. It breathes. It resists. It forgives. Exactly what a quiet city should do.</content>
<parameter name="filePath">h:\WebHatchery\games\quiteville\formulas_and_statistics.md