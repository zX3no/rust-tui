use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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
    tasks: Vec<Task>,
}

fn file_task() -> PathBuf {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("t\\tasks.toml");
    return dir;
}

fn file_old() -> PathBuf {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("t\\old.toml");
    return dir;
}

fn get_id(id: &mut Vec<usize>, args: &Vec<String>) -> bool {
    for elem in args[2..].iter() {
        if elem.parse::<usize>().is_ok() {
            let temp: usize = elem.parse().unwrap();
            id.push(temp - 1);
        } else {
            println!("Invalid task number.");
            return false;
        }
    }
    return true;
}

fn get_tasks() -> Data {
    let mut file = match File::open(&file_task()) {
        Err(why) => panic!("couldn't open {}: ", why),
        Ok(file) => file,
    };
    let mut contents = String::new();
    let mut data = Data { tasks: Vec::new() };

    file.read_to_string(&mut contents).unwrap();
    if contents != "" {
        data = toml::from_str(&contents).unwrap();
    } else {
        return data;
    }

    return data;
}

fn write_toml(file_name: PathBuf, data: &Data) -> std::io::Result<()> {
    let mut file = File::create(file_name).unwrap();
    let output = toml::to_string(&data).unwrap();
    file.write_all(output.as_bytes())?;

    Ok(())
}

fn append_toml(file_name: PathBuf, data: &Data) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();

    let output = toml::to_string(&data).unwrap();
    file.write_all(output.as_bytes())?;

    Ok(())
}

pub fn check_task(args: &Vec<String>) -> std::io::Result<()> {
    let mut id: Vec<usize> = Vec::new();
    if !get_id(&mut id, &args) {
        return Ok(());
    }

    let mut data = get_tasks();

    for i in id {
        data.tasks[i].checked = !data.tasks[i].checked;
    }

    write_toml(file_task(), &data)?;

    Ok(())
}

pub fn add_task(args: Vec<String>) -> std::io::Result<()> {
    let arguments: String = args[2..].join(" ");
    let task = Task {
        item: arguments,
        checked: false,
    };
    let data = Data { tasks: vec![task] };
    append_toml(file_task(), &data)?;

    Ok(())
}

pub fn delete_task(args: Vec<String>) -> std::io::Result<()> {
    let mut id: Vec<usize> = Vec::new();
    if !get_id(&mut id, &args) {
        return Ok(());
    }

    let mut data = get_tasks();

    //since we're deleting tasks the size will change
    let size = data.tasks.len();
    //this is annoying but again the size chagnes
    let mut indexes_removed = 0;

    for i in id {
        if i < size {
            data.tasks.remove((i / 1) - indexes_removed);
            indexes_removed += 1;
        } else if i != 0 {
            println!("There is no task {}.", i + 1);
            return Ok(());
        }
    }

    if data.tasks.len() == 0 {
        File::create(file_task())?;
        return Ok(());
    }

    write_toml(file_task(), &data)?;

    Ok(())
}

pub fn clear_tasks() -> std::io::Result<()> {
    let mut data_to_append: Data = Data { tasks: Vec::new() };

    //Get finished tasks and put them in buffer
    let mut data = get_tasks();
    let mut indexes_removed = 0;

    //return if there are no tasks to clear
    if data.tasks.len() == 0 {
        return Ok(());
    }

    //Copy checked tasks to new file
    for i in 0..data.tasks.len() {
        if data.tasks[i - indexes_removed].checked {
            data_to_append
                .tasks
                .push(data.tasks[i - indexes_removed].clone());
            data.tasks.remove(i - indexes_removed);
            indexes_removed += 1;
        }
    }

    if data.tasks.len() == 0 {
        File::create(file_task())?;
    } else {
        write_toml(file_task(), &data)?;
    }

    append_toml(file_old(), &data_to_append)?;

    Ok(())
}

pub fn print_tasks() -> std::io::Result<()> {
    let data = get_tasks();

    if data.tasks.is_empty() {
        println!("No Tasks!");
        return Ok(());
    }

    let total_tasks = data.tasks.len();
    let mut completed_tasks: usize = 0;

    //Check how many tasks are completed
    for elem in data.tasks.iter() {
        if elem.checked {
            completed_tasks += 1;
        }
    }

    print::header(completed_tasks, total_tasks)?;

    //Print all tasks
    for i in 0..data.tasks.len() {
        print::task(
            i + 1,
            data.tasks[i].checked,
            &data.tasks[i].item,
            total_tasks,
        )?;
    }

    print::footer(completed_tasks, total_tasks)?;

    Ok(())
}

pub fn check_files() -> std::io::Result<()> {
    let mut path = dirs::config_dir().unwrap();
    path.push("t");
    if !Path::new(&path).exists() {
        std::fs::create_dir(&path)?;
    }

    if !Path::new(&file_task()).exists() {
        File::create(&file_task())?;
    }

    if !Path::new(&file_old()).exists() {
        File::create(&file_old())?;
    }

    Ok(())
}
