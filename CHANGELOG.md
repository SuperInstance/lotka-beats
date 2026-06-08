# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-08

### Added
- Generalized n-species Lotka-Volterra solver with RK4 integration
- `MusicalSpecies` builder for defining genre-organisms with scales, rhythms, timbre
- `MusicEcosystem` for running multi-species simulations
- Equilibrium analysis: fixed-point finding with stability classification (Stable/Unstable/Saddle/Neutral)
- Auto-generated fusion genre names at equilibrium points
- MIDI output generation from ecosystem dynamics
- Shannon diversity index for population distribution
- Predefined genre species: Jazz, Classical, Electronic, Folk, Blues, Ambient, Rock, HipHop
- Classic predator-prey and competitive three-species presets
