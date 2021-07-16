mod print;
mod tasks;
fn main() -> crossterm::Result<()> {
    tasks::check_files()?;
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match &args[1] as &str {
            "clear" => tasks::clear_tasks()?,
            _ => {
                println!("Missing arguments for \'{}\'.", args[1]);
                return Ok(());
            }
        };
    } else if args.len() >= 3 {
        match &args[1] as &str {
            "add" | "a" => tasks::add_task(args)?,
            "rm" | "delete" => tasks::delete_task(args)?,
            "check" | "c" => tasks::check_task(args)?,
            _ => {
                println!("Invalid command.");
                return Ok(());
            }
        };
    }
    tasks::print_tasks()?;

    Ok(())
}
