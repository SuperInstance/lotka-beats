//! Advanced: custom interaction matrices, stability analysis, and MIDI generation.
//!
//! Run with: cargo run --example advanced

use lotka_beats::{
    MusicEcosystem, MusicalSpecies, TimbreProfile,
    LotkaVolterra, Stability,
    genre, midi, equilibrium,
};

fn main() {
    println!("=== Advanced Lotka-Volterra Beats ===\n");

    // ── Custom 4-species ecosystem ──
    println!("1. Custom 4-species ecosystem");
    let mut eco = MusicEcosystem::new(0.05).unwrap();

    // Jazz benefits from Blues, competes with Electronic
    let mut jazz = genre::jazz();
    jazz.interaction = vec![-0.1, 0.03, -0.05, 0.01];
    jazz.population = 1.0;
    eco.add_species(jazz);

    // Blues benefits from Jazz
    let mut blues = genre::blues();
    blues.interaction = vec![0.02, -0.08, -0.02, 0.01];
    blues.population = 0.5;
    eco.add_species(blues);

    // Electronic competes with everyone
    let mut electronic = genre::electronic();
    electronic.interaction = vec![-0.04, -0.03, -0.12, -0.02];
    electronic.population = 0.8;
    eco.add_species(electronic);

    // Ambient coexists peacefully
    let mut ambient = genre::ambient();
    ambient.interaction = vec![0.01, 0.01, -0.01, -0.04];
    ambient.population = 0.3;
    eco.add_species(ambient);

    // Run simulation
    eco.run(500).unwrap();
    let pops = eco.populations();
    println!("   After 500 steps (t={:.1}):", eco.time);
    for (i, sp) in eco.species.iter().enumerate() {
        println!("     {}: {:.4}", sp.name, pops[i]);
    }
    println!("   Total population: {:.2}", eco.total_population());
    println!("   Shannon diversity: {:.3}",
        MusicEcosystem::shannon_diversity(&pops));

    // ── Direct RK4 solver ──
    println!("\n2. Direct Lotka-Volterra solver");
    let species = genre::classic_predator_prey();
    let solver = LotkaVolterra::from_species(&species).unwrap();
    println!("   Intrinsic rates: {:?}", solver.intrinsic_rates());
    println!("   Interaction matrix:");
    for (i, row) in solver.interaction_matrix().iter().enumerate() {
        println!("     [{:.2}, {:.2}] ({})", row[0], row[1], species[i].name);
    }

    let mut pops = vec![1.0, 0.5];
    println!("   Initial: {:?}", pops);
    for _ in 0..100 {
        pops = solver.step(&pops, 0.01).unwrap();
    }
    println!("   After 100 steps: [{:.4}, {:.4}]", pops[0], pops[1]);

    // ── Equilibrium analysis ──
    println!("\n3. Equilibrium analysis");
    let fps = equilibrium::find_fixed_points(&eco).unwrap();
    for fp in &fps {
        println!("   {} ({:?})", fp.name.as_deref().unwrap_or("?"), fp.stability);
        println!("     Populations: {:?}", fp.populations
            .iter().zip(eco.species.iter())
            .map(|(p, s)| format!("{}={:.3}", s.name, p))
            .collect::<Vec<_>>());
    }

    // ── MIDI generation ──
    println!("\n4. Full MIDI generation");
    let seq = midi::ecosystem_to_midi(&eco).unwrap();
    println!("   Sequence: {}", seq.name);
    println!("   Events: {}", seq.events.len());
    println!("   Tempo: {:.1} BPM", seq.tempo_bpm);
    println!("   PPQN: {}", seq.ppqn);
    if let Some(first) = seq.events.first() {
        println!("   First event: tick={}, ch={}, note={}, vel={}",
            first.tick, first.channel, first.note, first.velocity);
    }

    // ── Population tracking over time ──
    println!("\n5. Population time series");
    let mut eco2 = MusicEcosystem::new(0.02).unwrap();
    for sp in genre::competitive_three() {
        eco2.add_species(sp);
    }
    eco2.run(50).unwrap();
    for snap in eco2.history.iter().step_by(10) {
        let names: Vec<String> = snap.populations.iter()
            .zip(eco2.species.iter())
            .map(|(p, s)| format!("{}={:.2}", s.name, p))
            .collect();
        println!("   t={:5.2}: {} (diversity={:.3})",
            snap.time, names.join(", "), snap.diversity);
    }

    // ── Building interaction vectors ──
    println!("\n6. Interaction helper");
    let v = genre::make_interaction(4, &[(0, -0.1), (2, 0.05), (3, -0.02)]);
    println!("   Interaction vector: {:?}", v);
}
