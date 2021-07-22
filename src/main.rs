mod print;
mod tasks;

fn main() -> crossterm::Result<()> {
    tasks::check_files()?;

    let mut args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if args[1].parse::<usize>().is_ok() {
            args.insert(0, "".to_string());
            tasks::check_task(&args)?;
        } else {
            match &args[1] as &str {
                "cls" => tasks::clear_tasks()?,
                "o" => {
                    tasks::print_old_tasks()?;
                    return Ok(());
                }
                "rm" | "c" => {
                    println!("Missing arguments for \'{}\'", args[1]);
                    return Ok(());
                }
                "h" => {
                    print::help();
                    return Ok(());
                }
                _ => {
                    println!("\'{}\' is not a command", args[1]);
                    return Ok(());
                }
            };
        }
    } else if args.len() >= 3 {
        if args[1].parse::<usize>().is_ok() {
            args.insert(0, "".to_string());
            tasks::check_task(&args)?;
        } else {
            match &args[1] as &str {
                "rm" => tasks::delete_task(args)?,
                "c" => tasks::check_task(&args)?,
                "h" => {
                    print::help();
                    return Ok(());
                }
                "o" => {
                    tasks::print_old_tasks()?;
                    return Ok(());
                }
                _ => {
                    args.insert(0, "".to_string());
                    tasks::add_task(args)?;
                }
            };
        }
    }

    tasks::print_tasks()?;

    Ok(())
}
