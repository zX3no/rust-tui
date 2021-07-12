use serde::{Deserialize, Serialize};

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[path = "./print.rs"]
mod print;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    item: String,
    checked: bool,
    //TODO add date
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    task: Vec<Task>
}

//TODO Change return type to option
fn get_id(args: &String) -> usize {
    if args.parse::<usize>().is_ok() {
        let id: usize = args.parse().unwrap();
        return id-1;
    }
    else {
        println!("Invalid task number.");
    }
    return 999; 
}

fn write_file(mut data: &Data) -> std::io::Result<()> {
    //TODO why do it destroy all my data?
    let toml = toml::to_string(&mut data).unwrap();
    let mut new_file = File::create("tasks.toml")?;
    new_file.write_all(&toml.as_bytes())?;

    Ok(())
}

pub fn check_task(task: &String) -> std::io::Result<()> {
    let id = get_id(&task);

    let old_file = read_file();
    let mut data: Data = toml::from_str(&old_file).unwrap();

    //Check task
    data.task[id].checked = !data.task[id].checked;

    let toml = toml::to_string(&mut data).unwrap();

    let mut new_file = File::create("tasks.toml")?;
    new_file.write_all(&toml.as_bytes())?;

    Ok(())
}

pub fn add_task(task: String) -> std::io::Result<()> {
    let file = read_file();
    let mut data: Data = toml::from_str(&file).unwrap();

    data.task.push(Task {item: task, checked: false});
    
    write_file(&data)?;

    Ok(())
}

pub fn delete_task(args: &String) -> std::io::Result<()> {
    let id = get_id(&args);
    let file = read_file();
    let mut data: Data = toml::from_str(&file).unwrap();
    let size = data.task.len();

    data.task.remove(id);

    if size > 1 {
        write_file(&data)?;
    }

    Ok(())
}

pub fn clear_tasks() -> std::io::Result<()> {
    //Get tasks out of tasks.toml
    let old_file = read_file();
    let mut data: Data = toml::from_str(&old_file).unwrap();

    for elem in data.task.iter_mut() {
        if !elem.checked {
            //Delete all unchecked tasks
            //TODO move to done.toml
        }
    }

    //TODO write to done.toml
    //write_file(&data)?;

    Ok(())
}
pub fn print_tasks() {
    let file = read_file();

    //Check if file is empty
    if file == "" {
        println!("No Tasks!");
        return;
    } 

    let data: Data = toml::from_str(&file).unwrap();

    let total_tasks = data.task.len();
    let mut completed_tasks: usize = 0;

    //Check how many tasks are completed
    for elem in data.task.iter() {
        if elem.checked {
            completed_tasks += 1;
        }
    }

    //Print header
    print::header(completed_tasks, total_tasks).ok();

    //Iterate through items and print
    //TODO Sort and order them numerically
    //ID might be uneccasary since items can be accessed iteratively 

    //Print all tasks
    for x in 0..data.task.len() {
            print::task(x as i32 + 1, data.task[x].checked, &data.task[x].item).ok();
    }
}

pub fn read_file() -> String {
    //Get contents of tasks.toml and put into string
    let mut file = File::open("tasks.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    return contents;
}

//Check if tasks.toml exists
pub fn check_file() {
    if !Path::new("tasks.toml").exists() {
        File::create("tasks.toml").ok();
    }
}