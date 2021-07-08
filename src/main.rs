mod print;
mod tasks;

//There will be two files
//Done and Doing 
//When a task is completed it will be moved to another file
//Tasks will have an id but these id's will be changed when a task is removed
//Task id's will always start from 1 and go up to 999

fn main() -> crossterm::Result<()> {
    tasks::read_file()?;
  
    Ok(())
}
