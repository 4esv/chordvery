use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};

use crate::theory::Chord;
use crate::ui::theme::Theme;

#[derive(Clone)]
pub struct ChordEntry {
    pub chord: Chord,
    pub age: u8,
}

pub struct ChordHistory {
    entries: Vec<ChordEntry>,
    max_entries: usize,
    fade: bool,
}

impl Default for ChordHistory {
    fn default() -> Self {
        Self::new(16)
    }
}

impl ChordHistory {
    pub fn new(max: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries: max,
            fade: false,
        }
    }

    pub fn push(&mut self, chord: Chord) {
        if let Some(last) = self.entries.last() {
            if last.chord.name() == chord.name() {
                return;
            }
        }

        for entry in &mut self.entries {
            entry.age = entry.age.saturating_add(1);
        }

        self.entries.push(ChordEntry { chord, age: 0 });

        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn set_fade(&mut self, fade: bool) {
        self.fade = fade;
    }

    pub fn tick(&mut self) {
        if self.fade {
            self.entries.retain(|e| e.age < 8);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn entries(&self) -> &[ChordEntry] {
        &self.entries
    }
}

impl Widget for &ChordHistory {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 5 || area.height < 1 {
            return;
        }

        if self.entries.is_empty() {
            let line = Line::from(vec![Span::styled("No chords yet...", Theme::text_dim())]);
            buf.set_line(area.x + 1, area.y + area.height / 2, &line, area.width);
            return;
        }

        let mut spans: Vec<Span> = Vec::new();

        for (i, entry) in self.entries.iter().enumerate() {
            let style = if self.fade {
                Theme::chord_history(entry.age)
            } else {
                Theme::chord_name()
            };

            spans.push(Span::styled(entry.chord.name(), style));

            if i < self.entries.len() - 1 {
                spans.push(Span::styled(" → ", Theme::text_dim()));
            }
        }

        let line = Line::from(spans);
        let y = area.y + area.height / 2;
        buf.set_line(area.x + 1, y, &line, area.width.saturating_sub(2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theory::{Note, Quality};

    #[test]
    fn test_push_and_age() {
        let mut history = ChordHistory::new(10);

        history.push(Chord::new(Note::new(60), Quality::Major));
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries[0].age, 0);

        history.push(Chord::new(Note::new(69), Quality::Minor));
        assert_eq!(history.entries.len(), 2);
        assert_eq!(history.entries[0].age, 1);
        assert_eq!(history.entries[1].age, 0);

        history.push(Chord::new(Note::new(65), Quality::Major));
        assert_eq!(history.entries[0].age, 2);
        assert_eq!(history.entries[1].age, 1);
        assert_eq!(history.entries[2].age, 0);
    }

    #[test]
    fn test_max_entries() {
        let mut history = ChordHistory::new(3);

        history.push(Chord::new(Note::new(60), Quality::Major));
        history.push(Chord::new(Note::new(62), Quality::Minor));
        history.push(Chord::new(Note::new(64), Quality::Major));
        history.push(Chord::new(Note::new(65), Quality::Major));

        assert_eq!(history.entries.len(), 3);
        assert_eq!(history.entries[0].chord.root.name(), "D");
    }

    #[test]
    fn test_no_duplicate_consecutive() {
        let mut history = ChordHistory::new(10);

        history.push(Chord::new(Note::new(60), Quality::Major));
        history.push(Chord::new(Note::new(60), Quality::Major));
        history.push(Chord::new(Note::new(60), Quality::Major));

        assert_eq!(history.entries.len(), 1);
    }

    #[test]
    fn test_fade_tick() {
        let mut history = ChordHistory::new(10);
        history.set_fade(true);

        history.push(Chord::new(Note::new(60), Quality::Major));

        for _ in 0..10 {
            history.push(Chord::new(Note::new(62), Quality::Minor));
            history.tick();
        }

        assert!(history.entries.iter().all(|e| e.age < 8));
    }
}
