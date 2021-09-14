#![allow(clippy::needless_return)]
use chrono::{DateTime, Utc};
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use hashbrown::HashMap;
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
use std::path::PathBuf;

use crate::config::Config;
use crate::date_format;
use crate::print;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub item: String,
    pub checked: bool,
    pub board_name: String,
    pub note: bool,
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub tasks: Vec<Task>,
}

fn clear() {
    execute!(
        stdout(),
        Hide,
        DisableBlinking,
        MoveTo(0, 0),
        Clear(ClearType::All)
    )
    .unwrap()
}

//TODO update macro to include a message
//fuck!("You forgot to add an argument!")
macro_rules! fuck {
    () => {
        quit::with_code(0);
    };
}

//TODO shouldn't this be optional?
//I feel like there are some weird cases were an empty stuct is made
//but no data gets put in it
//option might help

pub fn get_tasks() -> Data {
    let mut file = match File::open(&Config::current()) {
        Err(why) => panic!("couldn't open {}: ", why),
        Ok(file) => file,
    };
    let mut contents = String::new();
    let mut data = Data { tasks: Vec::new() };

    file.read_to_string(&mut contents).unwrap();
    if !contents.is_empty() {
        data = toml::from_str(&contents).unwrap();
    } else {
        return data;
    }

    return data;
}

pub fn write_toml(file_name: PathBuf, data: &Data) {
    let mut file = File::create(file_name).unwrap();
    let output = toml::to_string(&data).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn append_toml(file_name: PathBuf, data: &Data) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();

    let output = toml::to_string(&data).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn get_numbers(args: &Vec<String>) -> Vec<usize> {
    let mut numbers: Vec<usize> = Vec::new();

    let re = Regex::new(
        r"(?x)
    (?P<first>\d+)
    -
    (?P<last>\d+)
    ",
    )
    .unwrap();

    if args.len() == 1 {
        if let Some(caps) = re.captures(&args[0]) {
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
    }
    if args.len() >= 2 {
        if let Some(caps) = re.captures(&args[1]) {
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
    }

    for num in args {
        if let Ok(num) = num.parse::<usize>() {
            numbers.push(num - 1);
        }
    }

    return numbers;
}

pub fn add_task(args: Vec<String>) {
    let mut name: String = "Tasks".to_string();
    let arguments: String;
    let command = &args[0];

    //Get the board_name and task data
    if command.contains('!') {
        name = command.replace('!', "");
        arguments = args[1..].join(" ");
    } else {
        arguments = args[0..].join(" ");
    }

    let now: DateTime<Utc> = Utc::now();

    let task = Task {
        item: arguments,
        checked: false,
        board_name: name,
        note: false,
        date: now,
    };

    let data = Data { tasks: vec![task] };

    append_toml(Config::current(), &data);
}

pub fn add_note(args: Vec<String>) {
    let arguments = args[1..].join(" ");
    let now: DateTime<Utc> = Utc::now();

    let task = Task {
        item: arguments,
        checked: false,
        board_name: "Tasks".to_string(),
        note: true,
        date: now,
    };

    let data = Data { tasks: vec![task] };
    append_toml(Config::current(), &data);
}

pub fn check_task(args: Vec<String>) {
    let numbers = get_numbers(&args);

    if numbers.is_empty() {
        eprintln!("{} is not a valid number.", args[1]);
        fuck!();
    }

    let mut data = get_tasks();

    for id in numbers {
        if id > data.tasks.len() {
            eprintln!("'{}' is not a task!", id);
            fuck!();
        }
        data.tasks[id].checked = !data.tasks[id].checked;
    }

    write_toml(Config::current(), &data);
}

pub fn delete_task(args: Vec<String>) {
    let numbers = get_numbers(&args);

    if numbers.is_empty() {
        quit::with_code(1);
    }

    let mut data = get_tasks();

    //since we're deleting tasks the size will change
    let size = data.tasks.len();
    //this is annoying but again the size chagnes
    let mut indexes_removed = 0;

    for id in numbers {
        if id < size {
            data.tasks.remove(id - indexes_removed);
            indexes_removed += 1;
        } else if id != 0 {
            eprintln!("'{}' is not a task!", id);
            fuck!();
        }
    }

    if data.tasks.is_empty() {
        File::create(Config::current()).unwrap();
        fuck!();
    }

    write_toml(Config::current(), &data);
}

pub fn clear_tasks() {
    let mut data_to_append: Data = Data { tasks: Vec::new() };

    //Get finished tasks and put them in buffer
    let mut data = get_tasks();
    let mut indexes_removed = 0;

    //return if there are no tasks to clear
    if data.tasks.is_empty() {
        return;
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

    if data.tasks.is_empty() {
        File::create(Config::current()).unwrap();
    } else {
        write_toml(Config::current(), &data);
    }

    append_toml(Config::old(), &data_to_append);
}

pub fn tasks() {
    let data = get_tasks();

    if data.tasks.is_empty() {
        print::help_message();
        return;
    }

    let mut board_completed: HashMap<&str, usize> = HashMap::new();
    let mut board_total: HashMap<&str, usize> = HashMap::new();
    let mut board_list: Vec<&str> = Vec::new();

    let mut tasks_total = data.tasks.len();
    let mut tasks_completed = 0;
    let now: DateTime<Utc> = Utc::now();

    //Get a list of all boards
    for elem in data.tasks.iter() {
        board_list.push(elem.board_name.as_str());
        if elem.checked && !elem.note {
            tasks_completed += 1;
        }
    }

    //Remove repeated elements
    board_list = board_list.into_iter().unique().collect();

    //Get total and completed tasks for each board
    for board in &board_list {
        //boards completed and board total
        let (mut bc, mut bt) = (0, 0);

        for elem in data.tasks.iter() {
            if elem.board_name == *board {
                bt += 1;
                if elem.checked {
                    bc += 1;
                }
            }
        }

        //push the name and value into a hashmap
        board_completed.insert(board, bc);
        board_total.insert(board, bt);
    }

    //Remove the default board, we will print this last
    board_list.retain(|&x| x != "Tasks");

    let mut notes_total = 0;
    let mut index = 0;

    clear();

    //Print all the custom boards
    for board in board_list {
        print::header(board_completed[board], board_total[board], board);
        for elem in data.tasks.iter() {
            let day = (now - elem.date).num_days();
            if elem.board_name == board {
                index += 1;
                if elem.note {
                    print::note(index, elem.item.as_str(), tasks_total);
                    notes_total += 1;
                } else {
                    print::task(index, elem.checked, elem.item.as_str(), day, tasks_total);
                }
            }
        }
        println!();
    }

    //Print the header for the default board
    print::header(board_completed["Tasks"], board_total["Tasks"], "Tasks");

    //Print the default board
    for elem in data.tasks.iter() {
        if elem.board_name == "Tasks" {
            index += 1;
            let day = (now - elem.date).num_days();
            if elem.note {
                print::note(index, elem.item.as_str(), tasks_total);
                notes_total += 1;
            } else {
                print::task(
                    index,
                    elem.checked,
                    elem.item.as_str(),
                    day,
                    board_total["Tasks"],
                );
            }
        }
    }

    println!();

    //Don't count the notes
    tasks_total -= notes_total;

    print::footer(tasks_completed, tasks_total, notes_total);

    execute!(stdout(), Print("\n"), Show, EnableBlinking).unwrap();
}

pub fn old_tasks() {
    let mut file = match File::open(&Config::old()) {
        Err(why) => panic!("couldn't open {}: ", why),
        Ok(file) => file,
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let now: DateTime<Utc> = Utc::now();
    if !contents.is_empty() {
        let data: Data = toml::from_str(&contents).unwrap();
        let total_tasks = data.tasks.len();
        //how long ago the task was added in days
        for i in 0..data.tasks.len() {
            let day = (now - data.tasks[i].date).num_days();
            print::task(
                i + 1,
                data.tasks[i].checked,
                &data.tasks[i].item,
                day,
                total_tasks,
            );
        }
    } else {
        eprintln!("Task archive is empty.");
    }
}
