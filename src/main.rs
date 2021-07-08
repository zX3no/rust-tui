mod print;
mod tasks;
use std::env;

fn main() -> crossterm::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => 
        if args[1] == "add" {
            tasks::write_task(&args[2]);
        },
        _ => (),
    }

    //If files exist check whats in them
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