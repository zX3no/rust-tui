mod print_task;
mod config;

use crossterm::Result;
use print_task::{header, task};

#[allow(dead_code)]
fn tasks() -> Result<()>{
    let completed_tasks = 5;
    let total_tasks = 10;
    //todo toml file
    header(completed_tasks, total_tasks)?;
    task(10, true, "AAAAAAA")?;
    task(2, false, "Test")?;
    task(15, false, "Make this very hard project")?;
    task(999, false, "remember to do something")?;
    task(9, true, "Takssks")?;
    Ok(())
}

//There will be two files
//Done and Doing 
//When a task is completed it will be moved to another file
//Tasks will have an id but these id's will be changed when a task is removed
//Task id's will always start from 1 and go up to 999

fn main() -> Result<()> {
    header(0, 2).ok();
    config::create_config();
  
    Ok(())
}
