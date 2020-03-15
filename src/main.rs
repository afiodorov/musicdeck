use std::convert::TryFrom;

static SHARP_NOTES: [&'static str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

static FLAT_NOTES: [&'static str; 12] = [
    "A", "Bb", "Cb", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

fn add_semetones(frequency: f64, n: i64) -> f64 {
    frequency * 2f64.powf(n as f64 / 12.0)
}

fn frequency_to_note(frequency: f64, flat: bool) -> &'static str {
    let num_semisteps = 12.0 * (frequency / 440.0).log2();
    let num_semisteps = num_semisteps.round() as i64;
    let num_semisteps = num_semisteps.rem_euclid(12);
    let index = usize::try_from(num_semisteps).unwrap();
    if flat {
        return FLAT_NOTES[index];
    }
    SHARP_NOTES[index]
}

fn is_flat(note: &str) -> bool {
    let ch = note.chars().last().unwrap();
    ch == 'b'
}

fn note_to_frequency(note: &str) -> f64 {
    if let Some(p) = SHARP_NOTES.iter().position(|x| x == &note) {
        return 440.0 * 2.0f64.powf(p as f64 / 12.0);
    }
    let p = FLAT_NOTES.iter().position(|x| x == &note).unwrap();
    440.0 * 2.0f64.powf(p as f64 / 12.0)
}

fn perfect_n_fifth(note: &str, n: i64) -> &'static str {
    let start = note_to_frequency(note);
    frequency_to_note(add_semetones(start, 7 * n), is_flat(note))
}

fn perfect_fifth(note: &str) -> &'static str {
    perfect_n_fifth(note, 1)
}

fn perfect_n_forths(note: &str, n: i64) -> &'static str {
    let start = note_to_frequency(note);
    frequency_to_note(add_semetones(start, 5 * n), is_flat(note))
}

fn perfect_forth(note: &str) -> &'static str {
    perfect_n_forths(note, 1)
}

fn major_scale(note: &str) -> Vec<&'static str> {
    let scale = vec![-1, 0, 1, 2, 3, 4, 5];
    scale.iter().map(|s| perfect_n_fifth(note, *s)).collect()
}

fn enharmonic(note: &str) -> &'static str {
    frequency_to_note(note_to_frequency(note), !is_flat(note))
}

fn main() {
    println!("3 enharmonics; C# F# B# <-> Db Gb Cb");
    for note in &["C", "G", "D", "A", "E"] {
        println!("major scale of {}; {}", note, major_scale(note).join(" "))
    }
    for note in &["Ab", "Eb", "Bb", "F"] {
        println!("major scale of {}; {}", note, major_scale(note).join(" "))
    }
    for note in &["Db", "Gb", "Cb"] {
        println!(
            "major scale of {}; (<-> {}) {}",
            note,
            enharmonic(note),
            major_scale(note).join(" ")
        )
    }
    for note in &["C#", "F#", "B"] {
        println!(
            "major scale of {}; (<-> {}) {}",
            note,
            enharmonic(note),
            major_scale(note).join(" ")
        )
    }
}
