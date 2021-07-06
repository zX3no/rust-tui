mod print_task;
mod config;

use crossterm::Result;
use print_task::{header, task};
use serde::{Serialize, Deserialize};
//use std::fs::File;

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

//A task is made of
//the task, the id, date created

#[derive(Serialize, Deserialize, Debug)]
struct Task{
    item: String,
    id: i32,
    date: String
}

fn main() -> Result<()> {
    config::create_config();
    /*
    //tasks()?;
    let t = Task {item: String::from("This is a task."), id: 1, date: String::from("1/07/2021")};
    let t2 = Task {item: String::from("This is a task."), id: 2, date: String::from("4/07/2021")};

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&t).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Task = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    serde_json::to_writer(&File::create("test.json")?, &t)?;
*/
    Ok(())
}
