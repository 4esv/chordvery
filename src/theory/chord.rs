use std::collections::HashSet;

use super::note::Note;
use super::quality::Quality;

const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chord {
    pub root: Note,
    pub quality: Quality,
    pub bass: Option<Note>,
}

impl Chord {
    pub fn new(root: Note, quality: Quality) -> Self {
        Self {
            root,
            quality,
            bass: None,
        }
    }

    pub fn with_bass(mut self, bass: Note) -> Self {
        self.bass = Some(bass);
        self
    }

    pub fn detect(notes: &HashSet<u8>) -> Option<Self> {
        if notes.len() < 3 {
            return None;
        }

        let pitch_classes: HashSet<u8> = notes.iter().map(|&n| n % 12).collect();

        if pitch_classes.len() < 3 {
            return None;
        }

        let lowest_note = *notes.iter().min()?;
        let lowest_pitch_class = lowest_note % 12;

        let mut best_match: Option<Chord> = None;
        let mut best_score = 0;

        for &potential_root in pitch_classes.iter() {
            let intervals: HashSet<u8> = pitch_classes
                .iter()
                .map(|&pc| (pc + 12 - potential_root) % 12)
                .collect();

            for quality in Quality::all_sevenths()
                .iter()
                .chain(Quality::all_triads().iter())
            {
                let quality_intervals: HashSet<u8> =
                    quality.intervals().iter().map(|&i| i % 12).collect();

                if intervals == quality_intervals {
                    let is_root_position = potential_root == lowest_pitch_class;
                    let is_seventh = quality.intervals().len() == 4;
                    let score =
                        if is_root_position { 10 } else { 5 } + if is_seventh { 2 } else { 0 };

                    if score > best_score {
                        let mut chord = Chord::new(Note::new(potential_root + 60), *quality);

                        if !is_root_position {
                            chord.bass = Some(Note::new(lowest_note));
                        }

                        best_match = Some(chord);
                        best_score = score;
                    }
                }
            }
        }

        best_match
    }

    pub fn name(&self) -> String {
        let base = format!("{}{}", self.root.name(), self.quality.symbol());
        match &self.bass {
            Some(bass) if bass.pitch_class() != self.root.pitch_class() => {
                format!("{}/{}", base, bass.name())
            }
            _ => base,
        }
    }

    pub fn roman_numeral(&self, key: Note) -> String {
        let degree = (self.root.pitch_class() + 12 - key.pitch_class()) % 12;

        let numeral = match degree {
            0 => "I",
            1 => "bII",
            2 => "II",
            3 => "bIII",
            4 => "III",
            5 => "IV",
            6 => "bV",
            7 => "V",
            8 => "bVI",
            9 => "VI",
            10 => "bVII",
            11 => "VII",
            _ => unreachable!(),
        };

        let is_minor = matches!(
            self.quality,
            Quality::Minor | Quality::Minor7 | Quality::MinorMajor7 | Quality::HalfDim7
        );
        let is_diminished = matches!(self.quality, Quality::Diminished | Quality::Diminished7);

        let base = if is_minor || is_diminished {
            numeral.to_lowercase()
        } else {
            numeral.to_string()
        };

        let suffix = match self.quality {
            Quality::Major | Quality::Minor => String::new(),
            Quality::Diminished => "°".to_string(),
            Quality::Augmented => "+".to_string(),
            Quality::Major7 => "maj7".to_string(),
            Quality::Minor7 => "7".to_string(),
            Quality::Dominant7 => "7".to_string(),
            Quality::Diminished7 => "°7".to_string(),
            Quality::HalfDim7 => "ø7".to_string(),
            _ => self.quality.symbol().to_string(),
        };

        format!("{}{}", base, suffix)
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim();
        if name.is_empty() {
            return None;
        }

        let (root_str, rest) = if name.len() >= 2 && name.chars().nth(1) == Some('#') {
            (&name[..2], &name[2..])
        } else if !name.is_empty() {
            (&name[..1], &name[1..])
        } else {
            return None;
        };

        let root_pitch_class = NOTE_NAMES.iter().position(|&n| n == root_str)? as u8;
        let root = Note::new(root_pitch_class + 60);

        let (quality_str, bass_str) = if let Some(idx) = rest.find('/') {
            (&rest[..idx], Some(&rest[idx + 1..]))
        } else {
            (rest, None)
        };

        let quality = match quality_str {
            "" => Quality::Major,
            "m" => Quality::Minor,
            "dim" | "°" => Quality::Diminished,
            "+" | "aug" => Quality::Augmented,
            "maj7" | "M7" => Quality::Major7,
            "m7" | "min7" => Quality::Minor7,
            "7" | "dom7" => Quality::Dominant7,
            "dim7" | "°7" => Quality::Diminished7,
            "m7b5" | "ø7" | "ø" => Quality::HalfDim7,
            "mMaj7" | "mM7" => Quality::MinorMajor7,
            "+7" | "aug7" => Quality::Augmented7,
            "sus2" => Quality::Sus2,
            "sus4" | "sus" => Quality::Sus4,
            "add9" => Quality::Add9,
            _ => return None,
        };

        let mut chord = Chord::new(root, quality);

        if let Some(bass_name) = bass_str {
            let bass_pitch_class = NOTE_NAMES.iter().position(|&n| n == bass_name)? as u8;
            chord.bass = Some(Note::new(bass_pitch_class + 60));
        }

        Some(chord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn notes_set(midi_notes: &[u8]) -> HashSet<u8> {
        midi_notes.iter().copied().collect()
    }

    #[test]
    fn test_detect_major() {
        let notes = notes_set(&[60, 64, 67]); // C, E, G
        let chord = Chord::detect(&notes).unwrap();
        assert_eq!(chord.root.name(), "C");
        assert_eq!(chord.quality, Quality::Major);
        assert_eq!(chord.name(), "C");
    }

    #[test]
    fn test_detect_minor() {
        let notes = notes_set(&[69, 72, 76]); // A, C, E
        let chord = Chord::detect(&notes).unwrap();
        assert_eq!(chord.root.name(), "A");
        assert_eq!(chord.quality, Quality::Minor);
        assert_eq!(chord.name(), "Am");
    }

    #[test]
    fn test_detect_seventh() {
        let notes = notes_set(&[67, 71, 74, 77]); // G, B, D, F
        let chord = Chord::detect(&notes).unwrap();
        assert_eq!(chord.root.name(), "G");
        assert_eq!(chord.quality, Quality::Dominant7);
        assert_eq!(chord.name(), "G7");
    }

    #[test]
    fn test_detect_inversion() {
        let notes = notes_set(&[64, 67, 72]); // E, G, C (C/E - first inversion)
        let chord = Chord::detect(&notes).unwrap();
        assert_eq!(chord.root.name(), "C");
        assert_eq!(chord.quality, Quality::Major);
        assert!(chord.bass.is_some());
        assert_eq!(chord.name(), "C/E");
    }

    #[test]
    fn test_detect_empty() {
        let notes = notes_set(&[]);
        assert!(Chord::detect(&notes).is_none());
    }

    #[test]
    fn test_detect_two_notes() {
        let notes = notes_set(&[60, 67]); // C, G
        assert!(Chord::detect(&notes).is_none());
    }

    #[test]
    fn test_roman_numeral() {
        let c_major = Chord::new(Note::new(60), Quality::Major);
        let key_c = Note::new(60);
        assert_eq!(c_major.roman_numeral(key_c), "I");

        let a_minor = Chord::new(Note::new(69), Quality::Minor);
        assert_eq!(a_minor.roman_numeral(key_c), "vi");

        let g_dom7 = Chord::new(Note::new(67), Quality::Dominant7);
        assert_eq!(g_dom7.roman_numeral(key_c), "V7");
    }

    #[test]
    fn test_from_name() {
        let chord = Chord::from_name("C").unwrap();
        assert_eq!(chord.root.name(), "C");
        assert_eq!(chord.quality, Quality::Major);

        let chord = Chord::from_name("Am").unwrap();
        assert_eq!(chord.root.name(), "A");
        assert_eq!(chord.quality, Quality::Minor);

        let chord = Chord::from_name("G7").unwrap();
        assert_eq!(chord.root.name(), "G");
        assert_eq!(chord.quality, Quality::Dominant7);

        let chord = Chord::from_name("F#m7").unwrap();
        assert_eq!(chord.root.name(), "F#");
        assert_eq!(chord.quality, Quality::Minor7);
    }
}
