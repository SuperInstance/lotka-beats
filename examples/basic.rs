use lotka_beats::{MusicEcosystem, genre, equilibrium, midi};

fn main() {
    // Create a classic predator-prey ecosystem: Jazz vs Blues
    let mut eco = MusicEcosystem::new(0.05).unwrap();
    for sp in genre::classic_predator_prey() {
        eco.add_species(sp);
    }

    println!("=== Lotka-Beats: Ecosystem Dynamics ===\n");
    println!("Initial populations: {:?}", eco.populations());

    // Run for 200 steps
    eco.run(200).unwrap();

    println!("After 200 steps (t={:.1}):", eco.time);
    for sp in &eco.species {
        println!("  {}: population = {:.4}", sp.name, sp.population);
    }

    // Find equilibrium points (these are "new genres" invented by the math)
    let fixed_points = equilibrium::find_fixed_points(&eco).unwrap();
    println!("\nEquilibrium genres:");
    for fp in &fixed_points {
        println!(
            "  {} — {:?} stability: {:?}",
            fp.name.as_deref().unwrap_or("?"),
            fp.populations,
            fp.stability
        );
    }

    // Generate MIDI output
    let seq = midi::ecosystem_to_midi(&eco).unwrap();
    println!("\nMIDI output: {} events, {:.1} BPM", seq.events.len(), seq.tempo_bpm);

    // Show first few events
    for e in seq.events.iter().take(5) {
        println!(
            "  t={:05} ch={} note={:03} vel={:03} dur={}",
            e.tick, e.channel, e.note, e.velocity, e.duration
        );
    }

    // Try a 3-species competitive ecosystem
    let mut eco3 = MusicEcosystem::new(0.01).unwrap();
    for sp in genre::competitive_three() {
        eco3.add_species(sp);
    }
    eco3.run(500).unwrap();

    println!("\n=== 3-Species Competition ===");
    for sp in &eco3.species {
        println!("  {}: {:.4}", sp.name, sp.population);
    }
}
