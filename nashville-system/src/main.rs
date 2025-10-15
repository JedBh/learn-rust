#[derive(Debug)]
enum ChordType {
    Major,
    Minor,
    Dominant7,
    Diminished,
}

#[derive(Debug)]
struct Chord {
    root: String,
    chord_type: ChordType,
    extensions: Vec<String>,
}

impl Chord {
    fn default_chord(root: String) -> Self {
        Self {
            root,
            chord_type: ChordType::Major,
            extensions: vec![],
        }
    }
}

fn main() {
    let g_major = Chord::default_chord(String::from("G"));

    println!("{:#?}", g_major)
}
