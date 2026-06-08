use crate::species::{MusicalSpecies, TimbreProfile};

/// Pre-defined Jazz species.
pub fn jazz() -> MusicalSpecies {
    MusicalSpecies::new("Jazz", 1.0)
        .growth_rate(0.15)
        .death_rate(0.03)
        .scale(vec![0, 2, 3, 5, 7, 9, 10]) // dorian / mix of jazz scales
        .rhythm(vec![0.0, 0.33, 0.5, 0.67, 0.75]) // swing
        .tempo_range((100.0, 200.0))
        .timbre(TimbreProfile {
            brightness: 0.6,
            warmth: 0.7,
            complexity: 0.9,
            dynamics: 0.8,
        })
}

/// Pre-defined Classical species.
pub fn classical() -> MusicalSpecies {
    MusicalSpecies::new("Classical", 0.8)
        .growth_rate(0.08)
        .death_rate(0.02)
        .scale(vec![0, 2, 4, 5, 7, 9, 11]) // major
        .rhythm(vec![0.0, 0.25, 0.5, 0.75])
        .tempo_range((60.0, 180.0))
        .timbre(TimbreProfile {
            brightness: 0.4,
            warmth: 0.8,
            complexity: 0.7,
            dynamics: 0.9,
        })
}

/// Pre-defined Electronic species.
pub fn electronic() -> MusicalSpecies {
    MusicalSpecies::new("Electronic", 1.2)
        .growth_rate(0.2)
        .death_rate(0.04)
        .scale(vec![0, 3, 5, 7, 10]) // minor pentatonic
        .rhythm(vec![0.0, 0.25, 0.5, 0.75])
        .tempo_range((120.0, 180.0))
        .timbre(TimbreProfile {
            brightness: 0.9,
            warmth: 0.3,
            complexity: 0.5,
            dynamics: 0.4,
        })
}

/// Pre-defined Folk species.
pub fn folk() -> MusicalSpecies {
    MusicalSpecies::new("Folk", 0.6)
        .growth_rate(0.05)
        .death_rate(0.02)
        .scale(vec![0, 2, 4, 5, 7, 9]) // major pentatonic-ish
        .rhythm(vec![0.0, 0.5])
        .tempo_range((80.0, 140.0))
        .timbre(TimbreProfile {
            brightness: 0.3,
            warmth: 0.9,
            complexity: 0.3,
            dynamics: 0.6,
        })
}

/// Pre-defined Blues species.
pub fn blues() -> MusicalSpecies {
    MusicalSpecies::new("Blues", 0.9)
        .growth_rate(0.1)
        .death_rate(0.04)
        .scale(vec![0, 3, 5, 6, 7, 10]) // blues scale
        .rhythm(vec![0.0, 0.33, 0.67]) // shuffle
        .tempo_range((70.0, 140.0))
        .timbre(TimbreProfile {
            brightness: 0.5,
            warmth: 0.8,
            complexity: 0.6,
            dynamics: 0.7,
        })
}

/// Pre-defined Ambient species.
pub fn ambient() -> MusicalSpecies {
    MusicalSpecies::new("Ambient", 0.5)
        .growth_rate(0.03)
        .death_rate(0.01)
        .scale(vec![0, 2, 4, 7, 9]) // pentatonic
        .rhythm(vec![0.0, 0.5])
        .tempo_range((60.0, 90.0))
        .timbre(TimbreProfile {
            brightness: 0.2,
            warmth: 1.0,
            complexity: 0.2,
            dynamics: 0.3,
        })
}

/// Pre-defined Rock species.
pub fn rock() -> MusicalSpecies {
    MusicalSpecies::new("Rock", 1.0)
        .growth_rate(0.12)
        .death_rate(0.05)
        .scale(vec![0, 3, 5, 6, 7, 10]) // blues-influenced
        .rhythm(vec![0.0, 0.25, 0.5, 0.75]) // four-on-the-floor
        .tempo_range((100.0, 160.0))
        .timbre(TimbreProfile {
            brightness: 0.7,
            warmth: 0.5,
            complexity: 0.4,
            dynamics: 0.7,
        })
}

/// Pre-defined HipHop species.
pub fn hip_hop() -> MusicalSpecies {
    MusicalSpecies::new("HipHop", 1.1)
        .growth_rate(0.18)
        .death_rate(0.03)
        .scale(vec![0, 3, 5, 7, 10]) // minor pentatonic
        .rhythm(vec![0.0, 0.25, 0.375, 0.75]) // boom-bap
        .tempo_range((80.0, 110.0))
        .timbre(TimbreProfile {
            brightness: 0.5,
            warmth: 0.6,
            complexity: 0.8,
            dynamics: 0.6,
        })
}

/// Build interaction vectors for a pair of species in an n-species system.
/// Places coefficients at the correct indices.
pub fn make_interaction(n: usize, pairs: &[(usize, f64)]) -> Vec<f64> {
    let mut v = vec![0.0; n];
    for &(idx, coeff) in pairs {
        if idx < n {
            v[idx] = coeff;
        }
    }
    v
}

/// Create a classic 2-species predator-prey ecosystem (Jazz vs Blues).
pub fn classic_predator_prey() -> Vec<MusicalSpecies> {
    let mut j = jazz();
    j.interaction = vec![-0.1, 0.02]; // jazz self-limits, benefits from blues
    j.population = 1.0;

    let mut b = blues();
    b.interaction = vec![0.01, -0.05]; // blues benefits from jazz, self-limits
    b.population = 0.5;

    vec![j, b]
}

/// Create a 3-species competitive ecosystem (Electronic vs Rock vs Folk).
pub fn competitive_three() -> Vec<MusicalSpecies> {
    let mut e = electronic();
    e.interaction = vec![-0.1, -0.05, -0.03]; // competes with all
    e.population = 1.0;

    let mut r = rock();
    r.interaction = vec![-0.04, -0.08, -0.02];
    r.population = 0.8;

    let mut f = folk();
    f.interaction = vec![-0.02, -0.01, -0.06];
    f.population = 0.6;

    vec![e, r, f]
}
