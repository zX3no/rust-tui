mod config;
mod date_format;
mod print;
mod tasks;

fn single_argument(arg: &str) {
    let mut numbers = false;

    for char in arg.chars() {
        if char.is_numeric() {
            numbers = true;
            continue;
        }
    }

    match arg {
        "cls" => tasks::clear_tasks(),
        "o" | "old" => tasks::old_tasks(),
        "b" | "backup" => config::backup(),
        "d" | "n" | "a" | "c" => {
            println!("Missing arguments for \'{}\'", arg);
            return;
        }
        "h" | "--help" | "help" => print::help(),
        _ => {
            if numbers {
                tasks::check_task(vec![arg.to_string()]);
            } else {
                tasks::add_task(vec![arg.to_string()])
            }
        }
    };

    tasks::tasks();
}

fn multiple_arugments(args: Vec<String>) {
    let mut numbers = false;

    for num in &args {
        if let Ok(_) = num.parse::<usize>() {
            numbers = true;
            continue;
        }
    }

    match &args[0] as &str {
        "a" => tasks::add_task(args),
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
        "n" => tasks::add_note(args),
        _ => {
            if numbers {
                if tasks::check_task(args) {
                    return;
                }
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
        1 => single_argument(args[0].as_str()),
        _ => multiple_arugments(args),
    }
}
