# lotka-beats

[![crates.io](https://img.shields.io/crates/v/lotka-beats.svg)](https://crates.io/crates/lotka-beats)
[![docs.rs](https://docs.rs/lotka-beats/badge.svg)](https://docs.rs/lotka-beats)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Generative music via Lotka-Volterra population dynamics.**

Musical genres compete like species in an ecosystem. Jazz "hunts" classical.
Electronic competes with folk. The Lotka-Volterra equations model this
predator-prey dynamics, and the equilibrium points are genres that don't exist
yet — the math invents fusion genres.

`lotka-beats` simulates these ecosystems and translates population trajectories
into MIDI: population → note density, dominant species → scale selection,
equilibria → new genre discovery.

## Features

- **RK4 integration** — 4th-order Runge-Kutta solver for accurate population
  dynamics with oscillation and competitive exclusion
- **Musical species** — `MusicalSpecies` with scales, rhythms, timbre profiles,
  tempo ranges, and interaction matrices
- **Prebuilt genres** — jazz, classical, electronic, folk, blues, ambient, rock,
  hip-hop, plus classic predator-prey and competitive-three presets
- **Ecosystem simulation** — `MusicEcosystem` manages multi-species interaction
  with time-stepping, history tracking, and Shannon diversity measurement
- **Equilibrium analysis** — find fixed points of the dynamics and classify
  their stability (stable, unstable, saddle, neutral)
- **MIDI export** — convert population trajectories to `MidiSequence` with
  population-to-chord mapping, note/velocity/channel assignment
- **Timbre profiles** — brightness, warmth, attack, decay, complexity for
  timbral evolution driven by population state

## Quick Start

```rust
use lotka_beats::{MusicEcosystem, genre};

let mut eco = MusicEcosystem::new(0.1).unwrap();

// Add a classic predator-prey pair
for sp in genre::classic_predator_prey() {
    eco.add_species(sp);
}

// Run the simulation
eco.run(100).unwrap();

// Check ecosystem state
println!("Time: {:.1}", eco.time());
println!("Populations: {:?}", eco.populations());
println!("Diversity: {:.3}", MusicEcosystem::shannon_diversity(&eco.populations()));

// Export to MIDI
let seq = lotka_beats::midi::ecosystem_to_midi(&eco).unwrap();
println!("Generated {} MIDI events", seq.events.len());
```

## Defining Custom Species

```rust
use lotka_beats::{MusicalSpecies, TimbreProfile, MusicEcosystem};

let my_genre = MusicalSpecies::new("vapor-wave", 1.0)
    .growth_rate(0.3)
    .death_rate(0.1)
    .scale(vec![0, 2, 3, 5, 7, 8, 10])
    .rhythm(vec![0.0, 0.25, 0.5, 0.75])
    .tempo_range((90.0, 110.0))
    .interaction(vec![-0.1, 0.2]);

let mut eco = MusicEcosystem::new(0.05).unwrap();
eco.add_species(my_genre);
```

## Module Overview

| Module | Description |
|---|---|
| `species` | `MusicalSpecies`, `TimbreProfile` — species definition |
| `dynamics` | `LotkaVolterra` — RK4 solver for population dynamics |
| `ecosystem` | `MusicEcosystem`, `EcosystemSnapshot` — simulation manager |
| `equilibrium` | `EquilibriumGenre`, `Stability` — fixed-point analysis |
| `genre` | Prebuilt genre presets (jazz, classical, electronic, …) |
| `midi` | `MidiEvent`, `MidiSequence` — MIDI export pipeline |
| `error` | Error types |

## Links

- [Documentation](https://docs.rs/lotka-beats)
- [Repository](https://github.com/nightshift-crates/lotka-beats)
- [Crates.io](https://crates.io/crates/lotka-beats)

## License

MIT
