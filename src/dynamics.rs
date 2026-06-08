use crate::error::Error;
use crate::species::MusicalSpecies;

/// Generalized n-species Lotka-Volterra solver using RK4 integration.
///
/// The model:
///   dN_i/dt = N_i * (r_i + Σ_j a_ij * N_j)
///
/// where r_i = growth_rate - death_rate, and a_ij are interaction coefficients.
pub struct LotkaVolterra {
    /// Intrinsic growth rates (r_i = growth_rate_i - death_rate_i).
    intrinsic: Vec<f64>,
    /// Interaction matrix A[i][j] = effect of species j on species i.
    interaction_matrix: Vec<Vec<f64>>,
    /// Number of species.
    n: usize,
}

impl LotkaVolterra {
    /// Build a solver from a list of species.
    pub fn from_species(species: &[MusicalSpecies]) -> Result<Self, Error> {
        if species.is_empty() {
            return Err(Error::EmptyEcosystem);
        }
        let n = species.len();
        let intrinsic: Vec<f64> = species.iter().map(|s| s.growth_rate - s.death_rate).collect();

        let mut interaction_matrix = vec![vec![0.0_f64; n]; n];
        for (i, sp) in species.iter().enumerate() {
            if sp.interaction.len() != n {
                return Err(Error::InteractionMatrixMismatch {
                    expected: n,
                    got: sp.interaction.len(),
                });
            }
            for (j, &c) in sp.interaction.iter().enumerate() {
                interaction_matrix[i][j] = c;
            }
        }

        Ok(Self {
            intrinsic,
            interaction_matrix,
            n,
        })
    }

    /// Compute derivatives dN/dt for current populations.
    pub fn derivatives(&self, populations: &[f64]) -> Vec<f64> {
        let n = self.n;
        let mut dn = vec![0.0; n];
        for i in 0..n {
            let mut sum = self.intrinsic[i];
            for (j, &pop) in populations.iter().enumerate().take(n) {
                sum += self.interaction_matrix[i][j] * pop;
            }
            dn[i] = populations[i] * sum;
        }
        dn
    }

    /// Advance populations by one RK4 step.
    pub fn step(&self, populations: &[f64], dt: f64) -> Result<Vec<f64>, Error> {
        if dt <= 0.0 {
            return Err(Error::InvalidStepSize(dt));
        }
        let k1 = self.derivatives(populations);

        let mut p2 = vec![0.0; self.n];
        for i in 0..self.n {
            p2[i] = populations[i] + 0.5 * dt * k1[i];
        }
        let k2 = self.derivatives(&p2);

        let mut p3 = vec![0.0; self.n];
        for i in 0..self.n {
            p3[i] = populations[i] + 0.5 * dt * k2[i];
        }
        let k3 = self.derivatives(&p3);

        let mut p4 = vec![0.0; self.n];
        for i in 0..self.n {
            p4[i] = populations[i] + dt * k3[i];
        }
        let k4 = self.derivatives(&p4);

        let mut result = vec![0.0; self.n];
        for i in 0..self.n {
            result[i] = populations[i] + (dt / 6.0) * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
            if !result[i].is_finite() || result[i] < 0.0 {
                return Err(Error::DivergentPopulation {
                    species: format!("species_{i}"),
                    value: result[i],
                });
            }
        }
        Ok(result)
    }

    /// Get the interaction matrix.
    pub fn interaction_matrix(&self) -> &[Vec<f64>] {
        &self.interaction_matrix
    }

    /// Get intrinsic growth rates.
    pub fn intrinsic_rates(&self) -> &[f64] {
        &self.intrinsic
    }
}
