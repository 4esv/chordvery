use std::collections::HashSet;

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::midi::MidiInput;
use crate::theory::{Chord, Note, ProgressionTree};
use crate::ui::components::{ChordHistory, ChordTree, Piano};
use crate::ui::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Discovery,
    Jam,
}

impl Mode {
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Discovery => "Discovery",
            Mode::Jam => "Jam",
        }
    }
}

pub struct App {
    pub mode: Mode,
    pub midi: Option<MidiInput>,
    pub current_chord: Option<Chord>,
    pub history: ChordHistory,
    pub tree: ProgressionTree,
    pub should_quit: bool,
    pub extended_chords: bool,
    pub show_help: bool,
    key: Option<Note>,
    last_notes: HashSet<u8>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: Mode::Discovery,
            midi: None,
            current_chord: None,
            history: ChordHistory::new(16),
            tree: ProgressionTree::new(),
            should_quit: false,
            extended_chords: false,
            show_help: false,
            key: None,
            last_notes: HashSet::new(),
        }
    }

    pub fn connect_midi(&mut self) -> Result<()> {
        self.midi = Some(MidiInput::connect_first()?);
        Ok(())
    }

    pub fn connect_midi_port(&mut self, port: usize) -> Result<()> {
        self.midi = Some(MidiInput::connect(port)?);
        Ok(())
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Discovery => Mode::Jam,
            Mode::Jam => Mode::Discovery,
        };
        self.history.set_fade(self.mode == Mode::Jam);
    }

    pub fn toggle_extended(&mut self) {
        self.extended_chords = !self.extended_chords;
        self.tree.set_extended(self.extended_chords);
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn tick(&mut self) {
        let notes = self
            .midi
            .as_ref()
            .map(|m| m.held_notes())
            .unwrap_or_default();

        if notes != self.last_notes {
            self.last_notes = notes.clone();

            if let Some(chord) = Chord::detect(&notes) {
                if self.current_chord.as_ref().map(|c| c.name()) != Some(chord.name()) {
                    self.history.push(chord.clone());

                    if self.key.is_none() {
                        self.key = Some(chord.root);
                    }
                }
                self.current_chord = Some(chord);
            }
        }

        self.history.tick();
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Tab => self.toggle_mode(),
            KeyCode::Char('e') => self.toggle_extended(),
            KeyCode::Char('?') => self.toggle_help(),
            KeyCode::Char('c') => {
                self.history.clear();
                self.key = None;
            }
            _ => {}
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(10),
                Constraint::Length(8),
                Constraint::Length(1),
            ])
            .split(area);

        self.render_title(frame, main_layout[0]);

        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(main_layout[1]);

        self.render_tree(frame, content_layout[0]);
        self.render_history(frame, content_layout[1]);

        self.render_piano(frame, main_layout[2]);
        self.render_status(frame, main_layout[3]);

        if self.show_help {
            self.render_help_overlay(frame, area);
        }
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new(Line::from(vec![
            Span::styled(" Chordvery ", Theme::title()),
            Span::styled("─ Chord Discovery Tool", Theme::text_dim()),
        ]));
        frame.render_widget(title, area);
    }

    fn render_tree(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Suggestions ")
            .borders(Borders::ALL)
            .border_style(Theme::border());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if let Some(chord) = &self.current_chord {
            let node = self.tree.suggest(chord, self.key);
            let tree_widget = ChordTree::new().root(node);
            frame.render_widget(tree_widget, inner);
        } else {
            let tree_widget = ChordTree::new();
            frame.render_widget(tree_widget, inner);
        }
    }

    fn render_history(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" History ")
            .borders(Borders::ALL)
            .border_style(Theme::border());

        let inner = block.inner(area);
        frame.render_widget(block, area);
        frame.render_widget(&self.history, inner);
    }

    fn render_piano(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Piano ")
            .borders(Borders::ALL)
            .border_style(Theme::border());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let notes = self.last_notes.clone();
        let root = self.current_chord.as_ref().map(|c| c.root.midi);

        let piano = Piano::dynamic(&notes).pressed(notes).root(root);
        frame.render_widget(piano, inner);
    }

    fn render_status(&self, frame: &mut Frame, area: Rect) {
        let mode_style = match self.mode {
            Mode::Discovery => Theme::mode_discovery(),
            Mode::Jam => Theme::mode_jam(),
        };

        let chord_text = self
            .current_chord
            .as_ref()
            .map(|c| c.name())
            .unwrap_or_else(|| "—".to_string());

        let extended_text = if self.extended_chords { "ON" } else { "OFF" };

        let status = Line::from(vec![
            Span::styled(" [Tab] ", Theme::help_key()),
            Span::styled("Mode: ", Theme::status_bar()),
            Span::styled(self.mode.name(), mode_style),
            Span::styled(" │ ", Theme::status_bar()),
            Span::styled("Playing: ", Theme::status_bar()),
            Span::styled(&chord_text, Theme::chord_name()),
            Span::styled(" │ ", Theme::status_bar()),
            Span::styled("[e] ", Theme::help_key()),
            Span::styled("Extended: ", Theme::status_bar()),
            Span::styled(extended_text, Theme::text()),
            Span::styled(" │ ", Theme::status_bar()),
            Span::styled("[?] ", Theme::help_key()),
            Span::styled("Help", Theme::status_bar()),
        ]);

        let paragraph = Paragraph::new(status);
        frame.render_widget(paragraph, area);
    }

    fn render_help_overlay(&self, frame: &mut Frame, area: Rect) {
        let help_width = 40;
        let help_height = 12;
        let x = (area.width.saturating_sub(help_width)) / 2;
        let y = (area.height.saturating_sub(help_height)) / 2;

        let help_area = Rect::new(x, y, help_width, help_height);

        let help_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  Tab    ", Theme::help_key()),
                Span::styled("Toggle Discovery/Jam mode", Theme::help_text()),
            ]),
            Line::from(vec![
                Span::styled("  e      ", Theme::help_key()),
                Span::styled("Toggle extended chords", Theme::help_text()),
            ]),
            Line::from(vec![
                Span::styled("  c      ", Theme::help_key()),
                Span::styled("Clear history", Theme::help_text()),
            ]),
            Line::from(vec![
                Span::styled("  ?      ", Theme::help_key()),
                Span::styled("Toggle this help", Theme::help_text()),
            ]),
            Line::from(vec![
                Span::styled("  q/Esc  ", Theme::help_key()),
                Span::styled("Quit", Theme::help_text()),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "  Press any key to close",
                Theme::text_dim(),
            )]),
        ];

        let block = Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_style(Theme::border_focused());

        let help = Paragraph::new(help_text).block(block);
        frame.render_widget(help, help_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_toggle() {
        let mut app = App::new();
        assert_eq!(app.mode, Mode::Discovery);

        app.toggle_mode();
        assert_eq!(app.mode, Mode::Jam);

        app.toggle_mode();
        assert_eq!(app.mode, Mode::Discovery);
    }

    #[test]
    fn test_extended_toggle() {
        let mut app = App::new();
        assert!(!app.extended_chords);

        app.toggle_extended();
        assert!(app.extended_chords);

        app.toggle_extended();
        assert!(!app.extended_chords);
    }

    #[test]
    fn test_handle_key_quit() {
        let mut app = App::new();
        assert!(!app.should_quit);

        app.handle_key(KeyCode::Char('q'));
        assert!(app.should_quit);
    }

    #[test]
    fn test_handle_key_tab() {
        let mut app = App::new();
        assert_eq!(app.mode, Mode::Discovery);

        app.handle_key(KeyCode::Tab);
        assert_eq!(app.mode, Mode::Jam);
    }
}
