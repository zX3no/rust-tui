mod print;
mod tasks;

use std::env;
use std::fs::File;

fn main() -> crossterm::Result<()> {
    if !tasks::file_exists() { 
        File::create("doing.toml")?;
    }

    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    //TODO cleanup this mess
    if args.len() >= 2 {
        if args[0] == "add" {
            args.remove(0);
            let task: String = args.join(" ");
            tasks::add_task(task)?;
        }
        else if args[0] == "rm" {
            //TODO allow users to remove multiple tasks at the same time
            args.remove(0);
            if args[0].parse::<usize>().is_ok() {
                let id: usize = args[0].parse().unwrap(); 
                tasks::delete_task(id-1)?;
            }
            else {
                println!("Invalid task number.");
            }
        }
        else if args[0] == "check" {
            //TODO allow users to check multiple tasks at the same time
            args.remove(0);
            if args[0].parse::<usize>().is_ok() {
                let id: usize = args[0].parse().unwrap();
                tasks::check_task(id-1)?;
            }
            else {
                println!("Invalid task number.");
            }
        }
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