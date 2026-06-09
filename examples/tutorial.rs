//! Tutorial: step-by-step through the Lotka-Volterra music ecosystem.
//!
//! Run with: cargo run --example tutorial

use lotka_beats::{MusicEcosystem, MusicalSpecies, TimbreProfile, genre, midi, equilibrium};

fn main() {
    println!("=== Lotka-Volterra Beats Tutorial ===\n");

    // Step 1: Pre-defined genre species
    println!("Step 1: Pre-defined genres");
    let genres = [
        ("Jazz", genre::jazz()),
        ("Classical", genre::classical()),
        ("Electronic", genre::electronic()),
        ("Blues", genre::blues()),
        ("Rock", genre::rock()),
        ("HipHop", genre::hip_hop()),
    ];
    for (name, sp) in &genres {
        println!("  {}: scale={:?}, tempo={}-{}, brightness={:.1}",
            name, sp.scale, sp.tempo_range.0, sp.tempo_range.1, sp.timbre.brightness);
    }

    // Step 2: Classic predator-prey (Jazz vs Blues)
    println!("\nStep 2: Classic predator-prey (Jazz vs Blues)");
    let mut eco = MusicEcosystem::new(0.1).unwrap();
    for sp in genre::classic_predator_prey() {
        println!("  Adding {} (pop={:.1}, growth={:.2}, death={:.2})",
            sp.name, sp.population, sp.growth_rate, sp.death_rate);
        eco.add_species(sp);
    }

    // Step 3: Run and observe
    println!("\nStep 3: Run simulation");
    for step in 0..20 {
        eco.step().unwrap();
        if step % 5 == 0 {
            let pops = eco.populations();
            println!("  t={:5.1}: Jazz={:.3}, Blues={:.3}, diversity={:.3}",
                eco.time, pops[0], pops[1],
                MusicEcosystem::shannon_diversity(&pops));
        }
    }

    // Step 4: Equilibrium analysis
    println!("\nStep 4: Equilibrium (fixed points)");
    let fps = equilibrium::find_fixed_points(&eco).unwrap();
    for fp in &fps {
        println!("  {} ({:?})", fp.name.as_deref().unwrap_or("?"), fp.stability);
        println!("    Populations: {:?}", fp.populations);
    }

    // Step 5: Competitive 3-species
    println!("\nStep 5: 3-species competitive ecosystem");
    let mut eco3 = MusicEcosystem::new(0.05).unwrap();
    for sp in genre::competitive_three() {
        println!("  Adding {} (pop={:.1})", sp.name, sp.population);
        eco3.add_species(sp);
    }
    eco3.run(100).unwrap();
    let pops = eco3.populations();
    println!("  After 100 steps:");
    for (i, sp) in eco3.species.iter().enumerate() {
        println!("    {}: population={:.3}", sp.name, pops[i]);
    }

    // Step 6: MIDI output
    println!("\nStep 6: MIDI output");
    let seq = midi::ecosystem_to_midi(&eco).unwrap();
    println!("  {} events, tempo={:.1} BPM", seq.events.len(), seq.tempo_bpm);

    let chord = midi::population_to_chord(&eco.species, &eco.populations());
    println!("  Current chord (MIDI notes): {:?}", chord);

    // Step 7: Custom species
    println!("\nStep 7: Custom species");
    let vapor = MusicalSpecies::new("Vaporwave", 0.8)
        .growth_rate(0.08)
        .death_rate(0.02)
        .scale(vec![0, 2, 4, 7, 9])     // pentatonic
        .rhythm(vec![0.0, 0.25, 0.5])    // half-time feel
        .tempo_range((60.0, 90.0))
        .timbre(TimbreProfile {
            brightness: 0.3,
            warmth: 0.8,
            complexity: 0.4,
            dynamics: 0.3,
        });
    println!("  {} created: scale={:?}, tempo={}-{}",
        vapor.name, vapor.scale, vapor.tempo_range.0, vapor.tempo_range.1);
}
