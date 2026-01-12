const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Note {
    pub midi: u8,
}

impl Note {
    pub fn new(midi: u8) -> Self {
        Self { midi }
    }

    pub fn name(&self) -> &'static str {
        NOTE_NAMES[self.pitch_class() as usize]
    }

    pub fn octave(&self) -> i8 {
        (self.midi as i8 / 12) - 1
    }

    pub fn display(&self) -> String {
        format!("{}{}", self.name(), self.octave())
    }

    pub fn pitch_class(&self) -> u8 {
        self.midi % 12
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim();
        if name.is_empty() {
            return None;
        }

        let (note_part, octave_part) = if name.len() >= 2 && name.chars().nth(1) == Some('#') {
            (&name[..2], &name[2..])
        } else {
            (&name[..1], &name[1..])
        };

        let pitch_class = NOTE_NAMES.iter().position(|&n| n == note_part)? as u8;
        let octave: i8 = octave_part.parse().ok()?;
        let midi = ((octave + 1) * 12) as u8 + pitch_class;

        Some(Self { midi })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_to_name() {
        assert_eq!(Note::new(60).name(), "C");
        assert_eq!(Note::new(61).name(), "C#");
        assert_eq!(Note::new(62).name(), "D");
        assert_eq!(Note::new(63).name(), "D#");
        assert_eq!(Note::new(64).name(), "E");
        assert_eq!(Note::new(65).name(), "F");
        assert_eq!(Note::new(66).name(), "F#");
        assert_eq!(Note::new(67).name(), "G");
        assert_eq!(Note::new(68).name(), "G#");
        assert_eq!(Note::new(69).name(), "A");
        assert_eq!(Note::new(70).name(), "A#");
        assert_eq!(Note::new(71).name(), "B");
    }

    #[test]
    fn test_octave() {
        assert_eq!(Note::new(60).octave(), 4); // C4
        assert_eq!(Note::new(21).octave(), 0); // A0
        assert_eq!(Note::new(108).octave(), 8); // C8
        assert_eq!(Note::new(0).octave(), -1); // C-1
    }

    #[test]
    fn test_from_name() {
        assert_eq!(Note::from_name("C4"), Some(Note::new(60)));
        assert_eq!(Note::from_name("A0"), Some(Note::new(21)));
        assert_eq!(Note::from_name("C#4"), Some(Note::new(61)));
        assert_eq!(Note::from_name("F#3"), Some(Note::new(54)));
        assert_eq!(Note::from_name(""), None);
        assert_eq!(Note::from_name("X4"), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(Note::new(60).display(), "C4");
        assert_eq!(Note::new(69).display(), "A4");
        assert_eq!(Note::new(61).display(), "C#4");
    }

    #[test]
    fn test_pitch_class() {
        assert_eq!(Note::new(60).pitch_class(), 0); // C
        assert_eq!(Note::new(72).pitch_class(), 0); // C (octave up)
        assert_eq!(Note::new(69).pitch_class(), 9); // A
    }
}
