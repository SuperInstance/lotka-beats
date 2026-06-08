use serde::{Deserialize, Serialize};

use crate::ecosystem::MusicEcosystem;
use crate::error::Error;
use crate::species::MusicalSpecies;

/// A single MIDI event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiEvent {
    /// MIDI tick / time offset.
    pub tick: u32,
    /// MIDI note number (0-127).
    pub note: u8,
    /// Velocity (0-127).
    pub velocity: u8,
    /// Duration in ticks.
    pub duration: u32,
    /// Channel (0-15).
    pub channel: u8,
}

/// A complete MIDI sequence generated from ecosystem dynamics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiSequence {
    /// Events in chronological order.
    pub events: Vec<MidiEvent>,
    /// Tempo in BPM derived from dominant species.
    pub tempo_bpm: f64,
    /// Ticks per quarter note.
    pub ppqn: u16,
    /// Name for the sequence.
    pub name: String,
}

/// Convert an ecosystem snapshot to a MIDI sequence.
pub fn ecosystem_to_midi(ecosystem: &MusicEcosystem) -> Result<MidiSequence, Error> {
    if ecosystem.species.is_empty() {
        return Err(Error::EmptyEcosystem);
    }
    if ecosystem.history.is_empty() {
        return Err(Error::EmptyEcosystem);
    }

        let _n = ecosystem.species.len();
    let populations: Vec<f64> = ecosystem.species.iter().map(|s| s.population).collect();
    let total: f64 = populations.iter().sum();

    // Determine dominant species for tempo
    let dom_idx = MusicEcosystem::dominant_index(&populations);
    let dominant = &ecosystem.species[dom_idx];

    // Tempo: interpolate within dominant species' range based on its dominance ratio
    let dominance_ratio = if total > 0.0 {
        populations[dom_idx] / total
    } else {
        0.5
    };
    let tempo = dominant.tempo_range.0
        + dominance_ratio * (dominant.tempo_range.1 - dominant.tempo_range.0);

    let ppqn: u16 = 480;
    let mut events = Vec::new();

    // Generate events for each species proportional to population
        let _steps = ecosystem.history.len();
    let tick_step = ppqn as u32 * 4; // whole note per history step

    for (step_idx, snap) in ecosystem.history.iter().enumerate() {
        let base_tick = step_idx as u32 * tick_step;

        for (sp_idx, sp) in ecosystem.species.iter().enumerate() {
            let pop = snap.populations[sp_idx];
            if pop < 0.01 {
                continue;
            }

            let weight = if total > 0.0 { pop / total } else { 0.0 };

            // Number of notes proportional to population
            let note_count = ((weight * sp.rhythm.len() as f64).ceil() as usize).max(1);

            for note_i in 0..note_count {
                let rhythm_pos = if sp.rhythm.is_empty() {
                    0.0
                } else {
                    sp.rhythm[note_i % sp.rhythm.len()]
                };

                // Pitch: pick from scale, weighted by population rank
                let scale_idx = if !sp.scale.is_empty() {
                    (sp_idx + note_i) % sp.scale.len()
                } else {
                    0
                };
                let pitch_class = if !sp.scale.is_empty() {
                    sp.scale[scale_idx]
                } else {
                    60 // middle C fallback
                };

                // Octave: base octave + offset from species index
                let octave: u8 = 4 + (sp_idx % 3) as u8;
                let note = (octave * 12 + pitch_class).min(127);

                // Velocity: based on population dynamics and timbre
                let base_vel = (weight * 100.0 + 27.0).min(127.0);
                let dyn_factor = sp.timbre.dynamics;
                let velocity = ((base_vel * (0.5 + 0.5 * dyn_factor)) as u8).min(127);

                // Duration: based on rhythm position and timbre complexity
                let dur_ticks = (ppqn as f64 * (0.25 + 0.75 * sp.timbre.complexity)) as u32;

                let tick = base_tick + (rhythm_pos * tick_step as f64) as u32;

                events.push(MidiEvent {
                    tick,
                    note,
                    velocity,
                    duration: dur_ticks,
                    channel: (sp_idx % 16) as u8,
                });
            }
        }
    }

    // Sort by tick
    events.sort_by_key(|e| e.tick);

    let name = format!(
        "{}-Ecosystem-t{:.1}",
        dominant.name, ecosystem.time
    );

    Ok(MidiSequence {
        events,
        tempo_bpm: tempo,
        ppqn,
        name,
    })
}

/// Convert a single species snapshot to a chord based on population ratios.
pub fn population_to_chord(species: &[MusicalSpecies], populations: &[f64]) -> Vec<u8> {
    let total: f64 = populations.iter().sum();
    if total <= 0.0 || species.is_empty() {
        return vec![];
    }

    let mut chord = Vec::new();
    for (i, sp) in species.iter().enumerate() {
        let ratio = populations[i] / total;
        if ratio > 0.1 && !sp.scale.is_empty() {
            // Add root and third of the species' scale
            let root = sp.scale[0];
            let third = if sp.scale.len() > 2 { sp.scale[2] } else { root + 4 };
            let octave: u8 = 4 + (i % 2) as u8;
            chord.push((octave * 12 + root).min(127));
            chord.push((octave * 12 + third).min(127));
        }
    }
    chord.sort();
    chord.dedup();
    chord
}

/// Export a MIDI sequence as a simple text-based representation.
pub fn midi_to_text(seq: &MidiSequence) -> String {
    let mut lines = Vec::new();
    lines.push(format!("Sequence: {}", seq.name));
    lines.push(format!("Tempo: {:.1} BPM, PPQN: {}", seq.tempo_bpm, seq.ppqn));
    lines.push(format!("Events: {}", seq.events.len()));
    lines.push("---".to_string());
    for e in &seq.events {
        lines.push(format!(
            "t={:05} ch={} note={:03} vel={:03} dur={}",
            e.tick, e.channel, e.note, e.velocity, e.duration
        ));
    }
    lines.join("\n")
}
