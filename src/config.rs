use itertools::Itertools;

use crate::data::Data;
use crate::fuck;
use crate::tasks;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

pub struct Config {}

impl Config {
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
        let mut new_data = Data::new();
        let old_data = data.clone();

        let mut board_list: Vec<String> = Vec::new();

        //Get a list of all boards
        for task in data.iter() {
            board_list.push(task.board_name.clone());
        }

        //TODO wtf is going on here?
        board_list = board_list.into_iter().unique().collect();

        board_list.retain(|x| x != "Tasks");

        for task in old_data.iter() {
            if task.board_name == "Tasks" {
                new_data.push(task);
            }
        }

        for board in board_list {
            for task in old_data.iter() {
                if task.board_name == board {
                    new_data.push(task);
                }
            }
        }

        //Only write to file if tasks need to be sorted
        if !itertools::equal(&old_data.tasks, &new_data.tasks) {
            tasks::write_toml(Config::current(), &new_data);
        }
    }
}
