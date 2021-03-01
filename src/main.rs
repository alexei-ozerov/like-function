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

    // Get User Input
    let input_tuple = get_user_input(&notes, &quality);

    // Runtime
    link_tonality(input_tuple.0, input_tuple.1, notes, quality);
}

fn get_user_input(notes_vec: &Vec<&str>, quality_vec: &Vec<&str>) -> (String, String) {
    // Select Key
    let mut input_key = String::new();
    let mut index = 1;
    println!("Select a key: ");

    for note in notes_vec.iter() {
        println!("{}) {}", index, note);
        index = index + 1;
    }

    println!("");
    std::io::stdin().read_line(&mut input_key).unwrap();
    println!("");

    // Select Quality
    let mut input_quality = String::new();
    index = 1;
    println!("Select quality: ");

    for qual in quality_vec.iter() {
        println!("{}) {}", index, qual);
        index = index + 1
    }

    println!("");
    std::io::stdin().read_line(&mut input_quality).unwrap();
    println!("");

    // Cut Newline
    let output_key: Vec<&str> = input_key.split("\n").collect();
    let output_quality: Vec<&str> = input_quality.split("\n").collect();
    
    // Return Tuple
    (output_key[0].to_string(), output_quality[0].to_string())
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
fn link_tonality(chord_in: String, quality_in: String, note_names: Vec<&str>, chord_quality: Vec<&str>) {
    
    let chord_index = chord_in.parse::<i32>().unwrap() - 1;
    let quality_index = quality_in.parse::<i32>().unwrap() - 1;
    println!("Getting Pivot Options For: {}{}", note_names[chord_index as usize], chord_quality[quality_index as usize]);

    let _res = assemble_chord(note_names, chord_quality);
}
