# Contributing to lotka-beats

Thank you for your interest in ecological generative music!

## Getting Started

```bash
git clone https://github.com/SuperInstance/lotka-beats.git
cd lotka-beats
cargo test
```

## Architecture Decisions

### Why Lotka-Volterra for generative music?

The Lotka-Volterra equations produce naturalistic dynamics: oscillation, competition, coexistence, and chaos. These map beautifully to musical parameters — population = loudness/prominence, species = genre/tradition, interaction = influence. The key insight is that **equilibrium points are new genres** — the math invents fusion music.

### Why RK4 integration?

4th-order Runge-Kutta is the sweet spot for ODE integration: accurate enough for musical purposes (O(dt⁵) error per step), simple to implement, and doesn't require computing Jacobians. For stiff systems, you'd want an implicit method, but musical Lotka-Volterra systems are typically non-stiff with small n (2-8 species).

### Why Gaussian elimination for equilibria?

We solve A·N* = −r to find non-trivial fixed points. For the typical case of 2-8 species, Gaussian elimination with partial pivoting is perfectly adequate (O(n³) = trivial for n < 10).

### Why Shannon entropy for diversity?

Shannon entropy H = −Σ pᵢ ln(pᵢ) is the standard diversity metric in ecology. It captures both richness (how many species) and evenness (how balanced). H = 0 means one species dominates; H = ln(n) means perfect balance.

## How to Add a New Genre

1. Add a function in `src/genre.rs`:
```rust
pub fn my_genre() -> MusicalSpecies {
    MusicalSpecies::new("MyGenre", 1.0)
        .growth_rate(0.1)
        .death_rate(0.03)
        .scale(vec![0, 2, 4, 5, 7, 9, 11]) // major scale
        .rhythm(vec![0.0, 0.25, 0.5, 0.75])
        .tempo_range((100.0, 140.0))
        .timbre(TimbreProfile {
            brightness: 0.5,
            warmth: 0.5,
            complexity: 0.5,
            dynamics: 0.5,
        })
}
```

2. Add interaction coefficients when building ecosystems
3. Write tests verifying the species produces MIDI events

## How to Add a New Analysis

The equilibrium module can be extended with:
- **Bifurcation analysis**: vary a parameter and track fixed point changes
- **Limit cycle detection**: search for periodic orbits
- **Strange attractors**: Lyapunov exponent computation for chaotic systems

## Testing

```bash
cargo test                    # All tests
cargo test test_rk4           # Dynamics tests
cargo test test_ecosystem     # Ecosystem tests
cargo test test_equilibrium   # Equilibrium tests
cargo test test_midi          # MIDI tests
```

Key test patterns:
- RK4 accuracy: compare with analytical solutions
- Population positivity: verify populations stay positive in short runs
- Conservation-like: verify total population remains bounded
- MIDI validity: all note/velocity/channel values in range

## Code Style

- `cargo fmt` — no debate
- `cargo clippy` — warnings are errors in CI
- Doc comments on all `pub` items
- Builder pattern for species configuration

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` new features (new genres, new analysis)
- `fix:` bug fixes
- `docs:` documentation changes

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
