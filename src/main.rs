use crossterm::Result;

mod tasks;

fn single_argument(arg: &str) -> bool {
    if arg.len() == 3 {
        let first: usize = arg[0..1].parse().unwrap();
        let dash = &arg[1..2];
        let last: usize = arg[2..3].parse().unwrap();

        if dash == "-" {
            return tasks::check_task(vec![first, last], true);
        }
    }

    if let Ok(number) = arg.parse::<usize>() {
        return tasks::check_task(vec![number], false);
    }

    match arg {
        "cls" => {
            tasks::clear_tasks();
            return true;
        }
        "o" => tasks::print_old_tasks(),
        "b" => tasks::backup(),
        "d" | "n" | "a" => println!("Missing arguments for \'{}\'", arg),
        "h" | "--help" => tasks::print::help(),
        _ => {
            tasks::add_task(vec![arg.to_string()]);
            return true;
        }
    };

    return false;
}

fn multiple_arugments(args: Vec<String>) -> bool {
    //get all the numbers
    let mut numbers: Vec<usize> = Vec::new();
    for num in &args {
        if let Ok(num) = num.parse::<usize>() {
            numbers.push(num);
        }
    }

    match &args[0] as &str {
        "a" => tasks::add_task(args),
        "d" => {
            if !tasks::delete_task(args) {
                return true;
            }
        }
        "n" => tasks::add_note(args),
        _ => {
            //if we have numbers and none of the other arguments are called
            if !numbers.is_empty() {
                return tasks::check_task(numbers, false);
            }

            tasks::add_task(args);
        }
    };
    return false;
}

fn main() -> Result<()> {
    tasks::check_files()?;

    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        0 => {
            //empty
        }
        1 => {
            //check if we want to print tasks
            if single_argument(args[0].as_str()) {
                return Ok(());
            }
        }
        _ => {
            //check if we want to print tasks
            if multiple_arugments(args) {
                return Ok(());
            }
        }
    }

    tasks::print_tasks();

    Ok(())
}
