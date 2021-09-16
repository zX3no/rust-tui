#![allow(dead_code, unused_imports)]
use chrono::{DateTime, Utc};
use crossterm::cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use hashbrown::HashMap;
use itertools::Itertools;
use regex::{Captures, Regex};

use crate::task::Task;
use crate::{fuck, print};

use std::io::{self, stdout, Read, Write};
use std::slice::Iter;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

type Board<'a> = HashMap<String, usize>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}
impl Tasks {
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    pub fn remove(&mut self, index: usize) {
        self.tasks.remove(index);
    }
    pub fn push(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn iter(&self) -> Iter<Task> {
        self.tasks.iter()
    }
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    fn new() -> Tasks {
        Tasks { tasks: Vec::new() }
    }
}

impl IntoIterator for &Tasks {
    type Item = Task;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tasks.clone().into_iter()
    }
}

impl PartialEq for Tasks {
    fn eq(&self, other: &Self) -> bool {
        self.tasks == other.tasks
    }
}

pub struct Config {
    tasks: Tasks,
    old_tasks: Tasks,

    total_tasks: usize,

    file: PathBuf,
    old: PathBuf,

    args: Vec<String>,
}

impl Config {
    ///
    /// Construction
    ///

    //maybe the destructor should write to file ?
    pub fn new() -> Self {
        let file = dirs::config_dir().unwrap().join(r"t/tasks.toml");
        let old = dirs::config_dir().unwrap().join(r"t/old.toml");

        let tasks = Config::read(&file);
        let old_tasks = Config::read(&old);

        let mut total_tasks: usize = 0;
        for task in &tasks.tasks {
            if task.checked && !task.note {
                total_tasks += 1;
            }
        }

        //TODO sort tasks?
        Config {
            tasks,
            old_tasks,
            total_tasks,
            file,
            old,
            args: std::env::args().skip(1).collect(),
        }
    }

    ///
    /// Commands
    ///

    pub fn add_task(&mut self) {
        let mut board_name = String::from("Tasks");
        let item: String;

        if self.args.len() == 1 {
            item = self.args[0..].join(" ");
        } else if self.args[1].contains('!') {
            board_name = self.args[1].replace('!', "");
            item = self.args[2..].join(" ");
        } else if self.args[0] == "a" {
            item = self.args[1..].join(" ");
        } else {
            item = self.args[0..].join(" ");
        }

        self.tasks.push(Task {
            item,
            checked: false,
            board_name,
            note: false,
            date: Utc::now(),
            id: self.gen_id(),
        });
    }

    pub fn delete_task(&mut self) {
        let numbers = self.get_numbers();

        if numbers.is_empty() {
            eprintln!("{} is not a valid number.", self.args[1]);
            fuck!();
        }

        //since we're deleting tasks the size will change
        let size = self.tasks.len();

        //this is annoying but again the size chagnes
        let mut indexes_removed = 0;

        for id in numbers {
            if id < size {
                self.tasks.remove(id - indexes_removed);
                indexes_removed += 1;
            } else if id != 0 {
                eprintln!("'{}' is not a task!", id);
                fuck!();
            }
        }
    }

    pub fn add_note(&mut self) {
        let item = self.args[1..].join(" ");

        self.tasks.push(Task {
            item,
            checked: false,
            board_name: "Tasks".to_string(),
            note: true,
            date: Utc::now(),
            id: self.gen_id(),
        });
    }

    pub fn check_task(&mut self) {
        let numbers = self.get_numbers();

        if numbers.is_empty() && self.args.len() > 1 {
            eprintln!("{} is not a valid number.", self.args[1]);
            fuck!();
        } else if self.args.len() == 1 {
            eprintln!("{} is not a valid number.", self.args[0]);
            fuck!();
        }

        for id in numbers {
            if id > self.tasks.len() || self.tasks.tasks[id].note {
                eprintln!("'{}' is not a task!", id);
                fuck!();
            }

            //todo can this be done better?
            self.tasks.tasks[id].checked = !self.tasks.tasks[id].checked;
        }
    }

    pub fn clear_tasks(&mut self) {
        //if tasks is checked remove it
        let old: Vec<Task> = self
            .tasks
            .iter()
            .filter_map(|task| match task.checked {
                true => Some(task.clone()),
                false => None,
            })
            .collect();

        self.tasks.tasks.drain_filter(|task| task.checked);

        for task in old {
            self.old_tasks.push(task);
        }
    }

    pub fn backup(&self) {
        let file_path = dirs::config_dir().unwrap().join(r"t/backup.toml");
        self.write(&file_path);
        println!("Tasks are backed up!");
        fuck!();
    }

    pub fn print_tasks(&self) {
        self.check_empty();

        //todo wtf is this?
        let mut board_completed = Board::new();
        let mut board_total = Board::new();

        // let mut tasks_completed = self.tasks.iter().map(|task| task.checked).collect();
        let tasks_completed = 0;
        let now: DateTime<Utc> = Utc::now();

        //Get a list of all boards
        let mut board_list: Vec<String> = self
            .tasks
            .iter()
            .map(|task| task.board_name.clone())
            .unique()
            .collect();

        //Get total and completed tasks for each board
        for board in &board_list {
            //boards completed and board total
            let (mut bc, mut bt) = (0, 0);

            for task in &self.tasks {
                if &task.board_name == board {
                    bt += 1;
                    if task.checked {
                        bc += 1;
                    }
                }
            }

            //push the name and value into a hashmap
            board_completed.insert(board.clone(), bc);
            board_total.insert(board.clone(), bt);
        }

        //Remove the default board, we will print this last
        board_list.retain(|x| x != "Tasks");

        let mut total_notes = 0;
        let mut index = 0;

        // execute!(
        //     stdout(),
        //     Hide,
        //     DisableBlinking,
        //     MoveTo(0, 0),
        //     Clear(ClearType::All)
        // )
        // .unwrap();

        //Print the header for the default board
        print::header(
            board_completed["Tasks"],
            board_total["Tasks"],
            &"Tasks".to_string(),
        );

        //Print the default board
        for task in &self.tasks {
            if task.board_name == "Tasks" {
                index += 1;
                let day = (now - task.date).num_days();
                if task.note {
                    print::note(index, &task.item, self.total_tasks);
                    total_notes += 1;
                } else {
                    print::task(index, task.checked, &task.item, day, board_total["Tasks"]);
                }
            }
        }

        println!();

        //Print all the custom boards
        for board in board_list {
            print::header(
                board_completed[board.as_str()],
                board_total[board.as_str()],
                &board,
            );
            for task in &self.tasks {
                let day = (now - task.date).num_days();

                if task.board_name == board {
                    index += 1;
                    if task.note {
                        print::note(index, &task.item, self.total_tasks);
                        total_notes += 1;
                    } else {
                        print::task(index, task.checked, &task.item, day, self.total_tasks);
                    }
                }
            }
            println!();
        }

        print::footer(tasks_completed, self.total_tasks, total_notes);

        execute!(stdout(), Print("\n"), Show, EnableBlinking).unwrap();
    }

    pub fn print_old(&self) {
        let now: DateTime<Utc> = Utc::now();

        for task in &self.tasks {
            let day = (now - task.date).num_days();
            print::task(task.id + 1, task.checked, &task.item, day, self.total_tasks);
        }
    }

    ///
    /// Helpers
    ///

    fn append_toml(&self, file_name: PathBuf) {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
            .unwrap();

        let output = toml::to_string(&self.tasks).unwrap();
        file.write_all(output.as_bytes()).unwrap();
    }

    pub fn save(&self) {
        if Config::read(&self.file) != self.tasks {
            self.write(&self.file);
        } else if Config::read(&self.old) != self.old_tasks {
            self.write(&self.old);
        }
    }

    pub fn read(file_path: &PathBuf) -> Tasks {
        let mut data = File::open(file_path).unwrap();

        //Load contents into a string
        let mut contents = String::new();
        data.read_to_string(&mut contents).unwrap();

        if contents.is_empty() {
            return Tasks::new();
            //TODO
        }

        toml::from_str(&contents).unwrap()
    }

    pub fn write(&self, file_path: &PathBuf) {
        let mut file = File::create(file_path).unwrap();
        let output = toml::to_string(&self.tasks).unwrap();
        file.write_all(output.as_bytes()).unwrap();
    }

    fn gen_id(&self) -> usize {
        self.tasks.len()
    }

    fn sort_tasks(&mut self) {
        let old_data = self.tasks.clone();
        //Get a list of all boards and remove the duplicates
        let mut board_list: Vec<String> = self
            .tasks
            .iter()
            .map(|task| task.board_name.clone())
            .unique()
            .collect();

        //Remove the default board
        board_list.retain(|x| x != "Tasks");

        let mut sorted_tasks: Vec<Task> = self
            .tasks
            .iter()
            .filter_map(|task| match &task.board_name as &str {
                "Tasks" => Some(task.clone()),
                _ => None,
            })
            .collect();

        for board in board_list {
            for task in &self.tasks {
                if task.board_name == board {
                    sorted_tasks.push(task.clone());
                }
            }
        }

        self.tasks.tasks = sorted_tasks;

        //Only write to file if tasks need to be sorted
        if self.tasks != old_data {
            // tasks::write_toml(Config::current(), &new_data);
        }
    }

    fn get_numbers(&mut self) -> Vec<usize> {
        let mut numbers: Vec<usize> = Vec::new();

        let re = Regex::new(
            r"(?x)
                (?P<first>\d+)
                -
                (?P<last>\d+)
                ",
        )
        .unwrap();

        let mut caps: Option<Captures> = None;

        if self.args.len() == 1 {
            caps = re.captures(&self.args[0]);
        } else if self.args.len() >= 2 {
            caps = re.captures(&self.args[1]);
        }

        if let Some(caps) = caps {
            let first = caps["first"].parse::<usize>().unwrap();
            let last = caps["last"].parse::<usize>().unwrap();

            if first > last {
                return numbers;
            }

            for num in first - 1..last {
                numbers.push(num);
            }

            return numbers;
        }

        for num in &self.args {
            if let Ok(num) = num.parse::<usize>() {
                if num != 0 {
                    numbers.push(num - 1);
                }
            }
        }

        return numbers;
    }

    fn check_files(&mut self) -> io::Result<()> {
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
        if !Path::new(&self.file).exists() {
            File::create(&self.file)?;
        } else {
            self.sort_tasks();
        }

        //check if old.toml exists
        if !Path::new(&self.old).exists() {
            File::create(&self.old)?;
        }

        Ok(())
    }

    fn check_empty(&self) {
        if self.tasks.is_empty() {
            File::create(&self.file).unwrap();
            print::help_message();
            fuck!();
        }
    }
}
