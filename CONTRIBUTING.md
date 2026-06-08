# Contributing to lotka-beats

Thank you for your interest in contributing!

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Running Examples

```bash
cargo run --example basic
```

## Code Quality

Before submitting a PR:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes with clear commit messages
4. Ensure CI passes (fmt, clippy, test)
5. Open a pull request against `main`

## Adding New Genre Species

When adding a new genre, provide:
- A unique scale (pitch classes 0–11)
- A characteristic rhythm pattern (beat fractions)
- A tempo range (BPM)
- A `TimbreProfile` (brightness, warmth, complexity, dynamics)
- Appropriate interaction coefficients for the ecosystem

## Mathematical Background

The generalized Lotka-Volterra model:

```
dN_i/dt = N_i × (r_i + Σ_j a_ij × N_j)
```

- `r_i` = intrinsic growth rate (birth − death)
- `a_ij` = interaction coefficient (positive = mutualism, negative = competition)
- Integration uses 4th-order Runge-Kutta (RK4)
