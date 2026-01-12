use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};

use crate::theory::ProgressionNode;
use crate::ui::theme::Theme;

pub struct ChordTree {
    root: Option<ProgressionNode>,
    depth: usize,
}

impl Default for ChordTree {
    fn default() -> Self {
        Self::new()
    }
}

impl ChordTree {
    pub fn new() -> Self {
        Self {
            root: None,
            depth: 2,
        }
    }

    pub fn root(mut self, node: ProgressionNode) -> Self {
        self.root = Some(node);
        self
    }

    pub fn depth(mut self, d: usize) -> Self {
        self.depth = d;
        self
    }

    fn render_tree(&self, area: Rect, buf: &mut Buffer) {
        let Some(node) = &self.root else {
            let line = Line::from(vec![Span::styled("Play a chord...", Theme::text_dim())]);
            buf.set_line(area.x + 1, area.y + area.height / 2, &line, area.width);
            return;
        };

        let center_y = area.y + area.height / 2;
        let col_width = area.width / 4;

        let current_x = area.x + 1;
        let current_name = node.chord.name();
        let line = Line::from(vec![Span::styled(&current_name, Theme::tree_current())]);
        buf.set_line(current_x, center_y, &line, col_width);

        let connector_x = current_x + current_name.len() as u16 + 1;
        buf.set_string(connector_x, center_y, "─┬─", Theme::tree_connector());

        if let Some(left) = &node.left {
            let left_y = center_y.saturating_sub(1);
            buf.set_string(connector_x + 1, left_y, "┌", Theme::tree_connector());
            buf.set_string(connector_x + 2, left_y, "─", Theme::tree_connector());

            let left_x = connector_x + 4;
            let left_name = left.chord.name();
            let line = Line::from(vec![Span::styled(&left_name, Theme::tree_expected())]);
            buf.set_line(left_x, left_y, &line, col_width);

            if let (Some(ll), Some(lr)) = (&left.left, &left.right) {
                let ll_x = left_x + left_name.len() as u16 + 1;
                buf.set_string(ll_x, left_y, "─┬─", Theme::tree_connector());

                let ll_y = left_y.saturating_sub(1);
                buf.set_string(ll_x + 1, ll_y, "┌", Theme::tree_connector());
                let ll_name = ll.chord.name();
                buf.set_string(ll_x + 3, ll_y, &ll_name, Theme::tree_expected());

                let lr_y = left_y + 1;
                if lr_y < area.y + area.height {
                    buf.set_string(ll_x + 1, lr_y, "└", Theme::tree_connector());
                    let lr_name = lr.chord.name();
                    buf.set_string(ll_x + 3, lr_y, &lr_name, Theme::tree_surprise());
                }
            }
        }

        if let Some(right) = &node.right {
            let right_y = center_y + 1;
            if right_y < area.y + area.height {
                buf.set_string(connector_x + 1, right_y, "└", Theme::tree_connector());
                buf.set_string(connector_x + 2, right_y, "─", Theme::tree_connector());

                let right_x = connector_x + 4;
                let right_name = right.chord.name();
                let line = Line::from(vec![Span::styled(&right_name, Theme::tree_surprise())]);
                buf.set_line(right_x, right_y, &line, col_width);

                if let (Some(rl), Some(rr)) = (&right.left, &right.right) {
                    let rl_x = right_x + right_name.len() as u16 + 1;
                    buf.set_string(rl_x, right_y, "─┬─", Theme::tree_connector());

                    let rl_y = right_y;
                    buf.set_string(rl_x + 1, rl_y - 1, "┌", Theme::tree_connector());
                    let rl_name = rl.chord.name();
                    buf.set_string(rl_x + 3, rl_y - 1, &rl_name, Theme::tree_expected());

                    let rr_y = right_y + 1;
                    if rr_y < area.y + area.height {
                        buf.set_string(rl_x + 1, rr_y, "└", Theme::tree_connector());
                        let rr_name = rr.chord.name();
                        buf.set_string(rl_x + 3, rr_y, &rr_name, Theme::tree_surprise());
                    }
                }
            }
        }
    }
}

impl Widget for ChordTree {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 5 || area.width < 20 {
            return;
        }

        self.render_tree(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theory::{Chord, Note, Quality};

    #[test]
    fn test_render_single_node() {
        let chord = Chord::new(Note::new(60), Quality::Major);
        let node = ProgressionNode::new(chord);

        let tree = ChordTree::new().root(node);
        let area = Rect::new(0, 0, 60, 10);
        let mut buf = Buffer::empty(area);

        tree.render(area, &mut buf);

        let content: String = buf.content.iter().map(|c| c.symbol()).collect();
        assert!(content.contains("C"));
    }

    #[test]
    fn test_render_two_levels() {
        let c_major = Chord::new(Note::new(60), Quality::Major);
        let f_major = Chord::new(Note::new(65), Quality::Major);
        let a_minor = Chord::new(Note::new(69), Quality::Minor);
        let g_major = Chord::new(Note::new(67), Quality::Major);
        let d_minor = Chord::new(Note::new(62), Quality::Minor);

        let left = ProgressionNode::new(f_major).with_children(
            ProgressionNode::new(g_major.clone()),
            ProgressionNode::new(d_minor.clone()),
        );
        let right = ProgressionNode::new(a_minor)
            .with_children(ProgressionNode::new(d_minor), ProgressionNode::new(g_major));
        let root = ProgressionNode::new(c_major).with_children(left, right);

        let tree = ChordTree::new().root(root).depth(2);
        let area = Rect::new(0, 0, 60, 10);
        let mut buf = Buffer::empty(area);

        tree.render(area, &mut buf);

        let content: String = buf.content.iter().map(|c| c.symbol()).collect();
        assert!(content.contains("C"));
        assert!(content.contains("F"));
        assert!(content.contains("Am"));
    }
}
