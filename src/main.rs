use app::App;

#[macro_use]
extern crate lazy_static;

mod app;
mod database;
mod ui;

fn main() {
    let app = App::new();
    app.parse_args();
}
