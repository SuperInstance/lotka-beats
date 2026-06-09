//! Basic lotka-beats usage: create an ecosystem and watch genres compete.
//!
//! Run with: cargo run --example basic

use lotka_beats::{MusicEcosystem, genre};

fn main() {
    // Create a classic predator-prey ecosystem: Jazz vs Blues
    let mut eco = MusicEcosystem::new(0.1).unwrap();
    for sp in genre::classic_predator_prey() {
        eco.add_species(sp);
    }

    println!("Initial populations: {:?}", eco.populations());

    // Run for 100 steps
    eco.run(100).unwrap();

    println!("After 100 steps:");
    println!("  Time: {:.1}", eco.time);
    println!("  Populations: {:?}", eco.populations());
    println!("  Total: {:.2}", eco.total_population());
    println!("  History: {} snapshots", eco.history.len());

    // Check diversity
    let div = MusicEcosystem::shannon_diversity(&eco.populations());
    println!("  Shannon diversity: {:.3}", div);
}
