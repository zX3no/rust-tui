#![allow(unused_imports)]
use crate::task::{Task, Tasks};
use crate::{fuck, print};
use chrono::{DateTime, Utc};
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use hashbrown::HashMap;
use itertools::Itertools;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    fs::{self, create_dir},
    io::{stdout, Write},
    path::{Path, PathBuf},
};

type Board<'a> = HashMap<String, usize>;

pub enum ConfigFile {
    Current,
    Old,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    tasks: Tasks,
    old_tasks: Tasks,

    total_tasks: usize,

    args: Vec<String>,
}

fn exists(paths: &[&PathBuf]) {
    for path in paths {
        if !Path::new(&path).exists() {
            create_dir(&path).unwrap();
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let path = dirs::config_dir().unwrap();

        exists(&[
            &path,
            &path.join("t"),
            &path.join(r"t\tasks.toml"),
            &path.join(r"t\old.toml"),
        ]);

        let tasks = Config::deserialize(&path.join(r"t\tasks.toml"));
        let old_tasks = Config::deserialize(&path.join(r"t\old.toml"));

        let mut total_tasks: usize = 0;
        for task in &tasks.tasks {
            if task.checked && !task.note {
                total_tasks += 1;
            }
        }

        Config {
            tasks,
            old_tasks,
            total_tasks,
            args: std::env::args().skip(1).collect(),
        }
    }

    pub fn add_task(&mut self, note: bool) {
        let mut board_name = String::from("Tasks");
        let item: String;

        let mut args = self.args.clone();

        //Remove the add or note command
        if args[0] == "a" || args[0] == "n" {
            args.remove(0);
        }

        match args.len() {
            2.. => {
                if args[0].contains('!') {
                    //t !board 'task ...'
                    board_name = args[0].replace('!', "");
                    item = args[1..].join(" ");
                } else {
                    //t 'long task ...'
                    item = args[0..].join(" ");
                }
            }
            1.. => {
                if args[0].contains('!') {
                    //t !board
                    fuck!("Missing task!");
                } else {
                    //t 'task'
                    item = args[0..].join(" ");
                }
            }
            _ => panic!("This should not have happened."),
        };

        self.tasks.push(Task {
            item,
            checked: false,
            board_name,
            note,
            date: Utc::now(),
        });
    }

    pub fn delete_task(&mut self) {
        let numbers = self.get_numbers();

        if numbers.is_empty() {
            fuck!("{} is not a valid number.", self.args[1]);
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
                fuck!("'{}' is not a task!", id + 1);
            }
        }
    }

    pub fn check_task(&mut self) {
        let numbers = self.get_numbers();

        if numbers.is_empty() && self.args.len() > 1 {
            fuck!("{} is not a valid number.", self.args[1]);
        }

        for id in numbers {
            if id > self.tasks.len() || self.tasks.tasks[id].note {
                fuck!("'{}' is not a task!", id + 1);
            }

            //todo can this be done better?
            self.tasks.tasks[id].checked = !self.tasks.tasks[id].checked;
        }
    }

    pub fn clear_tasks(&mut self) {
        //Get a list of all checked tasks
        let old: Vec<Task> = self
            .tasks
            .iter()
            .filter_map(|task| match task.checked {
                true => Some(task.clone()),
                false => None,
            })
            .collect();

        //Add the checked tasks to the old tasks
        for task in old {
            self.old_tasks.tasks.push(task.clone());
        }

        //Delete the checked tasks
        self.tasks.tasks.retain(|task| !task.checked);
    }

    pub fn backup(&self) {
        fuck!("Tasks are backed up!");
    }

    pub fn print_tasks(&mut self) {
        self.check_empty();
        self.sort_tasks();

        //todo wtf is this?
        let mut board_completed = Board::new();
        let mut board_total = Board::new();

        let total_tasks_completed = 0;
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

        let mut total_notes = 0;
        let mut index = 0;

        //Clears the screen
        #[cfg(not(debug_assertions))]
        execute!(
            stdout(),
            Hide,
            DisableBlinking,
            MoveTo(0, 0),
            Clear(ClearType::All)
        )
        .unwrap();

        //Print the header for the default board
        if board_list.contains(&"Tasks".to_string()) {
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
        }

        //Remove the default board, we will print this last
        board_list.retain(|x| x != "Tasks");

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

        print::footer(total_tasks_completed, self.total_tasks, total_notes);

        #[cfg(not(debug_assertions))]
        execute!(stdout(), Print("\n"), Show, EnableBlinking).unwrap();
    }

    pub fn print_old(&self) {
        let total_tasks = self.old_tasks.len();

        if total_tasks == 0 {
            fuck!("You have no old tasks!");
        }

        print::header(total_tasks, total_tasks, &"Tasks".to_string());

        for (id, task) in (&self.old_tasks).into_iter().enumerate() {
            let day = (Utc::now() - task.date).num_days();
            print::task(id + 1, task.checked, &task.item, day, total_tasks);
        }

        println!();
        fuck!();
    }

    pub fn print_dir(&self) {
        let path = dirs::config_dir().unwrap();
        println!(
            "{}",
            &path.join(r"t\tasks.toml").as_path().to_string_lossy()
        );
        println!("{}", &path.join(r"t\old.toml").as_path().to_string_lossy());
        fuck!();
    }

    fn write(file: ConfigFile, data: &Tasks) {
        let mut path = dirs::config_dir().unwrap();
        match file {
            ConfigFile::Current => path = path.join(r"t\tasks.toml"),
            ConfigFile::Old => path = path.join(r"t\old.toml"),
        };

        if Config::deserialize(&path) != *data {
            let mut file = File::create(&path).unwrap();
            let output = toml::to_string(&data).unwrap();
            file.write_all(output.as_bytes()).unwrap();
        }
    }

    fn deserialize(file_path: &Path) -> Tasks {
        let data = fs::read_to_string(file_path).expect("Unable to read file.");

        if data.is_empty() {
            return Tasks::new();
        }
        toml::from_str(&data).unwrap()
    }

    fn sort_tasks(&mut self) {
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
        Config::write(ConfigFile::Current, &self.tasks)
    }

    fn get_numbers(&mut self) -> Vec<usize> {
        let mut numbers: Vec<usize> = Vec::new();

        //Match string: num-num
        //2-10 or 45-79
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

        numbers
    }

    fn check_empty(&self) {
        if self.tasks.is_empty() {
            File::create(dirs::config_dir().unwrap().join(r"t\tasks.toml")).unwrap();
            print::help_message();
            fuck!();
        }
    }
}

//Save to file after object is destroyed
impl Drop for Config {
    fn drop(&mut self) {
        Config::write(ConfigFile::Current, &self.tasks);
        Config::write(ConfigFile::Old, &self.old_tasks);
    }
}
