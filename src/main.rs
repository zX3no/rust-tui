#![allow(unused_imports)]
use crossterm::{
    cursor, queue,
    terminal::{Clear, ClearType},
    Result,
};
use std::io::stdout;
use tasks::check_files;
mod tasks;

fn single_argument(arg: &str) -> Result<bool> {
    match arg {
        "cls" => {
            tasks::clear_tasks()?;
            return Ok(true);
        }
        "o" => tasks::print_old_tasks()?,
        "d" | "b" | "a" => println!("Missing arguments for \'{}\'", arg),
        "h" => tasks::print::help(),
        _ => {
            tasks::add_task(vec![arg.to_string()])?;
            return Ok(true);
        }
    };

    Ok(false)
}

fn multiple_arugments(args: Vec<String>) -> Result<()> {
    match &args[0] as &str {
        "a" => tasks::add_task(args)?,
        "d" => tasks::delete_task(args)?,
        "c" => tasks::check_task(args)?,
        "n" => tasks::add_note(args)?,
        _ => tasks::add_task(args)?,
    };
    Ok(())
}

fn main() -> Result<()> {
    tasks::check_files()?;

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        tasks::print_tasks()?;
        return Ok(());
    }

    if args[0].parse::<usize>().is_ok() {
        tasks::check_task(args)?;
    } else {
        match args.len() {
            1 => {
                if let false = single_argument(args[0].as_str())? {
                    return Ok(());
                }
            }
            _ => multiple_arugments(args)?,
        }
    }

    tasks::print_tasks()?;

    Ok(())
}
