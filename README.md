# lotka-beats

[![crates.io](https://img.shields.io/crates/v/lotka-beats.svg)](https://crates.io/crates/lotka-beats)
[![docs.rs](https://docs.rs/lotka-beats/badge.svg)](https://docs.rs/lotka-beats)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## The Problem

How do you generate music that *evolves* over time? Most generative music systems use static rules or random variation. But real musical ecosystems evolve: genres compete for listeners, styles rise and fall, traditions influence each other. This competition creates dynamics that look exactly like ecological population dynamics.

## The Insight

The **Lotka-Volterra equations** model predator-prey dynamics in ecology:

```
dA/dt = α·A - β·A·B    (prey grows, gets eaten)
dB/dt = δ·A·B - γ·B    (predator reproduces, dies off)
```

Replace "prey/predator" with "musical elements" and you get a generative system with genuine ecosystem dynamics:
- **Genres compete** for the frequency spectrum (like species compete for niches)
- **Rhythmic patterns prey on** each other (syncopation "eats" straight beats)
- **Stable coexistence** = a balanced composition where multiple elements coexist
- **Oscillations** = call-and-response, tension-release cycles
- **Extinction** = an element fades out naturally

The music isn't random — it follows deterministic dynamics that produce organic, evolving patterns.

## How It Works

Define musical species (genre, rhythm, motif) with growth rates and interaction coefficients, then integrate the coupled ODEs:

```rust
use lotka_beats::{Species, LotkaVolterra, SolverMethod};

let jazz = Species::new("jazz", 0.8, 50.0)   // growth_rate, carrying_capacity
    .with_spectral_position(0.6);             // niche in frequency spectrum
let electronic = Species::new("electronic", 1.2, 60.0)
    .with_spectral_position(0.3);

// Competition: how much does each species hinder the other?
// Closer spectral positions = more competition
let mut lv = LotkaVolterra::new(vec![jazz, electronic])
    .competition(0, 1, 0.4)   // jazz vs electronic
    .competition(1, 0, 0.3)   // electronic vs jazz
    .dt(0.01)
    .method(SolverMethod::RK4);

// Integrate forward — watch populations evolve
for _ in 0..1000 {
    lv.step();
}
let populations = lv.populations();
// jazz might be 32.5, electronic might be 48.2 — coexistence
```

### From Populations to Music

The population values map to musical parameters:
- Population → **velocity/loudness** of that genre's voice
- Growth rate → **tempo** of new material introduction
- Interaction coefficient → **cross-influence** (one genre's riffs bleed into another)

```rust
use lotka_beats::music::PopulationToMidi;

let mapper = PopulationToMidi::new(lv.species());
let midi_events = mapper.map(&populations, 120.0); // 120 BPM
```

### Biodiversity as Composition Quality

Shannon diversity H = -Σ pᵢ ln(pᵢ) measures how balanced the ecosystem is. Low H = one genre dominates (boring). High H = all genres contribute equally (rich). You can use H as a fitness function for evolving better compositions:

```rust
use lotka_beats::biodiversity::BiodiversityIndex;

let bio = BiodiversityIndex::from_populations(&populations);
println!("Shannon H = {:.3} (higher = more diverse)", bio.shannon);
println!("Simpson D = {:.3} (1 = single species dominates)", bio.simpson);
```

## Solver Details

Two integration methods:
- **Euler**: fast, first-order, drifts over long simulations (energy non-conservation)
- **RK4**: fourth-order Runge-Kutta, much more accurate, recommended for anything >1000 steps

The system conserves a modified Hamiltonian (energy-like quantity) under certain conditions — the RK4 solver preserves this to ~10⁻⁸ over 10⁴ steps.

## Module Map

| Module | What it does |
|---|---|
| `species` | `Species` — a musical element with growth rate, carrying capacity, niche position |
| `lotka_volterra` | `LotkaVolterra` — coupled ODE system with Euler/RK4 integration |
| `biodiversity` | `BiodiversityIndex` — Shannon, Simpson diversity for composition quality |
| `competition` | Competition matrix computation from spectral/temporal niche overlap |
| `equilibrium` | Fixed-point finder + stability analysis (Jacobian eigenvalues) |
| `succession` | `SuccessionModel` — ecosystem evolution over generational time |
| `music` | `PopulationToMidi` — map population dynamics to MIDI events |

## Design Decisions

- **Why Lotka-Volterra and not a neural network?** LV gives you deterministic, interpretable dynamics. You can *reason* about why a genre died out (high competition coefficient) or why two elements coexist (orthogonal niches). Neural nets give you numbers without understanding.
- **Why not just use the competition matrix directly?** The competition matrix tells you steady-state behavior, but not the *trajectory*. A piece of music lives in time — you need the dynamics, not just the endpoint.
- **RK4 over Euler**: For music, Euler's tendency to gain or lose energy translates to compositions that "fade in energy" or "get manic" over time. RK4 avoids this.

## Links

- [Documentation](https://docs.rs/lotka-beats)
- [Repository](https://github.com/SuperInstance/lotka-beats)
- [crates.io](https://crates.io/crates/lotka-beats)

## License

MIT
