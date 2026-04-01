use std::io;
use crate::{app::App, db::init_databse};

mod app;
mod ui;
mod models;
mod db;

fn main() -> io::Result<()> {
    let conn = init_databse().expect("Failed to connect to database.");
    let mut terminal = ratatui::init();

    let mut app = App::default();

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
