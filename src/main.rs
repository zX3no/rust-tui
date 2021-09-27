use config::Config;

mod config;
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

fn arguments_missing(args: &Vec<String>) {
    if args.len() == 1 {
        match &args[0] as &str {
            "a" | "d" | "n" | "c" => {
                fuck!("Missing arguments for '{}'", &args[0]);
            }
            _ => (),
        }
    }
}

fn arguments(args: Vec<String>) {
    let mut numbers = false;

    for num in &args {
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

    arguments_missing(&args);

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

#[quit::main]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        0 => {
            let mut config = Config::new();
            config.print_tasks();
        }
        _ => arguments(args),
    }
}
