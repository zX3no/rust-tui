use std::io::{stdout, Write};

use app::App;

mod app;
mod database;
mod ui;

fn main() {
    App::run();
    stdout().flush().unwrap();
}
