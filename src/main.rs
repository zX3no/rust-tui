use std::io::stdout;

use app::App;
use tui::{backend::CrosstermBackend, Terminal};

#[macro_use]
extern crate lazy_static;

mod app;
mod database;
mod print;
mod ui;

//TODO: move to tui-rs for better clearing
fn main() {
    let app = App::new();
    // app.parse_args();

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
}
