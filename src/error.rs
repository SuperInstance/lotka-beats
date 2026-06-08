use std::fmt;

/// Errors produced by the lotka-beats crate.
#[derive(Debug)]
pub enum Error {
    /// No species have been added to the ecosystem.
    EmptyEcosystem,
    /// A species index was out of range.
    InvalidSpeciesIndex(usize),
    /// The integration step size must be positive.
    InvalidStepSize(f64),
    /// A population went negative or diverged to infinity/NaN.
    DivergentPopulation { species: String, value: f64 },
    /// Interaction matrix dimension mismatch.
    InteractionMatrixMismatch { expected: usize, got: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyEcosystem => write!(f, "ecosystem has no species"),
            Error::InvalidSpeciesIndex(i) => write!(f, "species index {i} out of range"),
            Error::InvalidStepSize(dt) => write!(f, "invalid step size: {dt}"),
            Error::DivergentPopulation { species, value } => {
                write!(f, "population for '{species}' diverged: {value}")
            }
            Error::InteractionMatrixMismatch { expected, got } => {
                write!(f, "interaction matrix mismatch: expected {expected} entries, got {got}")
            }
        }
    }
}

impl std::error::Error for Error {}
