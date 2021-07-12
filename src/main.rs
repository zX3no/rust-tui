mod print;
mod tasks;

//static APPLICATION: usize = 0;
static COMMAND: usize = 1;
static ARGUMENTS: usize = 2;

fn main() -> crossterm::Result<()> {
    tasks::check_file();  

    let mut args: Vec<String> = std::env::args().collect();

    if args.len() >= 2 {
        match &args[COMMAND] as &str {
            "add" => 
            {
                args.remove(0);
                args.remove(0);
                let task: String = args.join(" ");
                tasks::add_task(task)?; 
            },
            "rm" => tasks::delete_task(&args[ARGUMENTS])?,
            "check" => tasks::check_task(&args[ARGUMENTS])?,
            "clear" => tasks::clear_tasks()?,
            _ => ()
        };
    }

    tasks::print_tasks();

    Ok(())
}

//There will be two files
//Done and Doing 
//When a task is completed it will be moved to another file
//Tasks will have an id but these id's will be changed when a task is removed
//Task id's will always start from 1 and go up to 999
//Date is annoying todo so i'll skip it for now