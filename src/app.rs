use std::io;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, DefaultTerminal, Frame};

pub struct App {
    exit: bool,
    mode: Mode,
}

enum Mode {
    List,
    View,
    Edit,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            mode: Mode::List,
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.mode {
            Mode::List => self.render_list_mode(area, buf),
            Mode::View => self.render_view_mode(area, buf),
            Mode::Edit => self.render_edit_mode(area, buf),
        }
    }
}
