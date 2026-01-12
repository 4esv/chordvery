use super::chord::Chord;
use super::note::Note;
use super::quality::Quality;

#[derive(Clone, Debug)]
pub struct ProgressionNode {
    pub chord: Chord,
    pub left: Option<Box<ProgressionNode>>,
    pub right: Option<Box<ProgressionNode>>,
}

impl ProgressionNode {
    pub fn new(chord: Chord) -> Self {
        Self {
            chord,
            left: None,
            right: None,
        }
    }

    pub fn with_children(mut self, left: ProgressionNode, right: ProgressionNode) -> Self {
        self.left = Some(Box::new(left));
        self.right = Some(Box::new(right));
        self
    }
}

pub struct ProgressionTree {
    extended_mode: bool,
}

impl Default for ProgressionTree {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressionTree {
    pub fn new() -> Self {
        Self {
            extended_mode: false,
        }
    }

    pub fn set_extended(&mut self, extended: bool) {
        self.extended_mode = extended;
    }

    pub fn suggest(&self, current: &Chord, key: Option<Note>) -> ProgressionNode {
        let key = key.unwrap_or(current.root);
        let degree = self.get_degree(current, key);

        let (left_chord, right_chord) = self.get_suggestions(degree, key, current);

        let left_left_right =
            self.get_suggestions(self.get_degree(&left_chord, key), key, &left_chord);
        let right_left_right =
            self.get_suggestions(self.get_degree(&right_chord, key), key, &right_chord);

        let left_node = ProgressionNode::new(left_chord.clone()).with_children(
            ProgressionNode::new(left_left_right.0),
            ProgressionNode::new(left_left_right.1),
        );

        let right_node = ProgressionNode::new(right_chord.clone()).with_children(
            ProgressionNode::new(right_left_right.0),
            ProgressionNode::new(right_left_right.1),
        );

        ProgressionNode::new(current.clone()).with_children(left_node, right_node)
    }

    fn get_degree(&self, chord: &Chord, key: Note) -> u8 {
        (chord.root.pitch_class() + 12 - key.pitch_class()) % 12
    }

    fn get_suggestions(&self, degree: u8, key: Note, _current: &Chord) -> (Chord, Chord) {
        let (left_interval, left_quality, right_interval, right_quality) = match degree {
            0 => (5, Quality::Major, 9, Quality::Minor), // I -> IV, vi
            2 => (7, Quality::Major, 5, Quality::Major), // ii -> V, IV
            4 => (9, Quality::Minor, 5, Quality::Major), // iii -> vi, IV
            5 => (7, Quality::Major, 0, Quality::Major), // IV -> V, I
            7 => (0, Quality::Major, 9, Quality::Minor), // V -> I, vi
            9 => (2, Quality::Minor, 5, Quality::Major), // vi -> ii, IV
            11 => (0, Quality::Major, 4, Quality::Minor), // vii° -> I, iii
            _ => (7, Quality::Major, 0, Quality::Major), // Default: V, I
        };

        let left_root = Note::new((key.pitch_class() + left_interval) % 12 + 60);
        let right_root = Note::new((key.pitch_class() + right_interval) % 12 + 60);

        let left_quality = if self.extended_mode {
            self.extend_quality(left_quality)
        } else {
            left_quality
        };

        let right_quality = if self.extended_mode {
            self.extend_quality(right_quality)
        } else {
            right_quality
        };

        (
            Chord::new(left_root, left_quality),
            Chord::new(right_root, right_quality),
        )
    }

    fn extend_quality(&self, quality: Quality) -> Quality {
        match quality {
            Quality::Major => Quality::Major7,
            Quality::Minor => Quality::Minor7,
            _ => quality,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggest_from_i() {
        let tree = ProgressionTree::new();
        let c_major = Chord::new(Note::new(60), Quality::Major);
        let key = Note::new(60);

        let result = tree.suggest(&c_major, Some(key));

        assert_eq!(result.chord.name(), "C");
        assert!(result.left.is_some());
        assert!(result.right.is_some());

        let left = result.left.unwrap();
        let right = result.right.unwrap();

        assert_eq!(left.chord.name(), "F");
        assert_eq!(right.chord.name(), "Am");
    }

    #[test]
    fn test_suggest_from_v() {
        let tree = ProgressionTree::new();
        let g_major = Chord::new(Note::new(67), Quality::Major);
        let key = Note::new(60);

        let result = tree.suggest(&g_major, Some(key));

        let left = result.left.unwrap();
        let right = result.right.unwrap();

        assert_eq!(left.chord.name(), "C");
        assert_eq!(right.chord.name(), "Am");
    }

    #[test]
    fn test_two_levels() {
        let tree = ProgressionTree::new();
        let c_major = Chord::new(Note::new(60), Quality::Major);
        let key = Note::new(60);

        let result = tree.suggest(&c_major, Some(key));

        assert!(result.left.is_some());
        assert!(result.right.is_some());

        let left = result.left.as_ref().unwrap();
        assert!(left.left.is_some());
        assert!(left.right.is_some());

        let right = result.right.as_ref().unwrap();
        assert!(right.left.is_some());
        assert!(right.right.is_some());
    }

    #[test]
    fn test_extended_mode() {
        let mut tree = ProgressionTree::new();
        tree.set_extended(true);

        let c_major = Chord::new(Note::new(60), Quality::Major);
        let key = Note::new(60);

        let result = tree.suggest(&c_major, Some(key));
        let left = result.left.unwrap();

        assert_eq!(left.chord.quality, Quality::Major7);
    }
}
