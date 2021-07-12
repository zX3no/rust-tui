mod print;
mod tasks;

//static APPLICATION: usize = 0;
static COMMAND: usize = 1;
static ARGUMENTS: usize = 2;

fn main() -> crossterm::Result<()> {
    tasks::check_files();

    let mut args: Vec<String> = std::env::args().collect();

    if args.len() >= 2 {
        match &args[COMMAND] as &str {
            "add" => {
                args.remove(0);
                args.remove(0);
                let task: String = args.join(" ");
                tasks::add_task(task)?;
            }
            "rm" => tasks::delete_task(&args[ARGUMENTS])?,
            "check" => tasks::check_task(&args[ARGUMENTS])?,
            "clear" => tasks::clear_tasks()?,
            _ => (),
        };
    }
    tasks::print_tasks();

    Ok(())
}

//When a task is cleared it will be moved to another file
//Date is annoying todo so i'll skip it for now
