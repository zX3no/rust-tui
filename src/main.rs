mod print;
mod tasks;
use std::env;

fn main() -> crossterm::Result<()> {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    if args.len() >= 2 {
        if args[0] == "add" {
            args.remove(0);
            let task: String = args.join(" ");
            tasks::write_task(&task);
        }
    }
    else if args.len() == 1 {
        println!("Enter a task to add.");
        //TODO how to return?
        std::process::exit(0x0100);
    }

    if tasks::file_exists() {
        tasks::read_file()?;
    }

    Ok(())
}

//There will be two files
//Done and Doing 
//When a task is completed it will be moved to another file
//Tasks will have an id but these id's will be changed when a task is removed
//Task id's will always start from 1 and go up to 999
//Date is annoying todo so i'll skip it for now