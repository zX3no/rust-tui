mod config;
mod date_format;
mod print;
mod tasks;

fn arguments(args: Vec<String>) {
    let mut numbers = false;

    if args.len() > 1 {
        for num in &args {
            if let Ok(_) = num.parse::<usize>() {
                numbers = true;
                continue;
            }
        }
    } else {
        for char in args[0].chars() {
            if char.is_numeric() {
                numbers = true;
                continue;
            }
        }
    }

    match &args[0] as &str {
        "a" => {
            if tasks::add_task(args) {
                return;
            }
        }
        "d" => {
            if tasks::delete_task(args) {
                return;
            }
        }
        "c" => {
            if tasks::check_task(args) {
                return;
            }
        }
        "n" => {
            if tasks::add_note(args) {
                return;
            }
        }
        "cls" => tasks::clear_tasks(),
        "o" | "old" => tasks::old_tasks(),
        "b" | "backup" => config::backup(),
        "h" | "--help" | "help" => print::help(),
        _ => {
            if numbers {
                tasks::check_task(args);
            } else {
                tasks::add_task(args);
            }
        }
    };

    tasks::tasks();
}

fn main() {
    config::check_files().unwrap();

    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        0 => tasks::tasks(),
        _ => arguments(args),
    }
}
