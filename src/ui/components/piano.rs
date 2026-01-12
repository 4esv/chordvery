use std::collections::HashSet;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::ui::theme::Theme;

const BLACK_KEY_PATTERN: [bool; 12] = [
    false, true, false, true, false, false, true, false, true, false, true, false,
];

pub struct Piano {
    start_midi: u8,
    num_keys: usize,
    pressed: HashSet<u8>,
    root: Option<u8>,
}

impl Piano {
    pub fn new(start_midi: u8, num_keys: usize) -> Self {
        Self {
            start_midi,
            num_keys,
            pressed: HashSet::new(),
            root: None,
        }
    }

    pub fn dynamic(pressed: &HashSet<u8>) -> Self {
        if pressed.is_empty() {
            return Self::new(48, 25);
        }

        let min = *pressed.iter().min().unwrap();
        let max = *pressed.iter().max().unwrap();

        let start = (min.saturating_sub(5) / 12) * 12;
        let end = ((max + 7) / 12 + 1) * 12;
        let num_keys = (end - start).max(25) as usize;

        Self {
            start_midi: start,
            num_keys,
            pressed: pressed.clone(),
            root: None,
        }
    }

    pub fn pressed(mut self, keys: HashSet<u8>) -> Self {
        self.pressed = keys;
        self
    }

    pub fn root(mut self, midi: Option<u8>) -> Self {
        self.root = midi;
        self
    }

    fn is_black_key(midi: u8) -> bool {
        BLACK_KEY_PATTERN[(midi % 12) as usize]
    }

    fn white_key_count(&self) -> usize {
        (self.start_midi..self.start_midi + self.num_keys as u8)
            .filter(|&m| !Self::is_black_key(m))
            .count()
    }
}

impl Widget for Piano {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 4 || area.width < 10 {
            return;
        }

        let white_keys = self.white_key_count();
        let key_width = (area.width as usize / white_keys).max(2);
        let black_key_width = key_width.saturating_sub(1).max(1);

        let piano_height = area.height.min(6);
        let black_key_height = (piano_height * 3 / 5).max(2);

        let mut white_key_x = area.x;

        for midi in self.start_midi..self.start_midi + self.num_keys as u8 {
            if Self::is_black_key(midi) {
                continue;
            }

            let is_pressed = self.pressed.contains(&midi);
            let is_root = self.root == Some(midi);

            let style = if is_root {
                Theme::white_key_root()
            } else if is_pressed {
                Theme::white_key_pressed()
            } else {
                Theme::white_key()
            };

            for y in area.y..area.y + piano_height {
                for x in white_key_x..white_key_x + key_width as u16 {
                    if x < area.x + area.width {
                        buf.set_string(x, y, " ", style);
                    }
                }
            }

            if white_key_x + key_width as u16 <= area.x + area.width {
                for y in area.y..area.y + piano_height {
                    buf.set_string(white_key_x + key_width as u16 - 1, y, "│", Theme::border());
                }
            }

            white_key_x += key_width as u16;
        }

        white_key_x = area.x;

        for midi in self.start_midi..self.start_midi + self.num_keys as u8 {
            if Self::is_black_key(midi) {
                continue;
            }

            let next_midi = midi + 1;
            if next_midi < self.start_midi + self.num_keys as u8 && Self::is_black_key(next_midi) {
                let black_x = white_key_x + key_width as u16 - (black_key_width as u16 / 2) - 1;

                let is_pressed = self.pressed.contains(&next_midi);
                let is_root = self.root == Some(next_midi);

                let style = if is_root {
                    Theme::black_key_root()
                } else if is_pressed {
                    Theme::black_key_pressed()
                } else {
                    Theme::black_key()
                };

                for y in area.y..area.y + black_key_height {
                    for x in black_x..black_x + black_key_width as u16 {
                        if x < area.x + area.width && x >= area.x {
                            buf.set_string(x, y, " ", style);
                        }
                    }
                }
            }

            white_key_x += key_width as u16;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_octave_off() {
        let piano = Piano::new(60, 12);
        let area = Rect::new(0, 0, 40, 6);
        let mut buf = Buffer::empty(area);

        piano.render(area, &mut buf);

        let content = buf.content.iter().any(|c| c.symbol() == " ");
        assert!(content);
    }

    #[test]
    fn test_render_octave_on() {
        let mut pressed = HashSet::new();
        pressed.insert(60);
        pressed.insert(64);
        pressed.insert(67);

        let piano = Piano::new(60, 12).pressed(pressed);
        let area = Rect::new(0, 0, 40, 6);
        let mut buf = Buffer::empty(area);

        piano.render(area, &mut buf);

        let has_content = buf.content.iter().any(|c| !c.symbol().is_empty());
        assert!(has_content);
    }

    #[test]
    fn test_dynamic_range() {
        let mut pressed = HashSet::new();
        pressed.insert(36);
        pressed.insert(84);

        let piano = Piano::dynamic(&pressed);

        assert!(piano.start_midi <= 36);
        assert!(piano.start_midi + piano.num_keys as u8 >= 84);
    }

    #[test]
    fn test_dynamic_range_empty() {
        let pressed = HashSet::new();
        let piano = Piano::dynamic(&pressed);

        assert_eq!(piano.start_midi, 48);
        assert_eq!(piano.num_keys, 25);
    }

    #[test]
    fn test_is_black_key() {
        assert!(!Piano::is_black_key(60)); // C
        assert!(Piano::is_black_key(61)); // C#
        assert!(!Piano::is_black_key(62)); // D
        assert!(Piano::is_black_key(63)); // D#
        assert!(!Piano::is_black_key(64)); // E
        assert!(!Piano::is_black_key(65)); // F
        assert!(Piano::is_black_key(66)); // F#
    }
}
