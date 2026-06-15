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

    /// Detect a chord from a set of MIDI notes.
    /// Uses flexible detection for extended chords - requires essential intervals
    /// but allows omitted tones (5th, etc.) common in jazz voicings.
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
            // Basic intervals (mod 12) for triads and 7ths
            let intervals: HashSet<u8> = pitch_classes
                .iter()
                .map(|&pc| (pc + 12 - potential_root) % 12)
                .collect();

            // Extended intervals - check for notes an octave+ above
            let extended_intervals: HashSet<u8> = notes
                .iter()
                .filter_map(|&n| {
                    let pc = n % 12;
                    let interval_from_root = (pc + 12 - potential_root) % 12;
                    // If this note could be an extension (9, 11, 13), check if it's actually higher
                    if interval_from_root == 2 {
                        // Could be 9th (14)
                        Some(if n > lowest_note + 12 { 14 } else { 2 })
                    } else if interval_from_root == 5 {
                        // Could be 11th (17)
                        Some(if n > lowest_note + 12 { 17 } else { 5 })
                    } else if interval_from_root == 9 {
                        // Could be 13th (21) or 6th
                        Some(if n > lowest_note + 12 { 21 } else { 9 })
                    } else {
                        Some(interval_from_root)
                    }
                })
                .collect();

            for quality in Quality::all_for_detection() {
                let essential = quality.essential_intervals();
                let full_intervals = quality.intervals();

                // Check if essential intervals are present
                let essential_present = essential.iter().all(|&interval| {
                    let mod_interval = interval % 12;
                    // For extensions, check extended_intervals
                    if interval > 12 {
                        extended_intervals.contains(&interval)
                            || extended_intervals.contains(&mod_interval)
                    } else {
                        intervals.contains(&mod_interval)
                    }
                });

                if !essential_present {
                    continue;
                }

                // Score based on how many of the full intervals are present
                let full_mod: HashSet<u8> = full_intervals.iter().map(|&i| i % 12).collect();
                let matching_notes = intervals.intersection(&full_mod).count();
                let extra_notes = intervals.len() - matching_notes;

                // Penalize heavily for extra notes that don't belong
                if extra_notes > 1 {
                    continue;
                }

                let is_root_position = potential_root == lowest_pitch_class;
                let complexity_bonus = full_intervals.len(); // More complex chords score higher

                let score = if is_root_position { 20 } else { 10 }
                    + complexity_bonus * 2
                    + matching_notes * 3
                    - extra_notes * 5;

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

        let is_minor = self.quality.is_minor();
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
            // Extended chords
            Quality::Major9 => "maj9".to_string(),
            Quality::Minor9 => "9".to_string(),
            Quality::Dominant9 => "9".to_string(),
            Quality::Major13 => "maj13".to_string(),
            Quality::Minor13 => "13".to_string(),
            Quality::Dominant13 => "13".to_string(),
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
            // Triads
            "" => Quality::Major,
            "m" => Quality::Minor,
            "dim" | "°" => Quality::Diminished,
            "+" | "aug" => Quality::Augmented,
            "sus2" => Quality::Sus2,
            "sus4" | "sus" => Quality::Sus4,
            // 6th chords
            "6" => Quality::Major6,
            "m6" => Quality::Minor6,
            // 7th chords
            "maj7" | "M7" => Quality::Major7,
            "m7" | "min7" => Quality::Minor7,
            "7" | "dom7" => Quality::Dominant7,
            "dim7" | "°7" => Quality::Diminished7,
            "m7b5" | "ø7" | "ø" => Quality::HalfDim7,
            "mMaj7" | "mM7" => Quality::MinorMajor7,
            "+7" | "aug7" => Quality::Augmented7,
            "7sus4" => Quality::Dom7sus4,
            "7sus2" => Quality::Dom7sus2,
            // Add chords
            "add9" => Quality::Add9,
            "add11" => Quality::Add11,
            // 9th chords
            "maj9" | "M9" => Quality::Major9,
            "m9" | "min9" => Quality::Minor9,
            "9" => Quality::Dominant9,
            "mMaj9" | "mM9" => Quality::MinorMajor9,
            // 11th chords
            "maj11" | "M11" => Quality::Major11,
            "m11" | "min11" => Quality::Minor11,
            "11" => Quality::Dominant11,
            // 13th chords
            "maj13" | "M13" => Quality::Major13,
            "m13" | "min13" => Quality::Minor13,
            "13" => Quality::Dominant13,
            // Altered dominants
            "7b9" => Quality::Dom7b9,
            "7#9" => Quality::Dom7Sharp9,
            "7#11" => Quality::Dom7Sharp11,
            "7b13" => Quality::Dom7b13,
            "7alt" | "alt" => Quality::Dom7Alt,
            // Lydian
            "maj7#11" | "M7#11" => Quality::Maj7Sharp11,
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
    fn test_detect_dominant9() {
        // G9 = G B D F A (shell voicing: G B F A - no D)
        let notes = notes_set(&[55, 59, 65, 69]); // G, B, F, A
        let chord = Chord::detect(&notes).unwrap();
        assert_eq!(chord.root.name(), "G");
        assert_eq!(chord.quality, Quality::Dominant9);
        assert_eq!(chord.name(), "G9");
    }

    #[test]
    fn test_detect_minor7_shell() {
        // Dm7 shell voicing: D F C (no A)
        let notes = notes_set(&[62, 65, 72]); // D, F, C
        let chord = Chord::detect(&notes).unwrap();
        assert_eq!(chord.root.name(), "D");
        assert_eq!(chord.quality, Quality::Minor7);
    }

    #[test]
    fn test_detect_major6() {
        // C6 = C E G A
        let notes = notes_set(&[60, 64, 67, 69]); // C, E, G, A
        let chord = Chord::detect(&notes).unwrap();
        assert_eq!(chord.root.name(), "C");
        assert_eq!(chord.quality, Quality::Major6);
        assert_eq!(chord.name(), "C6");
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

    #[test]
    fn test_from_name_extended() {
        let chord = Chord::from_name("Cmaj9").unwrap();
        assert_eq!(chord.quality, Quality::Major9);

        let chord = Chord::from_name("D9").unwrap();
        assert_eq!(chord.quality, Quality::Dominant9);

        let chord = Chord::from_name("Am11").unwrap();
        assert_eq!(chord.quality, Quality::Minor11);

        let chord = Chord::from_name("G13").unwrap();
        assert_eq!(chord.quality, Quality::Dominant13);

        let chord = Chord::from_name("C7#9").unwrap();
        assert_eq!(chord.quality, Quality::Dom7Sharp9);
    }
}
