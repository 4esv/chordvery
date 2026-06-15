//! Chord substitution engine for jazz harmony.
//!
//! Provides substitution suggestions for chords based on common jazz practices:
//! - Tritone substitution
//! - Backdoor resolution
//! - Screendoor (ii-V of tritone sub)
//! - Relative/parallel major-minor
//! - Secondary dominants
//! - Diminished substitution
//! - Chromatic mediants

use super::chord::Chord;
use super::note::Note;
use super::quality::Quality;

/// Category of chord substitution
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubCategory {
    /// Functional substitutions - serve same harmonic function
    Functional,
    /// Color substitutions - add harmonic color/interest
    Color,
    /// Voice leading substitutions - smooth voice leading
    Voice,
}

/// A chord substitution with explanation
#[derive(Clone, Debug)]
pub struct Substitution {
    /// Short name for the substitution type
    pub name: &'static str,
    /// The substitute chord
    pub chord: Chord,
    /// Brief explanation of why this substitution works
    pub explanation: &'static str,
    /// Category of substitution
    pub category: SubCategory,
}

impl Substitution {
    fn new(name: &'static str, chord: Chord, explanation: &'static str, category: SubCategory) -> Self {
        Self {
            name,
            chord,
            explanation,
            category,
        }
    }
}

/// Get all substitutions for a chord in a given key context
pub fn substitutions_for(chord: &Chord, key: Note) -> Vec<Substitution> {
    let mut subs = Vec::new();

    // Tritone substitution - for dominant chords
    if chord.quality.is_dominant() {
        if let Some(sub) = tritone_sub(chord) {
            subs.push(sub);
        }
    }

    // Backdoor resolution - bVII7 can resolve to I
    if let Some(sub) = backdoor(chord, key) {
        subs.push(sub);
    }

    // Screendoor - ii-V of tritone sub
    if chord.quality.is_dominant() {
        if let Some(sub) = screendoor(chord) {
            subs.push(sub);
        }
    }

    // Relative major/minor
    if let Some(sub) = relative(chord) {
        subs.push(sub);
    }

    // Parallel major/minor
    if let Some(sub) = parallel(chord) {
        subs.push(sub);
    }

    // Secondary dominant
    if let Some(sub) = secondary_dominant(chord, key) {
        subs.push(sub);
    }

    // Diminished substitution
    if chord.quality.is_dominant() {
        if let Some(sub) = diminished_sub(chord) {
            subs.push(sub);
        }
    }

    // Chromatic mediants
    for sub in chromatic_mediants(chord) {
        subs.push(sub);
    }

    subs
}

/// Tritone substitution: dominant chord a tritone away shares b7 and 3
/// G7 -> Db7 (both have B/Cb and F)
fn tritone_sub(chord: &Chord) -> Option<Substitution> {
    if !chord.quality.is_dominant() {
        return None;
    }

    let tritone_root = (chord.root.pitch_class() + 6) % 12;
    let sub_chord = Chord::new(Note::new(tritone_root + 60), chord.quality);

    Some(Substitution::new(
        "tritone",
        sub_chord,
        "shares 3 and b7 (enharmonic)",
        SubCategory::Functional,
    ))
}

/// Backdoor resolution: bVII7 resolving to I
/// For a V7 chord, suggest using bVII7 instead for a "backdoor" feel
fn backdoor(chord: &Chord, key: Note) -> Option<Substitution> {
    // Check if this chord could be a V7 (dominant)
    if !chord.quality.is_dominant() {
        return None;
    }

    // V is 7 semitones above I
    let expected_v_root = (key.pitch_class() + 7) % 12;
    if chord.root.pitch_class() != expected_v_root {
        return None;
    }

    // bVII is 10 semitones above I
    let bvii_root = (key.pitch_class() + 10) % 12;
    let sub_chord = Chord::new(Note::new(bvii_root + 60), Quality::Dominant7);

    Some(Substitution::new(
        "backdoor",
        sub_chord,
        "bVII7 resolves to I with smooth voice leading",
        SubCategory::Functional,
    ))
}

/// Screendoor: ii-V of the tritone sub
/// For G7 -> C, tritone sub is Db7. Screendoor adds Abm7 -> Db7
fn screendoor(chord: &Chord) -> Option<Substitution> {
    if !chord.quality.is_dominant() {
        return None;
    }

    // Tritone root
    let tritone_root = (chord.root.pitch_class() + 6) % 12;
    // ii of tritone is a 4th below (or 5th above) = -5 semitones
    let ii_root = (tritone_root + 12 - 5) % 12;

    let sub_chord = Chord::new(Note::new(ii_root + 60), Quality::Minor7);

    Some(Substitution::new(
        "screendoor",
        sub_chord,
        "ii of tritone sub (prepares tritone resolution)",
        SubCategory::Functional,
    ))
}

/// Relative major/minor: Am <-> C
fn relative(chord: &Chord) -> Option<Substitution> {
    match chord.quality {
        Quality::Major | Quality::Major7 | Quality::Major6 => {
            // Relative minor is 3 semitones below
            let rel_root = (chord.root.pitch_class() + 12 - 3) % 12;
            let rel_quality = match chord.quality {
                Quality::Major7 => Quality::Minor7,
                Quality::Major6 => Quality::Minor7,
                _ => Quality::Minor,
            };
            Some(Substitution::new(
                "relative",
                Chord::new(Note::new(rel_root + 60), rel_quality),
                "relative minor shares same key signature",
                SubCategory::Color,
            ))
        }
        Quality::Minor | Quality::Minor7 | Quality::Minor6 => {
            // Relative major is 3 semitones above
            let rel_root = (chord.root.pitch_class() + 3) % 12;
            let rel_quality = match chord.quality {
                Quality::Minor7 => Quality::Major7,
                Quality::Minor6 => Quality::Major6,
                _ => Quality::Major,
            };
            Some(Substitution::new(
                "relative",
                Chord::new(Note::new(rel_root + 60), rel_quality),
                "relative major shares same key signature",
                SubCategory::Color,
            ))
        }
        _ => None,
    }
}

/// Parallel major/minor: Cm <-> C
fn parallel(chord: &Chord) -> Option<Substitution> {
    let (new_quality, explanation) = match chord.quality {
        Quality::Major => (Quality::Minor, "parallel minor - same root, darker color"),
        Quality::Minor => (Quality::Major, "parallel major - same root, brighter color"),
        Quality::Major7 => (Quality::Minor7, "parallel minor - same root, darker 7th"),
        Quality::Minor7 => (Quality::Major7, "parallel major - same root, brighter 7th"),
        Quality::Major9 => (Quality::Minor9, "parallel minor 9th"),
        Quality::Minor9 => (Quality::Major9, "parallel major 9th"),
        _ => return None,
    };

    Some(Substitution::new(
        "parallel",
        Chord::new(chord.root.clone(), new_quality),
        explanation,
        SubCategory::Color,
    ))
}

/// Secondary dominant: V7/X - the dominant that resolves to the current chord
fn secondary_dominant(chord: &Chord, key: Note) -> Option<Substitution> {
    // Secondary dominant is a 5th above (7 semitones)
    let v_root = (chord.root.pitch_class() + 7) % 12;

    // Don't suggest if it's already V7 of the key
    if v_root == (key.pitch_class() + 7) % 12 {
        return None;
    }

    // Don't suggest for diminished or augmented chords
    if matches!(
        chord.quality,
        Quality::Diminished | Quality::Diminished7 | Quality::Augmented | Quality::Augmented7
    ) {
        return None;
    }

    Some(Substitution::new(
        "secondary V",
        Chord::new(Note::new(v_root + 60), Quality::Dominant7),
        "secondary dominant - V7 that resolves to this chord",
        SubCategory::Functional,
    ))
}

/// Diminished substitution: dim7 shares 3 notes with dom7b9
/// Bdim7 ≈ G7b9 (both contain B, D, F, Ab)
fn diminished_sub(chord: &Chord) -> Option<Substitution> {
    if !chord.quality.is_dominant() {
        return None;
    }

    // The dim7 built on the 3rd of a dom7 shares notes with dom7b9
    // G7b9 = G B D F Ab, Bdim7 = B D F Ab
    let dim_root = (chord.root.pitch_class() + 4) % 12; // major 3rd above

    Some(Substitution::new(
        "dim sub",
        Chord::new(Note::new(dim_root + 60), Quality::Diminished7),
        "shares tones with dom7b9 (rootless voicing)",
        SubCategory::Voice,
    ))
}

/// Chromatic mediants: major chords a major/minor 3rd away
/// C -> E (chromatic mediant), C -> Ab (chromatic mediant)
fn chromatic_mediants(chord: &Chord) -> Vec<Substitution> {
    let mut subs = Vec::new();

    // Only for major chords
    if !matches!(chord.quality, Quality::Major | Quality::Major7) {
        return subs;
    }

    let quality = match chord.quality {
        Quality::Major7 => Quality::Major7,
        _ => Quality::Major,
    };

    // Upper chromatic mediant (major 3rd up)
    let upper_root = (chord.root.pitch_class() + 4) % 12;
    subs.push(Substitution::new(
        "upper mediant",
        Chord::new(Note::new(upper_root + 60), quality),
        "chromatic mediant - dramatic color shift",
        SubCategory::Color,
    ));

    // Lower chromatic mediant (major 3rd down)
    let lower_root = (chord.root.pitch_class() + 12 - 4) % 12;
    subs.push(Substitution::new(
        "lower mediant",
        Chord::new(Note::new(lower_root + 60), quality),
        "chromatic mediant - dramatic color shift",
        SubCategory::Color,
    ));

    subs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c_note() -> Note {
        Note::new(60)
    }

    #[test]
    fn test_tritone_sub() {
        // G7 -> Db7 (displayed as C#7 enharmonically)
        let g7 = Chord::new(Note::new(67), Quality::Dominant7);
        let sub = tritone_sub(&g7).unwrap();
        assert_eq!(sub.chord.root.name(), "C#"); // Db enharmonic
        assert_eq!(sub.chord.quality, Quality::Dominant7);
        assert_eq!(sub.name, "tritone");
    }

    #[test]
    fn test_tritone_sub_non_dominant() {
        let c_maj = Chord::new(c_note(), Quality::Major);
        assert!(tritone_sub(&c_maj).is_none());
    }

    #[test]
    fn test_backdoor() {
        // G7 in key of C -> Bb7 (backdoor)
        let g7 = Chord::new(Note::new(67), Quality::Dominant7);
        let sub = backdoor(&g7, c_note()).unwrap();
        assert_eq!(sub.chord.root.name(), "A#"); // Bb enharmonic
        assert_eq!(sub.chord.quality, Quality::Dominant7);
        assert_eq!(sub.name, "backdoor");
    }

    #[test]
    fn test_screendoor() {
        // G7 -> Abm7 (ii of Db, the tritone sub)
        let g7 = Chord::new(Note::new(67), Quality::Dominant7);
        let sub = screendoor(&g7).unwrap();
        assert_eq!(sub.chord.root.name(), "G#"); // Ab enharmonic
        assert_eq!(sub.chord.quality, Quality::Minor7);
        assert_eq!(sub.name, "screendoor");
    }

    #[test]
    fn test_relative_major_minor() {
        // C major -> A minor
        let c_maj = Chord::new(c_note(), Quality::Major);
        let sub = relative(&c_maj).unwrap();
        assert_eq!(sub.chord.root.name(), "A");
        assert_eq!(sub.chord.quality, Quality::Minor);

        // Am -> C
        let a_min = Chord::new(Note::new(69), Quality::Minor);
        let sub = relative(&a_min).unwrap();
        assert_eq!(sub.chord.root.name(), "C");
        assert_eq!(sub.chord.quality, Quality::Major);
    }

    #[test]
    fn test_parallel() {
        // C major -> C minor
        let c_maj = Chord::new(c_note(), Quality::Major);
        let sub = parallel(&c_maj).unwrap();
        assert_eq!(sub.chord.root.name(), "C");
        assert_eq!(sub.chord.quality, Quality::Minor);

        // Cm7 -> Cmaj7
        let c_min7 = Chord::new(c_note(), Quality::Minor7);
        let sub = parallel(&c_min7).unwrap();
        assert_eq!(sub.chord.root.name(), "C");
        assert_eq!(sub.chord.quality, Quality::Major7);
    }

    #[test]
    fn test_secondary_dominant() {
        // For Dm in key of C, secondary dom is A7
        let dm = Chord::new(Note::new(62), Quality::Minor);
        let sub = secondary_dominant(&dm, c_note()).unwrap();
        assert_eq!(sub.chord.root.name(), "A");
        assert_eq!(sub.chord.quality, Quality::Dominant7);
    }

    #[test]
    fn test_diminished_sub() {
        // G7 -> Bdim7
        let g7 = Chord::new(Note::new(67), Quality::Dominant7);
        let sub = diminished_sub(&g7).unwrap();
        assert_eq!(sub.chord.root.name(), "B");
        assert_eq!(sub.chord.quality, Quality::Diminished7);
    }

    #[test]
    fn test_chromatic_mediants() {
        // C major -> E major, Ab major
        let c_maj = Chord::new(c_note(), Quality::Major);
        let subs = chromatic_mediants(&c_maj);
        assert_eq!(subs.len(), 2);

        let roots: Vec<&str> = subs.iter().map(|s| s.chord.root.name()).collect();
        assert!(roots.contains(&"E"));
        assert!(roots.contains(&"G#")); // Ab enharmonic
    }

    #[test]
    fn test_substitutions_for_dominant() {
        // G7 in key of C should get multiple substitutions
        let g7 = Chord::new(Note::new(67), Quality::Dominant7);
        let subs = substitutions_for(&g7, c_note());

        let names: Vec<&str> = subs.iter().map(|s| s.name).collect();
        assert!(names.contains(&"tritone"));
        assert!(names.contains(&"backdoor"));
        assert!(names.contains(&"screendoor"));
        assert!(names.contains(&"dim sub"));
    }

    #[test]
    fn test_substitutions_for_major() {
        // C major in key of C
        let c_maj = Chord::new(c_note(), Quality::Major);
        let subs = substitutions_for(&c_maj, c_note());

        let names: Vec<&str> = subs.iter().map(|s| s.name).collect();
        assert!(names.contains(&"relative"));
        assert!(names.contains(&"parallel"));
        assert!(names.contains(&"upper mediant"));
        assert!(names.contains(&"lower mediant"));
    }
}
