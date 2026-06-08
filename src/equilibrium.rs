use serde::{Deserialize, Serialize};

use crate::dynamics::LotkaVolterra;
use crate::ecosystem::MusicEcosystem;
use crate::error::Error;

/// Stability classification of a fixed point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stability {
    /// All eigenvalues have negative real parts.
    Stable,
    /// At least one eigenvalue has positive real part.
    Unstable,
    /// Mixed positive and negative eigenvalues.
    Saddle,
    /// Eigenvalues are purely imaginary or zero.
    Neutral,
}

/// A fixed point (equilibrium genre) of the ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquilibriumGenre {
    /// Population values at the fixed point.
    pub populations: Vec<f64>,
    /// Stability classification.
    pub stability: Stability,
    /// Auto-generated name for a new fusion genre.
    pub name: Option<String>,
}

/// Find the trivial fixed point (all populations zero).
pub fn trivial_fixed_point(n: usize) -> Vec<f64> {
    vec![0.0; n]
}

/// Find non-trivial fixed points by solving the nullcline equations.
///
/// For dN_i/dt = 0 with N_i ≠ 0:
///   r_i + Σ_j a_ij * N_j = 0
///
/// This is a linear system: A * N = -r
pub fn find_fixed_points(ecosystem: &MusicEcosystem) -> Result<Vec<EquilibriumGenre>, Error> {
    if ecosystem.species.is_empty() {
        return Err(Error::EmptyEcosystem);
    }

    let solver = LotkaVolterra::from_species(&ecosystem.species)?;
    let n = ecosystem.species.len();
    let matrix = solver.interaction_matrix();
    let intrinsic = solver.intrinsic_rates();

    let mut results = Vec::new();

    // Trivial equilibrium
    let trivial = trivial_fixed_point(n);
    results.push(EquilibriumGenre {
        populations: trivial,
        stability: Stability::Unstable,
        name: Some("Silence".to_string()),
    });

    // Non-trivial: solve A * N* = -r via Gaussian elimination
    if let Some(mut fp) = solve_linear_system(matrix, intrinsic, n) {
        // Clamp small negatives to zero
        for val in &mut fp {
            if *val < 0.0 && *val > -1e-10 {
                *val = 0.0;
            }
        }
        let valid = fp.iter().all(|&v| v >= 0.0);
        if valid {
            let stability = analyze_stability(matrix, intrinsic, &fp, n);
            let name = generate_genre_name(&ecosystem.species, &fp);
            results.push(EquilibriumGenre {
                populations: fp,
                stability,
                name: Some(name),
            });
        }
    }

    Ok(results)
}

/// Gaussian elimination to solve A * x = -r.
#[allow(clippy::needless_range_loop)]
fn solve_linear_system(a: &[Vec<f64>], r: &[f64], n: usize) -> Option<Vec<f64>> {
    // Augmented matrix [A | -r]
    let mut aug = vec![vec![0.0; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = a[i][j];
        }
        aug[i][n] = -r[i];
    }

    // Forward elimination with partial pivoting
    for col in 0..n {
        // Find pivot
        let mut max_row = col;
        let mut max_val = aug[col][col].abs();
        for row in (col + 1)..n {
            if aug[row][col].abs() > max_val {
                max_val = aug[row][col].abs();
                max_row = row;
            }
        }
        if max_val < 1e-12 {
            return None; // Singular
        }
        aug.swap(col, max_row);

        let pivot = aug[col][col];
        for row in (col + 1)..n {
            let factor = aug[row][col] / pivot;
            for j in col..=n {
                aug[row][j] -= factor * aug[col][j];
            }
        }
    }

    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        if aug[i][i].abs() < 1e-12 {
            return None;
        }
        x[i] = aug[i][n];
        for j in (i + 1)..n {
            x[i] -= aug[i][j] * x[j];
        }
        x[i] /= aug[i][i];
    }
    Some(x)
}

/// Analyze stability at a fixed point via the Jacobian.
///
/// The Jacobian of the generalized LV system at equilibrium N*:
///   J_ii = r_i + Σ_j a_ij * N*_j + a_ii * N*_i = a_ii * N*_i  (since r_i + Σ a_ij N*_j = 0)
///   J_ij = a_ij * N*_i  (for i ≠ j)
fn analyze_stability(a: &[Vec<f64>], _r: &[f64], fp: &[f64], n: usize) -> Stability {
    // Build Jacobian
    let mut jac = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            jac[i][j] = a[i][j] * fp[i];
        }
    }

    // Compute eigenvalues (for small n, use analytical or iterative method)
    let eigenvalues = compute_eigenvalues_2x2_or_trace_det(&jac, n);

    let has_positive = eigenvalues.iter().any(|(re, _)| *re > 1e-10);
    let has_negative = eigenvalues.iter().any(|(re, _)| *re < -1e-10);

    if has_positive && has_negative {
        Stability::Saddle
    } else if has_positive {
        Stability::Unstable
    } else if has_negative {
        Stability::Stable
    } else {
        Stability::Neutral
    }
}

/// Compute eigenvalue approximations using trace and determinant for 2x2,
/// or Gershgorin circles for larger matrices.
fn compute_eigenvalues_2x2_or_trace_det(jac: &[Vec<f64>], n: usize) -> Vec<(f64, f64)> {
    if n == 1 {
        return vec![(jac[0][0], 0.0)];
    }
    if n == 2 {
        // Exact eigenvalues for 2x2
        let trace = jac[0][0] + jac[1][1];
        let det = jac[0][0] * jac[1][1] - jac[0][1] * jac[1][0];
        let disc = trace * trace - 4.0 * det;
        if disc >= 0.0 {
            let sqrt_disc = disc.sqrt();
            vec![
                ((trace + sqrt_disc) / 2.0, 0.0),
                ((trace - sqrt_disc) / 2.0, 0.0),
            ]
        } else {
            let sqrt_disc = (-disc).sqrt();
            vec![
                (trace / 2.0, sqrt_disc / 2.0),
                (trace / 2.0, -sqrt_disc / 2.0),
            ]
        }
    } else {
        // Gershgorin circle estimate: use diagonal as real part estimate
        jac.iter()
            .enumerate()
            .map(|(i, row)| {
                let radius: f64 = row.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| v.abs()).sum();
                (row[i], radius)
            })
            .collect()
    }
}

/// Auto-generate a genre name from the equilibrium population distribution.
fn generate_genre_name(species: &[crate::species::MusicalSpecies], fp: &[f64]) -> String {
    let total: f64 = fp.iter().sum();
    if total <= 0.0 {
        return "Void".to_string();
    }

    // Get the top contributing species
    let mut indexed: Vec<(usize, f64)> = fp.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let top = &indexed[0];
    if top.1 / total > 0.8 {
        format!("Pure {} Fusion", species[top.0].name)
    } else if indexed.len() >= 2 && indexed[1].1 / total > 0.2 {
        format!(
            "{}-{} Crossover",
            species[indexed[0].0].name, species[indexed[1].0].name
        )
    } else {
        format!("{}-dominated Equilibrium", species[indexed[0].0].name)
    }
}
