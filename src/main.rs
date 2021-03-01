#[derive(Debug)]
struct Chord {
    function: String,
    degree: i32,
    quality: String,
    note: String,
}

fn main() {
    // Initialize Data
    let _major_vec: Vec<Chord> = Vec::new();
    let _minor_vec: Vec<Chord> = Vec::new();

    let notes: Vec<&str> = vec![
        "C", "C#", "D", "D# / Eb", "E", "F", "F#", "G", "G# / Ab", "A", "A# / Bb", "B",
    ];
    let quality: Vec<&str> = vec!["ma6", "ma7", "mi7", "mi(ma7)", "mi7(b5)", "dim", "7"];

    // Runtime
    link_tonality("".to_string(), notes, quality);
}

// Based on user input, create resulting chord struct.
fn assemble_chord(note_names: Vec<&str>, chord_quality: Vec<&str>) -> Chord {
    let current_chord = Chord {
        function: "Tonic".to_string(),
        degree: 2,
        quality: chord_quality[0].to_owned(),
        note: note_names[2].to_string(),
    };

    current_chord
}

// Link given chord to tonal matrix
fn link_tonality(chord_in: String, note_names: Vec<&str>, chord_quality: Vec<&str>) {
    let res = assemble_chord(note_names, chord_quality);
    println!("{:#?}, {}", res, chord_in);
}
