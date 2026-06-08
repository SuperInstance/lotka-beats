use serde::{Deserialize, Serialize};

use crate::dynamics::LotkaVolterra;
use crate::error::Error;
use crate::species::MusicalSpecies;

/// Snapshot of ecosystem state at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSnapshot {
    /// Simulation time.
    pub time: f64,
    /// Population of each species.
    pub populations: Vec<f64>,
    /// Index of the dominant (most populous) species.
    pub dominant: usize,
    /// Shannon entropy of population distribution.
    pub diversity: f64,
}

/// An ecosystem of competing musical traditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicEcosystem {
    /// The musical species in this ecosystem.
    pub species: Vec<MusicalSpecies>,
    /// Current simulation time.
    pub time: f64,
    /// Integration step size.
    pub dt: f64,
    /// History of ecosystem snapshots.
    pub history: Vec<EcosystemSnapshot>,
}

impl MusicEcosystem {
    /// Create a new ecosystem with the given step size.
    pub fn new(dt: f64) -> Result<Self, Error> {
        if dt <= 0.0 {
            return Err(Error::InvalidStepSize(dt));
        }
        Ok(Self {
            species: Vec::new(),
            time: 0.0,
            dt,
            history: Vec::new(),
        })
    }

    /// Add a species to the ecosystem.
    /// Returns the index of the added species.
    pub fn add_species(&mut self, species: MusicalSpecies) -> usize {
        let idx = self.species.len();
        self.species.push(species);
        idx
    }

    /// Compute Shannon diversity (entropy) of the current population distribution.
    pub fn shannon_diversity(populations: &[f64]) -> f64 {
        let total: f64 = populations.iter().sum();
        if total <= 0.0 {
            return 0.0;
        }
        let mut entropy = 0.0;
        for &p in populations {
            if p > 0.0 {
                let frac = p / total;
                entropy -= frac * frac.ln();
            }
        }
        entropy
    }

    /// Find the index of the dominant species.
    pub fn dominant_index(populations: &[f64]) -> usize {
        populations
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Take a snapshot of the current state.
    pub fn snapshot(&self) -> EcosystemSnapshot {
        let populations: Vec<f64> = self.species.iter().map(|s| s.population).collect();
        EcosystemSnapshot {
            time: self.time,
            dominant: Self::dominant_index(&populations),
            diversity: Self::shannon_diversity(&populations),
            populations,
        }
    }

    /// Advance the ecosystem by one time step using RK4 integration.
    pub fn step(&mut self) -> Result<(), Error> {
        if self.species.is_empty() {
            return Err(Error::EmptyEcosystem);
        }

        let populations: Vec<f64> = self.species.iter().map(|s| s.population).collect();

        // Build interaction coefficients for the solver
        let solver = LotkaVolterra::from_species(&self.species)?;
        let new_pops = solver.step(&populations, self.dt)?;

        for (i, sp) in self.species.iter_mut().enumerate() {
            sp.population = new_pops[i];
        }
        self.time += self.dt;

        // Record snapshot
        let snap = self.snapshot();
        self.history.push(snap);

        Ok(())
    }

    /// Run the ecosystem for `steps` time steps.
    pub fn run(&mut self, steps: usize) -> Result<(), Error> {
        for _ in 0..steps {
            self.step()?;
        }
        Ok(())
    }

    /// Get current populations as a vector.
    pub fn populations(&self) -> Vec<f64> {
        self.species.iter().map(|s| s.population).collect()
    }

    /// Total population across all species.
    pub fn total_population(&self) -> f64 {
        self.species.iter().map(|s| s.population).sum()
    }
}
