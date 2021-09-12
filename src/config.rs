use itertools::Itertools;

use crate::tasks;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

//TODO change this to a const fn if that's possible?
// static CURRENT: PathBuf = dirs::config_dir().unwrap().join(r"t/tasks.toml");
// static OLD: PathBuf = dirs::config_dir().unwrap().join(r"t/old.toml");

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
    let data = tasks::get_tasks();
    let path = dirs::config_dir().unwrap().join(r"t/backup.toml");
    tasks::write_toml(path, &data);
}

fn sort_tasks() {
    let old_data = tasks::get_tasks();

    if old_data.tasks.is_empty() {
        return;
    }

    let mut new_data = tasks::Data { tasks: Vec::new() };

    let data = tasks::get_tasks();

    let mut board_list: Vec<String> = Vec::new();

    //Get a list of all boards
    for elem in data.tasks.iter() {
        board_list.push(elem.board_name.clone());
    }

    board_list = board_list.into_iter().unique().collect();

    board_list.retain(|x| x != "Tasks");

    for board in board_list {
        for elem in old_data.tasks.iter() {
            if elem.board_name == board {
                new_data.tasks.push(elem.clone());
            }
        }
    }

    for elem in old_data.tasks.iter() {
        if elem.board_name == "Tasks" {
            new_data.tasks.push(elem.clone());
        }
    }

    //Only write to file if tasks need to be sorted
    if !itertools::equal(&old_data.tasks, &new_data.tasks) {
        tasks::write_toml(Config::current(), &new_data);
    }
}
