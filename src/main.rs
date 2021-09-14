mod config;
mod date_format;
mod print;
mod tasks;

fn arguments_missing() -> bool {
    let args: Vec<String> = std::env::args().skip(1).collect();
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

    if arguments_missing() {
        return;
    }

    match &args[0] as &str {
        "a" => tasks::add_task(args),
        "d" => tasks::delete_task(args),
        "c" => tasks::check_task(args),
        "n" => tasks::add_note(args),
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

#[quit::main]
fn main() {
    config::check_files().unwrap();

    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        0 => tasks::tasks(),
        _ => arguments(args),
    }
}
