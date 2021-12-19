#[macro_use]
extern crate lazy_static;

use app::App;

mod app;
mod database;
mod ui;

fn main() {
    App::run();
}
