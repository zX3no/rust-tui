use database::Database;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

mod config;
mod database;
mod date_format;
mod print;
mod task;

//TODO:
//refactor and simplify code
//better argument handling

fn get_numbers() -> Vec<usize> {
    let args: Vec<String> = std::env::args().skip(1).collect();

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

    let caps = if args.len() == 1 {
        re.captures(&args[0])
    } else if args.len() >= 2 {
        re.captures(&args[1])
    } else {
        panic!("no arguments?");
    };

    if let Some(caps) = caps {
        //t 1-10
        let first = caps["first"].parse::<usize>().unwrap();
        let last = caps["last"].parse::<usize>().unwrap();

        if first > last {
            return Vec::new();
        }

        (first..last + 1).collect()
    } else {
        //t 1 2 3 4
        args.iter().flat_map(|arg| arg.parse::<usize>()).collect()
    }
}

fn print_tasks(db: &Database) {
    let tasks = db.get_tasks();
    let len = tasks.len();
    if tasks.is_empty() {
        print::help_message();
        return;
    }
    let checked = db.get_checked().unwrap().len();
    print::header(checked, len, "Tasks");
    for task in tasks {
        print::task(task.id, task.checked, &task.content, 0, len);
    }

    print::footer(checked, len, 0);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let db = Database::new();
    //ui struct with 'a ref to db?

    match args.len() {
        0 => {
            print_tasks(&db);
        }
        _ => {
            let mut numbers = false;

            for num in &args {
                for char in num.chars() {
                    if char.is_numeric() {
                        numbers = true;
                        continue;
                    } else if char != '-' {
                        numbers = false;
                        continue;
                    }
                }
            }

            match &args[0] as &str {
                "h" | "--help" | "help" => print::help(),
                _ => (),
            }
            if args.len() == 1 {
                match &args[0] as &str {
                    "a" | "d" | "n" | "c" => {
                        println!("Missing arguments for '{}'", &args[0]);
                        return;
                    }
                    _ => (),
                }
            }

            let command = args[0].as_str();
            let data = args[1..].to_owned();
            match command {
                "a" => {
                    let mut board_name = None;

                    let item = if data.len() >= 2 {
                        if data[0].contains('!') {
                            //t !board 'task ...'
                            board_name = Some(data[0].replace('!', ""));
                            data[1..].join(" ")
                        } else {
                            //t 'long task ...'
                            data[0..].join(" ")
                        }
                    } else if !args.is_empty() {
                        if data[0].contains('!') {
                            //t !board
                            panic!("Missing task!");
                        } else {
                            //t 'task'
                            data[0..].join(" ")
                        }
                    } else {
                        panic!("This should not have happend!");
                    };
                    db.insert_task(&item, board_name);
                }
                "d" => db.delete_tasks(&get_numbers()),
                "c" => db.check_tasks(&get_numbers()),
                "cls" => db.clear_tasks().unwrap(),
                "n" => db.insert_note(&data.join(" ")),
                _ => {
                    let data = &args[0..];
                    if numbers {
                        db.check_tasks(&get_numbers());
                    } else {
                        db.insert_task(&data.join(" "), None);
                    }
                }
            };

            print_tasks(&db);
        }
    }
}
