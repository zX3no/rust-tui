use crossterm::Result;
mod tasks;

fn single_argument(arg: &str) -> Result<bool> {
    match arg {
        "cls" => {
            tasks::clear_tasks()?;
            return Ok(true);
        }
        "o" => tasks::print_old_tasks()?,
        "d" | "b" | "a" => println!("Missing arguments for \'{}\'", arg),
        "h" | "--help" => tasks::print::help(),
        _ => {
            tasks::add_task(vec![arg.to_string()])?;
            return Ok(true);
        }
    };

    Ok(false)
}

fn multiple_arugments(args: Vec<String>) -> Result<bool> {
    match &args[0] as &str {
        "a" => tasks::add_task(args)?,
        "d" => {
            if let false = tasks::delete_task(args)? {
                return Ok(false);
            }
        }
        "c" => {
            if let false = tasks::check_task(args)? {
                return Ok(false);
            }
        }
        "n" => tasks::add_note(args)?,
        _ => tasks::add_task(args)?,
    };
    Ok(true)
}

fn main() -> Result<()> {
    tasks::check_files()?;

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        tasks::print_tasks()?;
        return Ok(());
    }

    if args[0].parse::<usize>().is_ok() {
        if let false = tasks::check_task(args)? {
            return Ok(());
        }
    } else {
        match args.len() {
            1 => {
                if let false = single_argument(args[0].as_str())? {
                    return Ok(());
                }
            }
            _ => {
                if let false = multiple_arugments(args)? {
                    return Ok(());
                }
            }
        }
    }

    tasks::print_tasks()?;

    Ok(())
}
