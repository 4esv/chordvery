#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Major7,
    Minor7,
    Dominant7,
    Diminished7,
    HalfDim7,
    MinorMajor7,
    Augmented7,
    Sus2,
    Sus4,
    Add9,
    Unknown,
}

impl Quality {
    pub fn symbol(&self) -> &'static str {
        match self {
            Quality::Major => "",
            Quality::Minor => "m",
            Quality::Diminished => "dim",
            Quality::Augmented => "+",
            Quality::Major7 => "maj7",
            Quality::Minor7 => "m7",
            Quality::Dominant7 => "7",
            Quality::Diminished7 => "dim7",
            Quality::HalfDim7 => "m7b5",
            Quality::MinorMajor7 => "mMaj7",
            Quality::Augmented7 => "+7",
            Quality::Sus2 => "sus2",
            Quality::Sus4 => "sus4",
            Quality::Add9 => "add9",
            Quality::Unknown => "?",
        }
    }

    pub fn intervals(&self) -> &'static [u8] {
        match self {
            Quality::Major => &[0, 4, 7],
            Quality::Minor => &[0, 3, 7],
            Quality::Diminished => &[0, 3, 6],
            Quality::Augmented => &[0, 4, 8],
            Quality::Major7 => &[0, 4, 7, 11],
            Quality::Minor7 => &[0, 3, 7, 10],
            Quality::Dominant7 => &[0, 4, 7, 10],
            Quality::Diminished7 => &[0, 3, 6, 9],
            Quality::HalfDim7 => &[0, 3, 6, 10],
            Quality::MinorMajor7 => &[0, 3, 7, 11],
            Quality::Augmented7 => &[0, 4, 8, 10],
            Quality::Sus2 => &[0, 2, 7],
            Quality::Sus4 => &[0, 5, 7],
            Quality::Add9 => &[0, 4, 7, 14],
            Quality::Unknown => &[],
        }
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
}
