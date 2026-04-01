use std::io;
use rusqlite::Connection;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, DefaultTerminal, Frame};

pub struct App {
    exit: bool,
    mode: Mode,
    conn: &Connection,
}

enum Mode {
    List,
    View,
    Edit,
    Help, 
    Shortcuts,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event);
            },
            _ => {},
        }

        Ok(())
    }

    fn handle_key_events(&mut self, event: KeyEvent) {
        match self.mode {
            Mode::List => self.handle_list_inputs(event),
            Mode::View => self.handle_view_inputs(event),
            Mode::Edit => self.handle_edit_inputs(event),
            Mode::Help => self.handle_help_inputs(event), 
            Mode::Shortcuts => self.handle_shortcut_inputs(event), 
        }
    }

    fn handle_list_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
            KeyCode::Char('e') => self.mode = Mode::Edit,
            KeyCode::Char('\\') => self.mode = Mode::Shortcuts,
            KeyCode::Enter => self.mode = Mode::View,
            _ => {}
        }
    }

    fn handle_view_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
            KeyCode::Esc => self.mode = Mode::List,
            KeyCode::Char('e') => self.mode = Mode::Edit,
            _ => {}
        }
    }

    fn handle_edit_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
            KeyCode::Esc => self.mode = Mode::List,
            _ => {}
        }
    }

    fn handle_help_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => self.mode = Mode::List,
            _ => {}
        }
    }

    fn handle_shortcut_inputs(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => self.mode = Mode::List,
            _ => {}
        }
    }
}

impl Default for App {
    fn default(conn: &Connection) -> Self {
        Self {
            exit: false,
            mode: Mode::List,
            conn: &Connection,
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
            Mode::Help => self.render_help_mode(area, buf),
            Mode::Shortcuts => self.render_shortcut_mode(area, buf),
        }
    }
}
