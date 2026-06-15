#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quality {
    // Triads
    Major,
    Minor,
    Diminished,
    Augmented,
    Sus2,
    Sus4,

    // 6th chords
    Major6,
    Minor6,

    // 7th chords
    Major7,
    Minor7,
    Dominant7,
    Diminished7,
    HalfDim7,
    MinorMajor7,
    Augmented7,

    // Suspended 7ths
    Dom7sus4,
    Dom7sus2,

    // Add chords
    Add9,
    Add11,

    // 9th chords
    Major9,
    Minor9,
    Dominant9,
    MinorMajor9,

    // 11th chords
    Major11,
    Minor11,
    Dominant11,

    // 13th chords
    Major13,
    Minor13,
    Dominant13,

    // Altered dominants
    Dom7b9,
    Dom7Sharp9,
    Dom7Sharp11,
    Dom7b13,
    Dom7Alt,

    // Lydian voicings
    Maj7Sharp11,

    Unknown,
}

impl Quality {
    pub fn symbol(&self) -> &'static str {
        match self {
            // Triads
            Quality::Major => "",
            Quality::Minor => "m",
            Quality::Diminished => "dim",
            Quality::Augmented => "+",
            Quality::Sus2 => "sus2",
            Quality::Sus4 => "sus4",

            // 6th chords
            Quality::Major6 => "6",
            Quality::Minor6 => "m6",

            // 7th chords
            Quality::Major7 => "maj7",
            Quality::Minor7 => "m7",
            Quality::Dominant7 => "7",
            Quality::Diminished7 => "dim7",
            Quality::HalfDim7 => "m7b5",
            Quality::MinorMajor7 => "mMaj7",
            Quality::Augmented7 => "+7",

            // Suspended 7ths
            Quality::Dom7sus4 => "7sus4",
            Quality::Dom7sus2 => "7sus2",

            // Add chords
            Quality::Add9 => "add9",
            Quality::Add11 => "add11",

            // 9th chords
            Quality::Major9 => "maj9",
            Quality::Minor9 => "m9",
            Quality::Dominant9 => "9",
            Quality::MinorMajor9 => "mMaj9",

            // 11th chords
            Quality::Major11 => "maj11",
            Quality::Minor11 => "m11",
            Quality::Dominant11 => "11",

            // 13th chords
            Quality::Major13 => "maj13",
            Quality::Minor13 => "m13",
            Quality::Dominant13 => "13",

            // Altered dominants
            Quality::Dom7b9 => "7b9",
            Quality::Dom7Sharp9 => "7#9",
            Quality::Dom7Sharp11 => "7#11",
            Quality::Dom7b13 => "7b13",
            Quality::Dom7Alt => "7alt",

            // Lydian voicings
            Quality::Maj7Sharp11 => "maj7#11",

            Quality::Unknown => "?",
        }
    }

    /// Returns the characteristic intervals for this chord quality.
    /// For extended chords, includes all typical voices but detection
    /// should use flexible matching (see `essential_intervals`).
    pub fn intervals(&self) -> &'static [u8] {
        match self {
            // Triads
            Quality::Major => &[0, 4, 7],
            Quality::Minor => &[0, 3, 7],
            Quality::Diminished => &[0, 3, 6],
            Quality::Augmented => &[0, 4, 8],
            Quality::Sus2 => &[0, 2, 7],
            Quality::Sus4 => &[0, 5, 7],

            // 6th chords
            Quality::Major6 => &[0, 4, 7, 9],
            Quality::Minor6 => &[0, 3, 7, 9],

            // 7th chords
            Quality::Major7 => &[0, 4, 7, 11],
            Quality::Minor7 => &[0, 3, 7, 10],
            Quality::Dominant7 => &[0, 4, 7, 10],
            Quality::Diminished7 => &[0, 3, 6, 9],
            Quality::HalfDim7 => &[0, 3, 6, 10],
            Quality::MinorMajor7 => &[0, 3, 7, 11],
            Quality::Augmented7 => &[0, 4, 8, 10],

            // Suspended 7ths
            Quality::Dom7sus4 => &[0, 5, 7, 10],
            Quality::Dom7sus2 => &[0, 2, 7, 10],

            // Add chords
            Quality::Add9 => &[0, 4, 7, 14],
            Quality::Add11 => &[0, 4, 7, 17],

            // 9th chords (R 3 5 7 9)
            Quality::Major9 => &[0, 4, 7, 11, 14],
            Quality::Minor9 => &[0, 3, 7, 10, 14],
            Quality::Dominant9 => &[0, 4, 7, 10, 14],
            Quality::MinorMajor9 => &[0, 3, 7, 11, 14],

            // 11th chords (R 3 5 7 9 11)
            Quality::Major11 => &[0, 4, 7, 11, 14, 17],
            Quality::Minor11 => &[0, 3, 7, 10, 14, 17],
            Quality::Dominant11 => &[0, 4, 7, 10, 14, 17],

            // 13th chords (R 3 5 7 9 13) - 11 often omitted
            Quality::Major13 => &[0, 4, 7, 11, 14, 21],
            Quality::Minor13 => &[0, 3, 7, 10, 14, 21],
            Quality::Dominant13 => &[0, 4, 7, 10, 14, 21],

            // Altered dominants
            Quality::Dom7b9 => &[0, 4, 7, 10, 13],
            Quality::Dom7Sharp9 => &[0, 4, 7, 10, 15],
            Quality::Dom7Sharp11 => &[0, 4, 7, 10, 18],
            Quality::Dom7b13 => &[0, 4, 7, 10, 20],
            Quality::Dom7Alt => &[0, 4, 10, 13, 15, 20], // R 3 b7 b9 #9 b13

            // Lydian voicings
            Quality::Maj7Sharp11 => &[0, 4, 7, 11, 18],

            Quality::Unknown => &[],
        }
    }

    /// Returns the essential intervals that must be present for flexible detection.
    /// For extended chords, this is typically the shell (R, 3, 7) plus the defining extension.
    pub fn essential_intervals(&self) -> &'static [u8] {
        match self {
            // Triads need all notes
            Quality::Major => &[0, 4, 7],
            Quality::Minor => &[0, 3, 7],
            Quality::Diminished => &[0, 3, 6],
            Quality::Augmented => &[0, 4, 8],
            Quality::Sus2 => &[0, 2, 7],
            Quality::Sus4 => &[0, 5, 7],

            // 6th chords: need the 6th
            Quality::Major6 => &[0, 4, 9],
            Quality::Minor6 => &[0, 3, 9],

            // 7th chords: R, 3/b3, 7/b7
            Quality::Major7 => &[0, 4, 11],
            Quality::Minor7 => &[0, 3, 10],
            Quality::Dominant7 => &[0, 4, 10],
            Quality::Diminished7 => &[0, 3, 6, 9],
            Quality::HalfDim7 => &[0, 3, 6, 10],
            Quality::MinorMajor7 => &[0, 3, 11],
            Quality::Augmented7 => &[0, 4, 8, 10],

            // Suspended 7ths: need sus + b7
            Quality::Dom7sus4 => &[0, 5, 10],
            Quality::Dom7sus2 => &[0, 2, 10],

            // Add chords: triad + extension
            Quality::Add9 => &[0, 4, 14],
            Quality::Add11 => &[0, 4, 17],

            // 9th chords: shell + 9
            Quality::Major9 => &[0, 4, 11, 14],
            Quality::Minor9 => &[0, 3, 10, 14],
            Quality::Dominant9 => &[0, 4, 10, 14],
            Quality::MinorMajor9 => &[0, 3, 11, 14],

            // 11th chords: shell + 11
            Quality::Major11 => &[0, 4, 11, 17],
            Quality::Minor11 => &[0, 3, 10, 17],
            Quality::Dominant11 => &[0, 4, 10, 17],

            // 13th chords: shell + 13
            Quality::Major13 => &[0, 4, 11, 21],
            Quality::Minor13 => &[0, 3, 10, 21],
            Quality::Dominant13 => &[0, 4, 10, 21],

            // Altered dominants: R, 3, b7 + alteration
            Quality::Dom7b9 => &[0, 4, 10, 13],
            Quality::Dom7Sharp9 => &[0, 4, 10, 15],
            Quality::Dom7Sharp11 => &[0, 4, 10, 18],
            Quality::Dom7b13 => &[0, 4, 10, 20],
            Quality::Dom7Alt => &[0, 4, 10, 13], // Needs at least b9

            // Lydian voicings
            Quality::Maj7Sharp11 => &[0, 4, 11, 18],

            Quality::Unknown => &[],
        }
    }

    /// Returns true if this quality represents a dominant function chord
    pub fn is_dominant(&self) -> bool {
        matches!(
            self,
            Quality::Dominant7
                | Quality::Dominant9
                | Quality::Dominant11
                | Quality::Dominant13
                | Quality::Dom7sus4
                | Quality::Dom7sus2
                | Quality::Dom7b9
                | Quality::Dom7Sharp9
                | Quality::Dom7Sharp11
                | Quality::Dom7b13
                | Quality::Dom7Alt
        )
    }

    /// Returns true if this quality has a minor third
    pub fn is_minor(&self) -> bool {
        matches!(
            self,
            Quality::Minor
                | Quality::Diminished
                | Quality::Minor7
                | Quality::Diminished7
                | Quality::HalfDim7
                | Quality::MinorMajor7
                | Quality::Minor6
                | Quality::Minor9
                | Quality::Minor11
                | Quality::Minor13
                | Quality::MinorMajor9
        )
    }

    pub fn all_triads() -> &'static [Quality] {
        &[
            Quality::Major,
            Quality::Minor,
            Quality::Diminished,
            Quality::Augmented,
            Quality::Sus2,
            Quality::Sus4,
        ]
    }

    pub fn all_sevenths() -> &'static [Quality] {
        &[
            Quality::Major7,
            Quality::Minor7,
            Quality::Dominant7,
            Quality::Diminished7,
            Quality::HalfDim7,
            Quality::MinorMajor7,
            Quality::Augmented7,
            Quality::Dom7sus4,
            Quality::Dom7sus2,
        ]
    }

    pub fn all_extended() -> &'static [Quality] {
        &[
            Quality::Major6,
            Quality::Minor6,
            Quality::Add9,
            Quality::Add11,
            Quality::Major9,
            Quality::Minor9,
            Quality::Dominant9,
            Quality::MinorMajor9,
            Quality::Major11,
            Quality::Minor11,
            Quality::Dominant11,
            Quality::Major13,
            Quality::Minor13,
            Quality::Dominant13,
        ]
    }

    pub fn all_altered() -> &'static [Quality] {
        &[
            Quality::Dom7b9,
            Quality::Dom7Sharp9,
            Quality::Dom7Sharp11,
            Quality::Dom7b13,
            Quality::Dom7Alt,
            Quality::Maj7Sharp11,
        ]
    }

    /// Returns all qualities in detection priority order (more specific first)
    pub fn all_for_detection() -> &'static [Quality] {
        &[
            // 13th chords first (most notes)
            Quality::Major13,
            Quality::Minor13,
            Quality::Dominant13,
            // 11th chords
            Quality::Major11,
            Quality::Minor11,
            Quality::Dominant11,
            // 9th chords
            Quality::Major9,
            Quality::Minor9,
            Quality::Dominant9,
            Quality::MinorMajor9,
            // Altered dominants
            Quality::Dom7Alt,
            Quality::Dom7b9,
            Quality::Dom7Sharp9,
            Quality::Dom7Sharp11,
            Quality::Dom7b13,
            // Lydian
            Quality::Maj7Sharp11,
            // 7th chords
            Quality::Major7,
            Quality::Minor7,
            Quality::Dominant7,
            Quality::Diminished7,
            Quality::HalfDim7,
            Quality::MinorMajor7,
            Quality::Augmented7,
            Quality::Dom7sus4,
            Quality::Dom7sus2,
            // 6th chords
            Quality::Major6,
            Quality::Minor6,
            // Add chords
            Quality::Add9,
            Quality::Add11,
            // Triads last
            Quality::Major,
            Quality::Minor,
            Quality::Diminished,
            Quality::Augmented,
            Quality::Sus2,
            Quality::Sus4,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_intervals() {
        assert_eq!(Quality::Major.intervals(), &[0, 4, 7]);
        assert_eq!(Quality::Minor.intervals(), &[0, 3, 7]);
        assert_eq!(Quality::Diminished.intervals(), &[0, 3, 6]);
        assert_eq!(Quality::Augmented.intervals(), &[0, 4, 8]);
        assert_eq!(Quality::Dominant7.intervals(), &[0, 4, 7, 10]);
        assert_eq!(Quality::Major7.intervals(), &[0, 4, 7, 11]);
        assert_eq!(Quality::Minor7.intervals(), &[0, 3, 7, 10]);
    }

    #[test]
    fn test_quality_symbol() {
        assert_eq!(Quality::Major.symbol(), "");
        assert_eq!(Quality::Minor.symbol(), "m");
        assert_eq!(Quality::Diminished.symbol(), "dim");
        assert_eq!(Quality::Augmented.symbol(), "+");
        assert_eq!(Quality::Dominant7.symbol(), "7");
        assert_eq!(Quality::Major7.symbol(), "maj7");
        assert_eq!(Quality::Minor7.symbol(), "m7");
        assert_eq!(Quality::HalfDim7.symbol(), "m7b5");
    }

    #[test]
    fn test_extended_chord_intervals() {
        assert_eq!(Quality::Dominant9.intervals(), &[0, 4, 7, 10, 14]);
        assert_eq!(Quality::Minor11.intervals(), &[0, 3, 7, 10, 14, 17]);
        assert_eq!(Quality::Major13.intervals(), &[0, 4, 7, 11, 14, 21]);
    }

    #[test]
    fn test_altered_chord_intervals() {
        assert_eq!(Quality::Dom7b9.intervals(), &[0, 4, 7, 10, 13]);
        assert_eq!(Quality::Dom7Sharp9.intervals(), &[0, 4, 7, 10, 15]);
    }

    #[test]
    fn test_is_dominant() {
        assert!(Quality::Dominant7.is_dominant());
        assert!(Quality::Dom7b9.is_dominant());
        assert!(Quality::Dominant13.is_dominant());
        assert!(!Quality::Major7.is_dominant());
        assert!(!Quality::Minor7.is_dominant());
    }

    #[test]
    fn test_is_minor() {
        assert!(Quality::Minor.is_minor());
        assert!(Quality::Minor7.is_minor());
        assert!(Quality::Minor9.is_minor());
        assert!(!Quality::Major.is_minor());
        assert!(!Quality::Dominant7.is_minor());
    }

    #[test]
    fn test_essential_intervals_subset() {
        // Essential intervals should be a subset of full intervals
        for quality in Quality::all_for_detection() {
            let full = quality.intervals();
            let essential = quality.essential_intervals();
            for interval in essential {
                assert!(
                    full.contains(interval),
                    "{:?} essential interval {} not in full intervals {:?}",
                    quality,
                    interval,
                    full
                );
            }
        }
    }
}
