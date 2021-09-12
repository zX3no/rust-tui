mod tasks;

fn single_argument(arg: &str) {
    //todo implement for delete
    if arg.len() == 3 {
        if let Ok(first) = arg[0..1].parse::<usize>() {
            if let Ok(last) = arg[2..3].parse::<usize>() {
                let dash = &arg[1..2];

                if dash == "-" {
                    if tasks::check_task(vec![first, last], true) {
                        return;
                    }
                }
            }
        }
    }

    if let Ok(number) = arg.parse::<usize>() {
        if tasks::check_task(vec![number], false) {
            return;
        }
    }

    match arg {
        "cls" => tasks::clear_tasks(),
        "o" => tasks::print_old_tasks(),
        "b" => tasks::backup(),
        "d" | "n" | "a" | "c" => {
            println!("Missing arguments for \'{}\'", arg);
            return;
        }
        "h" | "--help" => tasks::print::help(),
        _ => (),
        //todo maybe reimpliment?
        //tasks::add_task(vec![arg.to_string()]),
    };

    tasks::print_tasks();
}

fn multiple_arugments(args: Vec<String>) {
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
            if tasks::delete_task(args) {
                return;
            }
        }
        "c" => {
            if tasks::check_task(numbers, false) {
                return;
            }
        }
        "n" => tasks::add_note(args),
        _ => {
            //if we have numbers and none of the other arguments are called
            if !numbers.is_empty() {
                if tasks::check_task(numbers, false) {
                    return;
                }
            }

            tasks::add_task(args);
        }
    };

    tasks::print_tasks();
}

fn main() {
    tasks::check_files().unwrap();

    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        0 => tasks::print_tasks(),
        1 => single_argument(args[0].as_str()),
        _ => multiple_arugments(args),
    }
}
