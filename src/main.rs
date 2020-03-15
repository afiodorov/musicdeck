use std::convert::TryFrom;

static NOTES: [&'static str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

fn frequency_to_note(frequency: f64) -> &'static str {
    let num_semisteps = 12.0 * (frequency / 440.0).log2();
    let num_semisteps = num_semisteps.round() as i64;
    let num_semisteps = num_semisteps.rem_euclid(12);
    let index = usize::try_from(num_semisteps).unwrap();
    NOTES[index]
}

fn note_to_frequency(note: &str) -> f64 {
    let pos = NOTES.iter().position(|x| x == &note).unwrap() as f64;
    440.0 * 2.0f64.powf(pos / 12.0)
}

fn major_scale(note: &str) -> Vec<&'static str> {
    let scale = vec![0, 2, 4, 5, 7, 9, 11];
    let start = note_to_frequency(note);
    let v: Vec<_> = scale
        .iter()
        .map(|s| {
            let s = *s as f64;
            frequency_to_note(start * 2f64.powf(s / 12.0))
        })
        .collect();
    v
}

fn main() {
    for note in &NOTES {
        println!("notes in {} major; {}", note, major_scale(note).join(" "));
    }
}
