use ratatui::style::{Color, Modifier, Style};

pub struct Theme;

impl Theme {
    pub fn white_key() -> Style {
        Style::default().fg(Color::Black).bg(Color::White)
    }

    pub fn white_key_pressed() -> Style {
        Style::default().fg(Color::White).bg(Color::Blue)
    }

    pub fn white_key_root() -> Style {
        Style::default().fg(Color::White).bg(Color::Magenta)
    }

    pub fn black_key() -> Style {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    }

    pub fn black_key_pressed() -> Style {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    }

    pub fn black_key_root() -> Style {
        Style::default().fg(Color::Black).bg(Color::Magenta)
    }

    pub fn border() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn border_focused() -> Style {
        Style::default().fg(Color::Cyan)
    }

    pub fn title() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    pub fn text() -> Style {
        Style::default().fg(Color::White)
    }

    pub fn text_dim() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn chord_name() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    pub fn chord_history(age: u8) -> Style {
        let color = match age {
            0 => Color::Yellow,
            1 => Color::White,
            2 => Color::Gray,
            _ => Color::DarkGray,
        };
        Style::default().fg(color)
    }

    pub fn tree_current() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    pub fn tree_expected() -> Style {
        Style::default().fg(Color::Green)
    }

    pub fn tree_surprise() -> Style {
        Style::default().fg(Color::Magenta)
    }

    pub fn tree_connector() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn mode_discovery() -> Style {
        Style::default().fg(Color::Cyan)
    }

    pub fn mode_jam() -> Style {
        Style::default().fg(Color::Magenta)
    }

    pub fn status_bar() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn help_key() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    pub fn help_text() -> Style {
        Style::default().fg(Color::White)
    }
}
