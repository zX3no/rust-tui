use config::Config;
use database::Database;

#[macro_use]
extern crate lazy_static;

mod config;
mod database;
mod date_format;
mod print;
mod task;

#[macro_export]
macro_rules! fuck {
    () => (quit::with_code(0));
    ($($arg:tt)*) => ({
       eprintln!($($arg)*);
       quit::with_code(0);
    })
}

fn arguments_missing(args: &[String]) {
    if args.len() == 1 {
        match &args[0] as &str {
            "a" | "d" | "n" | "c" => {
                fuck!("Missing arguments for '{}'", &args[0]);
            }
            _ => (),
        }
    }
}

fn arguments(args: &[String]) {
    let mut numbers = false;

    for num in args {
        for char in num.chars() {
            if char.is_numeric() {
                numbers = true;
                continue;
            } else if char != '-' {
                numbers = false;
                continue;
            }
        }
    }

    match &args[0] as &str {
        "h" | "--help" | "help" => print::help(),
        _ => (),
    }

    arguments_missing(args);

    let mut config = Config::new();

    match &args[0] as &str {
        "a" => config.add_task(false),
        "d" => config.delete_task(),
        "c" => config.check_task(),
        "n" => config.add_task(true),
        "cls" => config.clear_tasks(),
        "dir" => config.print_dir(),
        "o" | "old" => config.print_old(),
        "b" | "backup" => config.backup(),
        _ => {
            if numbers {
                config.check_task();
            } else {
                config.add_task(false);
            }
        }
    };

    config.print_tasks();
}

//TODO:
//remove fuck!()
//change to sqlite database
//refactor and simplify code
//better argument handling

#[quit::main]
fn main() {
    // let args: Vec<String> = std::env::args().skip(1).collect();

    let db = Database::new();
    db.insert_task("this is a test task", None);
    db.check_task(1);
    db.clear_tasks().unwrap();
    dbg!(db.get_tasks());
    dbg!(db.get_old_tasks());
    // db.delete_task(1);

    // match args.len() {
    //     0 => {
    //         let mut config = Config::new();
    //         config.print_tasks();
    //     }
    //     _ => arguments(&args),
    // }
}
