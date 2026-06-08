//! # lotka-beats
//!
//! Generative music via Lotka-Volterra population dynamics.
//!
//! Musical traditions (genres, scales, rhythms) compete like species in an ecosystem.
//! The Lotka-Volterra equations model predator-prey dynamics:
//! - Jazz "hunts" classical. Electronic "competes" with folk.
//! - Equilibrium points are genres that don't exist yet — the math invents fusion genres.
//!
//! # Quick start
//!
//! ```
//! use lotka_beats::{MusicEcosystem, genre};
//!
//! let mut eco = MusicEcosystem::new(0.1).unwrap();
//! let species = genre::classic_predator_prey();
//! for sp in species {
//!     eco.add_species(sp);
//! }
//! eco.run(100).unwrap();
//! println!("After 100 steps: {:?}", eco.populations());
//! ```

pub mod dynamics;
pub mod ecosystem;
pub mod equilibrium;
pub mod error;
pub mod genre;
pub mod midi;
pub mod species;

pub use dynamics::LotkaVolterra;
pub use ecosystem::{EcosystemSnapshot, MusicEcosystem};
pub use equilibrium::{EquilibriumGenre, Stability};
pub use error::Error;
pub use midi::{MidiEvent, MidiSequence};
pub use species::{MusicalSpecies, TimbreProfile};

#[cfg(test)]
mod tests {
    use super::*;

    // ── Species tests ──

    #[test]
    fn species_builder_works() {
        let sp = MusicalSpecies::new("Test", 1.0)
            .growth_rate(0.2)
            .death_rate(0.1)
            .scale(vec![0, 2, 4])
            .rhythm(vec![0.0, 0.5])
            .tempo_range((80.0, 120.0));

        assert_eq!(sp.name, "Test");
        assert!((sp.growth_rate - 0.2).abs() < 1e-10);
        assert_eq!(sp.scale, vec![0, 2, 4]);
    }

    #[test]
    fn default_timbre_is_balanced() {
        let t = TimbreProfile::default();
        assert!((t.brightness - 0.5).abs() < 1e-10);
        assert!((t.warmth - 0.5).abs() < 1e-10);
    }

    // ── Dynamics / RK4 tests ──

    #[test]
    fn rk4_single_species_growth() {
        // Exponential growth: dN/dt = r * N
        let sp = MusicalSpecies::new("A", 1.0)
            .growth_rate(0.1)
            .death_rate(0.0)
            .interaction(vec![0.0]);

        let solver = LotkaVolterra::from_species(&[sp]).unwrap();
        let dt = 0.1;
        let result = solver.step(&[1.0], dt).unwrap();
        let _expected = 1.0_f64.exp_m1() * 0.0; // just check it grew
        assert!(result[0] > 1.0, "population should grow: got {}", result[0]);
        assert!(
            (result[0] - 1.01005).abs() < 0.001,
            "RK4 accuracy: expected ~1.01005, got {}",
            result[0]
        );
    }

    #[test]
    fn rk4_predator_prey_oscillates() {
        let species = genre::classic_predator_prey();
        let solver = LotkaVolterra::from_species(&species).unwrap();

        let mut pops = vec![1.0, 0.5];
        let mut max_pop = 0.0_f64;
        let mut min_pop = f64::MAX;

        for _ in 0..500 {
            pops = solver.step(&pops, 0.01).unwrap();
            max_pop = max_pop.max(pops[0]).max(pops[1]);
            min_pop = min_pop.min(pops[0]).min(pops[1]);
        }

        // Should oscillate (max >> min)
        assert!(max_pop > min_pop * 1.5, "predator-prey should oscillate");
    }

    #[test]
    fn rk4_rejects_negative_dt() {
        let sp = MusicalSpecies::new("A", 1.0).interaction(vec![0.0]);
        let solver = LotkaVolterra::from_species(&[sp]).unwrap();
        assert!(solver.step(&[1.0], -0.1).is_err());
    }

    #[test]
    fn dynamics_rejects_empty_ecosystem() {
        let result = LotkaVolterra::from_species(&[]);
        assert!(result.is_err());
    }

    // ── Ecosystem tests ──

    #[test]
    fn ecosystem_step_advances_time() {
        let mut eco = MusicEcosystem::new(0.1).unwrap();
        let mut sp = genre::jazz();
        sp.interaction = vec![0.0];
        eco.add_species(sp);
        eco.step().unwrap();
        assert!((eco.time - 0.1).abs() < 1e-10);
    }

    #[test]
    fn ecosystem_run_multiple_steps() {
        let mut eco = MusicEcosystem::new(0.05).unwrap();
        for sp in genre::classic_predator_prey() {
            eco.add_species(sp);
        }
        eco.run(200).unwrap();
        assert!((eco.time - 10.0).abs() < 1e-10);
        assert_eq!(eco.history.len(), 200);
    }

    #[test]
    fn shannon_diversity_single_species_is_zero() {
        let d = MusicEcosystem::shannon_diversity(&[1.0]);
        assert!((d).abs() < 1e-10, "single species => zero entropy");
    }

    #[test]
    fn shannon_diversity_balanced_is_high() {
        let d = MusicEcosystem::shannon_diversity(&[0.5, 0.5]);
        // ln(2) ≈ 0.693
        assert!((d - 2.0_f64.ln()).abs() < 1e-10);
    }

    #[test]
    fn dominant_index_picks_largest() {
        assert_eq!(MusicEcosystem::dominant_index(&[0.2, 0.7, 0.1]), 1);
    }

    #[test]
    fn ecosystem_rejects_zero_dt() {
        assert!(MusicEcosystem::new(0.0).is_err());
    }

    #[test]
    fn ecosystem_step_empty_is_error() {
        let mut eco = MusicEcosystem::new(0.1).unwrap();
        assert!(eco.step().is_err());
    }

    #[test]
    fn total_population_accessible() {
        let mut eco = MusicEcosystem::new(0.1).unwrap();
        eco.add_species(MusicalSpecies::new("A", 1.0).interaction(vec![0.0]));
        let total = eco.total_population();
        assert!((total - 1.0).abs() < 1e-10);
    }

    // ── Equilibrium tests ──

    #[test]
    fn trivial_fixed_point_is_zeros() {
        let fp = equilibrium::trivial_fixed_point(3);
        assert_eq!(fp, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn find_fixed_points_returns_trivial() {
        let mut eco = MusicEcosystem::new(0.1).unwrap();
        for sp in genre::classic_predator_prey() {
            eco.add_species(sp);
        }
        let fps = equilibrium::find_fixed_points(&eco).unwrap();
        assert!(!fps.is_empty(), "should at least have trivial fixed point");
        assert_eq!(fps[0].populations, vec![0.0, 0.0]);
    }

    #[test]
    fn find_fixed_points_empty_is_error() {
        let eco = MusicEcosystem::new(0.1).unwrap();
        assert!(equilibrium::find_fixed_points(&eco).is_err());
    }

    #[test]
    fn stability_enum_values() {
        assert_ne!(Stability::Stable, Stability::Unstable);
        assert_ne!(Stability::Saddle, Stability::Neutral);
    }

    // ── Genre tests ──

    #[test]
    fn predefined_genres_have_scales() {
        let genres: Vec<MusicalSpecies> = vec![
            genre::jazz(),
            genre::classical(),
            genre::electronic(),
            genre::folk(),
            genre::blues(),
            genre::ambient(),
            genre::rock(),
            genre::hip_hop(),
        ];
        for g in &genres {
            assert!(!g.scale.is_empty(), "{} has no scale", g.name);
            assert!(!g.rhythm.is_empty(), "{} has no rhythm", g.name);
            assert!(g.tempo_range.0 > 0.0, "{} has invalid tempo", g.name);
        }
    }

    #[test]
    fn predator_prey_pair_has_correct_size() {
        let pp = genre::classic_predator_prey();
        assert_eq!(pp.len(), 2);
        assert_eq!(pp[0].interaction.len(), 2);
    }

    #[test]
    fn competitive_three_has_correct_size() {
        let ct = genre::competitive_three();
        assert_eq!(ct.len(), 3);
        for sp in &ct {
            assert_eq!(sp.interaction.len(), 3);
        }
    }

    // ── MIDI tests ──

    #[test]
    fn midi_empty_ecosystem_is_error() {
        let eco = MusicEcosystem::new(0.1).unwrap();
        assert!(midi::ecosystem_to_midi(&eco).is_err());
    }

    #[test]
    fn midi_generates_events() {
        let mut eco = MusicEcosystem::new(0.1).unwrap();
        for sp in genre::classic_predator_prey() {
            eco.add_species(sp);
        }
        eco.run(4).unwrap();
        let seq = midi::ecosystem_to_midi(&eco).unwrap();
        assert!(!seq.events.is_empty(), "should generate MIDI events");
        assert!(seq.tempo_bpm > 0.0, "tempo should be positive");
    }

    #[test]
    fn midi_events_have_valid_ranges() {
        let mut eco = MusicEcosystem::new(0.1).unwrap();
        for sp in genre::classic_predator_prey() {
            eco.add_species(sp);
        }
        eco.run(4).unwrap();
        let seq = midi::ecosystem_to_midi(&eco).unwrap();
        for e in &seq.events {
            assert!(e.note <= 127, "note out of range: {}", e.note);
            assert!(e.velocity <= 127, "velocity out of range: {}", e.velocity);
            assert!(e.channel <= 15, "channel out of range: {}", e.channel);
        }
    }

    #[test]
    fn midi_text_export_works() {
        let mut eco = MusicEcosystem::new(0.1).unwrap();
        for sp in genre::classic_predator_prey() {
            eco.add_species(sp);
        }
        eco.run(2).unwrap();
        let seq = midi::ecosystem_to_midi(&eco).unwrap();
        let text = midi::midi_to_text(&seq);
        assert!(text.contains("Sequence:"));
        assert!(text.contains("Tempo:"));
    }

    #[test]
    fn population_to_chord_works() {
        let species = genre::classic_predator_prey();
        let pops = vec![1.0, 0.5];
        let chord = midi::population_to_chord(&species, &pops);
        assert!(!chord.is_empty());
        for &note in &chord {
            assert!(note <= 127);
        }
    }

    #[test]
    fn population_to_chord_empty_is_empty() {
        let chord = midi::population_to_chord(&[], &[]);
        assert!(chord.is_empty());
    }

    // ── Snapshot / history tests ──

    #[test]
    fn snapshot_captures_state() {
        let mut eco = MusicEcosystem::new(0.1).unwrap();
        let mut sp = genre::jazz();
        sp.interaction = vec![0.0];
        eco.add_species(sp);
        let snap = eco.snapshot();
        assert!((snap.time - 0.0).abs() < 1e-10);
        assert_eq!(snap.populations.len(), 1);
    }

    // ── Conservation-ish / sanity tests ──

    #[test]
    fn populations_remain_positive_short_run() {
        let mut eco = MusicEcosystem::new(0.01).unwrap();
        for sp in genre::classic_predator_prey() {
            eco.add_species(sp);
        }
        eco.run(1000).unwrap();
        for sp in &eco.species {
            assert!(sp.population > 0.0, "population of {} went negative: {}", sp.name, sp.population);
        }
    }

    #[test]
    fn competitive_three_remains_bounded() {
        let mut eco = MusicEcosystem::new(0.01).unwrap();
        for sp in genre::competitive_three() {
            eco.add_species(sp);
        }
        eco.run(500).unwrap();
        for sp in &eco.species {
            assert!(
                sp.population < 1e6,
                "population of {} exploded: {}",
                sp.name,
                sp.population
            );
        }
    }

    #[test]
    fn make_interaction_helper() {
        let v = genre::make_interaction(3, &[(0, -0.1), (2, 0.05)]);
        assert_eq!(v, vec![-0.1, 0.0, 0.05]);
    }

    #[test]
    fn error_display_works() {
        let e = Error::EmptyEcosystem;
        assert!(e.to_string().contains("no species"));
    }
}
