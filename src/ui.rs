use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, List, ListItem, Padding, Widget},
};

use crate::App;

impl App {
    pub fn render_list_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" List Mode ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let list = List::new([ListItem::new(Line::from("Name of a Service"))]);

        list.render(Block::inner(&block, area), buf);
        block.render(area, buf);
    }

    pub fn render_edit_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Edit Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }

    pub fn render_view_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" View Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }

    pub fn render_help_mode(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Help Mode ");
        let block = Block::bordered()
            .title(title)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }
}
