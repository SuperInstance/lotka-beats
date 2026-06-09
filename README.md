# lotka-beats

**Generative music via Lotka-Volterra population dynamics — genres compete like species, and new fusion genres emerge at equilibrium.**

## The Problem

Generative music systems either sound random (white noise with envelope) or static (looping pre-composed patterns). What's missing is *ecology*: the idea that musical traditions compete, cooperate, and evolve like biological species. Jazz "preys on" blues. Electronic "competes with" rock. At equilibrium, you get fusion genres that don't exist yet — the math *invents* them.

## The Key Insight

The **Lotka-Volterra equations** model predator-prey dynamics in ecology:

```
dN₁/dt = N₁(r₁ + a₁₁N₁ + a₁₂N₂)
dN₂/dt = N₂(r₂ + a₂₁N₁ + a₂₂N₂)
```

Each species has a growth rate, death rate, and interaction coefficients with every other species. The dynamics produce:
- **Oscillations**: predator-prey cycles (jazz rises, blues falls, jazz falls, blues rises...)
- **Competition**: mutually harmful interactions drive one species to extinction
- **Coexistence**: negative self-interaction + positive cross-interaction → stable equilibrium
- **Chaos**: 3+ species with the right interaction matrix → complex, non-repeating dynamics

The "aha moment" is that **equilibrium points are new genres**. When you solve for dN/dt = 0, the population ratios define a fusion genre — say, 60% Jazz, 30% Blues, 10% Ambient. The stability of that equilibrium tells you whether it's a sustainable genre or a transient fad.

This crate implements:
- Generalized n-species Lotka-Volterra with RK4 integration
- Musical species with scales, rhythms, tempo ranges, and timbre profiles
- Equilibrium analysis via Jacobian eigenvalues
- Auto-generated genre names for fusion equilibria
- MIDI output that maps population dynamics to notes
- 8 pre-defined genre species (Jazz, Classical, Electronic, Folk, Blues, Ambient, Rock, HipHop)

## Architecture

```
                    ┌─────────────────────────────┐
                    │      MusicEcosystem          │
                    │ (species list, time, dt,     │
                    │  history of snapshots)        │
                    └──────────┬──────────────────┘
                               │
              ┌────────────────┼──────────────────┐
              │                │                  │
    ┌─────────▼──────┐ ┌──────▼──────┐ ┌────────▼───────┐
    │ MusicalSpecies │ │LotkaVolterra│ │ Equilibrium    │
    │ (name, scale,  │ │ (RK4 solver │ │ Analysis       │
    │  rhythm, timbre│ │  for dN/dt) │ │ (fixed points, │
    │  growth, death,│ │             │ │  stability,    │
    │  interaction)  │ │             │ │  genre names)  │
    └────────────────┘ └─────────────┘ └────────────────┘
              │                │                  │
              └────────────────┼──────────────────┘
                               │
                    ┌──────────▼──────────────────┐
                    │       MIDI Output            │
                    │ (ecosystem → MidiSequence,   │
                    │  population → chord,         │
                    │  text export)                │
                    └──────────────────────────────┘
```

### Module Overview

| Module | Purpose |
|--------|---------|
| `species` | MusicalSpecies with scales, rhythms, timbre, interaction coefficients |
| `dynamics` | Generalized Lotka-Volterra RK4 solver |
| `ecosystem` | MusicEcosystem — the simulation container |
| `equilibrium` | Fixed point analysis, stability classification, genre naming |
| `genre` | Pre-defined species (Jazz, Classical, Electronic, etc.) |
| `midi` | Ecosystem → MIDI sequence conversion |
| `error` | Error types |

## The Math: Generalized Lotka-Volterra

For n species with populations N₁, ..., Nₙ:

```
dNᵢ/dt = Nᵢ × (rᵢ + Σⱼ aᵢⱼ × Nⱼ)
```

Where:
- `rᵢ = growth_rateᵢ - death_rateᵢ` (intrinsic growth rate)
- `aᵢⱼ` = interaction coefficient (effect of species j on species i)
  - Positive: species j *benefits* species i (mutualism, commensalism)
  - Negative: species j *harms* species i (competition, predation)
  - Self-interaction `aᵢᵢ` is usually negative (self-limiting)

### RK4 Integration

We use 4th-order Runge-Kutta for accuracy:

```
k₁ = f(N)
k₂ = f(N + dt/2 × k₁)
k₃ = f(N + dt/2 × k₂)
k₄ = f(N + dt × k₃)
N(t+dt) = N(t) + dt/6 × (k₁ + 2k₂ + 2k₃ + k₄)
```

This is 4th-order accurate: the error is O(dt⁵) per step.

### Equilibrium Analysis

Fixed points are where dN/dt = 0. For non-trivial equilibria (N ≠ 0):

```
rᵢ + Σⱼ aᵢⱼ × Nⱼ* = 0
```

This is a linear system A × N* = −r, solved via Gaussian elimination.

Stability is analyzed via the Jacobian at the fixed point:

```
Jᵢⱼ = aᵢⱼ × Nᵢ*
```

- All eigenvalues with negative real parts → **Stable** (sustainable genre)
- Any positive real part → **Unstable** (transient fad)
- Mixed → **Saddle** (stable in some directions, unstable in others)

## Quick Start

```rust
use lotka_beats::{MusicEcosystem, genre};

// Create a 2-species predator-prey ecosystem
let mut eco = MusicEcosystem::new(0.1).unwrap();
for sp in genre::classic_predator_prey() {
    eco.add_species(sp);
}

// Run for 100 time steps
eco.run(100).unwrap();

println!("Populations: {:?}", eco.populations());
println!("Total: {:.2}", eco.total_population());
```

## Pre-defined Genre Species

```rust
use lotka_beats::genre;

// 8 pre-defined genres, each with scales, rhythms, timbre, and tempo
let jazz = genre::jazz();
let classical = genre::classical();
let electronic = genre::electronic();
let folk = genre::folk();
let blues = genre::blues();
let ambient = genre::ambient();
let rock = genre::rock();
let hip_hop = genre::hip_hop();

println!("Jazz scale: {:?}", jazz.scale);         // [0,2,3,5,7,9,10]
println!("Blues rhythm: {:?}", blues.rhythm);     // [0,0.33,0.67]
println!("Electronic timbre: brightness={}", electronic.timbre.brightness); // 0.9
```

## Building Custom Ecosystems

```rust
use lotka_beats::{MusicEcosystem, MusicalSpecies, TimbreProfile};

let mut eco = MusicEcosystem::new(0.05).unwrap();

// Custom species with interaction coefficients
let mut dubstep = MusicalSpecies::new("Dubstep", 1.0)
    .growth_rate(0.15)
    .death_rate(0.03)
    .scale(vec![0, 3, 5, 6, 7, 10])
    .rhythm(vec![0.0, 0.25, 0.5, 0.75])
    .tempo_range((140.0, 150.0))
    .timbre(TimbreProfile {
        brightness: 0.95,
        warmth: 0.1,
        complexity: 0.7,
        dynamics: 0.9,
    });
dubstep.interaction = vec![-0.1, 0.02]; // self-limits, benefits from ambient

let mut ambient = MusicalSpecies::new("Ambient", 0.5)
    .growth_rate(0.03)
    .death_rate(0.01)
    .scale(vec![0, 2, 4, 7, 9])
    .rhythm(vec![0.0, 0.5])
    .tempo_range((60.0, 90.0));
ambient.interaction = vec![0.01, -0.05]; // benefits from dubstep, self-limits

eco.add_species(dubstep);
eco.add_species(ambient);

eco.run(200).unwrap();
```

## Equilibrium Analysis

```rust
use lotka_beats::{MusicEcosystem, genre, equilibrium};

let mut eco = MusicEcosystem::new(0.1).unwrap();
for sp in genre::classic_predator_prey() {
    eco.add_species(sp);
}

// Find equilibrium genres
let fixed_points = equilibrium::find_fixed_points(&eco).unwrap();
for fp in &fixed_points {
    println!("Genre: {} ({:?})", fp.name.as_deref().unwrap_or("?"), fp.stability);
    println!("  Populations: {:?}", fp.populations);
}
```

## MIDI Output

```rust
use lotka_beats::{MusicEcosystem, genre, midi};

let mut eco = MusicEcosystem::new(0.1).unwrap();
for sp in genre::classic_predator_prey() {
    eco.add_species(sp);
}
eco.run(10).unwrap();

// Convert to MIDI sequence
let seq = midi::ecosystem_to_midi(&eco).unwrap();
println!("Events: {}", seq.events.len());
println!("Tempo: {:.1} BPM", seq.tempo_bpm);

// Text export
let text = midi::midi_to_text(&seq);
println!("{}", text);

// Population → chord
let chord = midi::population_to_chord(&eco.species, &eco.populations());
println!("Chord: {:?}", chord);
```

## Shannon Diversity

The ecosystem computes Shannon entropy as a diversity metric:

```
H = −Σ (Nᵢ/N_total) × ln(Nᵢ/N_total)
```

- H = 0: one species dominates (monoculture)
- H = ln(n): all species equally present (maximum diversity)

```rust
use lotka_beats::MusicEcosystem;

let diversity = MusicEcosystem::shannon_diversity(&[0.5, 0.3, 0.2]);
println!("Diversity: {:.3} (max for 3 species: {:.3})", diversity, 3.0_f64.ln());
```

## Performance

- **RK4 step**: O(n²) where n = number of species
- **Equilibrium solving**: O(n³) via Gaussian elimination
- **MIDI generation**: O(species × history_steps)
- Typical: 2-3 species, 1000 steps → <1ms

## Comparison

| Feature | lotka-beats | Conventional generative music | Markov chain music |
|---------|------------|------------------------------|-------------------|
| Model | Lotka-Volterra ecology | Random/stochastic | Transition probabilities |
| Emergence | ✅ Fusion genres at equilibrium | ❌ Pre-defined | Partial |
| Dynamics | ✅ Predator-prey oscillation | ❌ | ❌ |
| Genre theory | ✅ Equilibrium = new genre | ❌ | ❌ |
| MIDI output | ✅ Built-in | Varies | Varies |
| Stability analysis | ✅ Jacobian eigenvalues | ❌ | ❌ |

## SuperInstance Ecosystem

`lotka-beats` integrates with:
- `groovemesh-plr` — PLR voice-leading for smooth chord transitions within species
- `tropical-synth` — Tropical timbre mapping for species timbre profiles
- `spreadsheet-engine` — Lotka-Volterra as a simulation cell type
- `noether-guard` — Conservation checking for population dynamics

## License

MIT
