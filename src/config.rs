use itertools::Itertools;

use crate::data::Data;
use crate::data::Task;
use crate::fuck;
use crate::tasks;

use std::io::Read;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

pub struct Config {
    tasks: Vec<Task>,
    file: PathBuf,
    old: PathBuf,
}

impl Config {
    pub fn new(&self) -> Self {
        let mut file = File::open(&Config::current()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        if contents.is_empty() {
            //TODO
        }
        Config {
            tasks: toml::from_str(&contents).unwrap(),
            file: dirs::config_dir().unwrap().join(r"t/tasks.toml"),
            old: dirs::config_dir().unwrap().join(r"t/old.toml"),
        }
    }

    pub fn current() -> PathBuf {
        dirs::config_dir().unwrap().join(r"t/tasks.toml")
    }
    pub fn old() -> PathBuf {
        dirs::config_dir().unwrap().join(r"t/old.toml")
    }
}

pub fn check_files() -> std::io::Result<()> {
    let mut path = dirs::config_dir().unwrap();

    //check if the config dir exists
    if !Path::new(&path).exists() {
        std::fs::create_dir(&path)?;
    }

    path.push("t");

    //check if config/t exists
    if !Path::new(&path).exists() {
        std::fs::create_dir(&path)?;
    }

    //check if tasks.toml exists
    if !Path::new(&Config::current()).exists() {
        File::create(Config::current())?;
    } else {
        sort_tasks();
    }

    //check if old.toml exists
    if !Path::new(&Config::old()).exists() {
        File::create(Config::old())?;
    }

    Ok(())
}

pub fn backup() {
    let data = Data::tasks();
    let path = dirs::config_dir().unwrap().join(r"t/backup.toml");
    tasks::write_toml(path, &data);
    println!("Tasks are backed up!");
    fuck!();
}

fn sort_tasks() {
    if let Some(data) = Data::option() {
        //Get a list of all boards and remove the duplicates
        let mut board_list: Vec<String> = data
            .iter()
            .map(|task| task.board_name.clone())
            .unique()
            .collect();

        //Remove the default board
        board_list.retain(|x| x != "Tasks");

        let mut new_data: Data = data
            .iter()
            .filter_map(|task| match &task.board_name as &str {
                "Tasks" => Some(task.clone()),
                _ => None,
            })
            .collect();

        for board in board_list {
            for task in data.iter() {
                if task.board_name == board {
                    new_data.push(task);
                }
            }
        }

        //Only write to file if tasks need to be sorted
        if !itertools::equal(&data.tasks, &new_data.tasks) {
            tasks::write_toml(Config::current(), &new_data);
        }
    }
}
