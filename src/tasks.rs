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
use regex::{Captures, Regex};

use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
use std::path::PathBuf;

use crate::config::Config;
use crate::data::Data;
use crate::print;

type Board<'a> = HashMap<&'a str, usize>;

fn clear_console() {
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
#[macro_export]
macro_rules! fuck {
    () => {
        quit::with_code(0);
    };
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

fn get_numbers(args: &[String]) -> Vec<usize> {
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

    if args.len() == 1 {
        caps = re.captures(&args[0]);
    } else if args.len() >= 2 {
        caps = re.captures(&args[1]);
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

    for num in args {
        if let Ok(num) = num.parse::<usize>() {
            if num != 0 {
                numbers.push(num - 1);
            }
        }
    }

    return numbers;
}

pub fn add_task(args: Vec<String>) {
    let mut board_name = String::from("Tasks");
    let item: String;

    let date: DateTime<Utc> = Utc::now();
    if args.len() > 1 {
        let command = &args[1];

        if command.contains('!') {
            board_name = command.replace('!', "");
            item = args[2..].join(" ");
        } else {
            item = args[1..].join(" ");
        }

        let data = Data::from(item, false, board_name.as_str(), false, date);
        dbg!(&data);

        append_toml(Config::current(), &data);
    } else {
        let command = args[0].clone();
    }
}

pub fn add_note(args: Vec<String>) {
    let item = args[1..].join(" ");
    let date: DateTime<Utc> = Utc::now();

    let data = Data::from(item, false, "Tasks", true, date);

    append_toml(Config::current(), &data);
}

pub fn check_task(args: Vec<String>) {
    let numbers = get_numbers(&args);

    if numbers.is_empty() {
        eprintln!("{} is not a valid number.", args[1]);
        fuck!();
    }

    let mut data = Data::tasks();

    for id in numbers {
        if id > data.len() {
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
        eprintln!("{} is not a valid number.", args[1]);
        fuck!();
    }

    let mut data = Data::tasks();

    //since we're deleting tasks the size will change
    let size = data.len();

    //this is annoying but again the size chagnes
    let mut indexes_removed = 0;

    for id in numbers {
        if id < size {
            data.remove(id - indexes_removed);
            indexes_removed += 1;
        } else if id != 0 {
            eprintln!("'{}' is not a task!", id);
            fuck!();
        }
    }

    if data.is_empty() {
        File::create(Config::current()).unwrap();
        eprintln!("No tasks WTF?");
        fuck!();
    }

    write_toml(Config::current(), &data);
}

pub fn clear_tasks() {
    let mut data_to_append = Data::new();

    //Get finished tasks and put them in buffer
    let mut data = Data::tasks();
    let mut indexes_removed = 0;

    //return if there are no tasks to clear
    if data.is_empty() {
        return;
    }

    //Copy checked tasks to new file
    for i in 0..data.len() {
        if data.tasks[i - indexes_removed].checked {
            data_to_append.push(&data.tasks[i - indexes_removed]);
            data.remove(i - indexes_removed);
            indexes_removed += 1;
        }
    }

    if data.is_empty() {
        File::create(Config::current()).unwrap();
    } else {
        write_toml(Config::current(), &data);
    }

    append_toml(Config::old(), &data_to_append);
}

pub fn tasks() {
    let data = Data::tasks();

    if data.is_empty() {
        print::help_message();
        return;
    }

    let mut board_completed = Board::new();
    let mut board_total = Board::new();

    let mut board_list: Vec<&str> = Vec::new();

    let mut tasks_total = data.len();
    let mut tasks_completed = 0;
    let now: DateTime<Utc> = Utc::now();

    //Get a list of all boards
    for task in data.iter() {
        board_list.push(task.board_name.as_str());
        if task.checked && !task.note {
            tasks_completed += 1;
        }
    }

    //Remove repeated elements
    board_list = board_list.into_iter().unique().collect();

    //Get total and completed tasks for each board
    for board in &board_list {
        //boards completed and board total
        let (mut bc, mut bt) = (0, 0);

        for task in data.iter() {
            if task.board_name == *board {
                bt += 1;
                if task.checked {
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

    clear_console();

    //Print the header for the default board
    print::header(board_completed["Tasks"], board_total["Tasks"], "Tasks");

    //Print the default board
    for task in data.iter() {
        if task.board_name == "Tasks" {
            index += 1;
            let day = (now - task.date).num_days();
            if task.note {
                print::note(index, task.item.as_str(), tasks_total);
                notes_total += 1;
            } else {
                print::task(
                    index,
                    task.checked,
                    task.item.as_str(),
                    day,
                    board_total["Tasks"],
                );
            }
        }
    }

    println!();

    //Print all the custom boards
    for board in board_list {
        print::header(board_completed[board], board_total[board], board);
        for elem in data.iter() {
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

    //Don't count the notes in footer
    tasks_total -= notes_total;

    print::footer(tasks_completed, tasks_total, notes_total);

    execute!(stdout(), Print("\n"), Show, EnableBlinking).unwrap();
}

pub fn old_tasks() {
    let mut file = File::open(&Config::old()).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let now: DateTime<Utc> = Utc::now();

    if contents.is_empty() {
        eprintln!("Task archive is empty.");
        fuck!();
    } else {
        let data: Data = toml::from_str(&contents).unwrap();
        let total_tasks = data.len();

        for i in 0..total_tasks {
            //how long ago the task was added in days
            let day = (now - data.tasks[i].date).num_days();

            print::task(
                i + 1,
                data.tasks[i].checked,
                &data.tasks[i].item,
                day,
                total_tasks,
            );
        }
    }
}
