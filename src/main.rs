// #![feature(drain_filter)]
use config::Config;

mod config;
mod date_format;
mod print;
mod task;

#[macro_export]
macro_rules! fuck {
    () => {
        quit::with_code(0);
    };
}

fn arguments_missing(args: &Vec<String>) -> bool {
    if args.len() == 1 {
        match &args[0] as &str {
            "a" | "d" | "n" | "c" => {
                //TODO change this to print::missing_argument();
                eprintln!("Missing arguments for '{}'", &args[0]);
                return true;
            }
            _ => (),
        }
    }
    return false;
}

fn arguments(args: Vec<String>) {
    let mut numbers = false;

    for num in &args {
        for char in num.chars() {
            if char.is_numeric() {
                numbers = true;
                continue;
            }
        }
    }

    if arguments_missing(&args) {
        return;
    }

    match &args[0] as &str {
        "h" | "--help" | "help" => print::help(),
        _ => (),
    }

    let mut config = Config::new();

    match &args[0] as &str {
        "a" => config.add_task(),
        "d" => config.delete_task(),
        "c" => config.check_task(),
        "n" => config.add_note(),
        "cls" => config.clear_tasks(),
        "o" | "old" => config.print_old(),
        "b" | "backup" => config.backup(),
        "h" | "--help" | "help" => print::help(),
        _ => {
            if numbers {
                config.check_task();
            } else {
                config.add_task();
            }
        }
    };

    config.print_tasks();

    //config.write()
}

#[quit::main]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        0 => {
            let config = Config::new();
            config.print_tasks();
        }
        _ => arguments(args),
    }
}
