use serde::{Deserialize, Serialize};

/// Timbre characteristics for a musical species.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimbreProfile {
    /// High-frequency energy (0-1).
    pub brightness: f64,
    /// Low-mid warmth (0-1).
    pub warmth: f64,
    /// Harmonic / rhythmic complexity (0-1).
    pub complexity: f64,
    /// Dynamic range tendency (0-1).
    pub dynamics: f64,
}

impl Default for TimbreProfile {
    fn default() -> Self {
        Self {
            brightness: 0.5,
            warmth: 0.5,
            complexity: 0.5,
            dynamics: 0.5,
        }
    }
}

/// A musical tradition modeled as a Lotka-Volterra species.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicalSpecies {
    /// Human-readable name (e.g. "Jazz").
    pub name: String,
    /// Current population (relative abundance).
    pub population: f64,
    /// Intrinsic growth rate (α).
    pub growth_rate: f64,
    /// Death rate (γ).
    pub death_rate: f64,
    /// Interaction coefficients with other species (β_ij).
    /// Positive = benefits from that species; negative = harmed.
    pub interaction: Vec<f64>,
    /// Pitch classes used (MIDI note numbers mod 12).
    pub scale: Vec<u8>,
    /// Rhythmic patterns as beat fractions (e.g. [0.0, 0.5] = quarter + half).
    pub rhythm: Vec<f64>,
    /// (min_bpm, max_bpm) tempo range.
    pub tempo_range: (f64, f64),
    /// Timbre characteristics.
    pub timbre: TimbreProfile,
}

impl MusicalSpecies {
    /// Create a new species with a name and initial population.
    pub fn new(name: impl Into<String>, population: f64) -> Self {
        Self {
            name: name.into(),
            population,
            growth_rate: 0.1,
            death_rate: 0.05,
            interaction: Vec::new(),
            scale: vec![0, 2, 4, 5, 7, 9, 11], // major scale default
            rhythm: vec![0.0, 0.25, 0.5, 0.75],
            tempo_range: (100.0, 140.0),
            timbre: TimbreProfile::default(),
        }
    }

    /// Builder: set growth rate.
    pub fn growth_rate(mut self, rate: f64) -> Self {
        self.growth_rate = rate;
        self
    }

    /// Builder: set death rate.
    pub fn death_rate(mut self, rate: f64) -> Self {
        self.death_rate = rate;
        self
    }

    /// Builder: set interaction coefficients.
    pub fn interaction(mut self, coeffs: Vec<f64>) -> Self {
        self.interaction = coeffs;
        self
    }

    /// Builder: set scale (pitch classes 0-11).
    pub fn scale(mut self, notes: Vec<u8>) -> Self {
        self.scale = notes;
        self
    }

    /// Builder: set rhythm patterns (beat fractions).
    pub fn rhythm(mut self, pattern: Vec<f64>) -> Self {
        self.rhythm = pattern;
        self
    }

    /// Builder: set tempo range (min, max) in BPM.
    pub fn tempo_range(mut self, range: (f64, f64)) -> Self {
        self.tempo_range = range;
        self
    }

    /// Builder: set timbre profile.
    pub fn timbre(mut self, profile: TimbreProfile) -> Self {
        self.timbre = profile;
        self
    }
}
