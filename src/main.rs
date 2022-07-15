use database::*;
use rusqlite::Connection;
use std::{
    env, fs,
    io::{StdoutLock, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

mod database;
mod ui;

fn print(conn: &Connection) {
    let total_tasks = total_tasks(conn);
    let total_notes = total_notes(conn);
    let total = total(conn);

    if total == 0 {
        return ui::help_message();
    }

    let total_checked = total_checked(conn);
    let boards = get_boards(conn);
    let mut i = 1;

    ui::clear();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    for board in boards {
        ui::header(board.checked, board.total, &board.name);

        for task in board.tasks {
            if task.note {
                ui::note(i, &task.content, total);
            } else {
                let days = ((now - task.date) as f64 * 0.000011574).round() as u64;
                ui::task(i, task.checked, &task.content, days, total);
            }
            i += 1;
        }

        queue!("\n");
    }

    ui::footer(total_checked, total_tasks, total_notes);
}

fn print_old(conn: &Connection) {
    let old_tasks = get_old(conn);
    if old_tasks.is_empty() {
        return println!("No old tasks.");
    }

    ui::clear();
    ui::old_header();

    for (i, task) in old_tasks.iter().enumerate() {
        ui::note(i + 1, task, old_tasks.len());
    }
}

fn is_row_of_numbers(input: &str) -> bool {
    for char in input.chars() {
        match char {
            ' ' => (),
            _ if char.is_numeric() => (),
            _ => return false,
        }
    }
    true
}

fn get_range(input: &str) -> Option<(usize, usize)> {
    let mut start_str = String::new();
    let mut start = None;

    let mut end_str = String::new();
    let mut end = None;

    if !input.contains('-') {
        return None;
    }

    for char in input.chars() {
        match char {
            '-' | ' ' => {
                if !start_str.is_empty() && start.is_none() {
                    start = Some(start_str.parse::<usize>().unwrap());
                }

                if !end_str.is_empty() && end.is_none() {
                    end = Some(end_str.parse::<usize>().unwrap());
                }
            }
            _ if char.is_numeric() => {
                if start.is_none() {
                    start_str.push(char);
                } else if end.is_none() {
                    end_str.push(char);
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    if end.is_none() {
        end = Some(end_str.parse::<usize>().unwrap());
    }

    Some((start.unwrap(), end.unwrap()))
}

fn ids(args: &[String], conn: &Connection) -> Result<Vec<usize>, Option<&'static str>> {
    let args = if args.iter().any(|arg| arg == &String::from('d')) {
        &args[1..]
    } else {
        &args[0..]
    }
    .join(" ")
    .trim()
    .to_string();

    if is_row_of_numbers(&args) {
        args.split(' ')
            .map(|str| {
                if let Ok(num) = str.parse() {
                    if num > total(conn) {
                        Err(Some("Task does not exist."))
                    } else {
                        Ok(num)
                    }
                } else {
                    Err(Some("Invalid number."))
                }
            })
            .collect()
    } else if let Some((first, last)) = get_range(&args) {
        if first > last {
            Err(Some(
                "Invalid range! First number must be smaller than last.",
            ))
        } else if last > total_tasks(conn) {
            Err(Some("Task does not exist."))
        } else {
            Ok((first..=last).collect())
        }
    } else {
        Err(None)
    }
}

fn add(args: &[String], conn: &Connection, is_note: bool) -> Result<(), &'static str> {
    let args = if is_note { &args[1..] } else { args };
    let mut board_name = None;

    let item = if args[0].contains('!') {
        if args.len() >= 2 {
            //t !Task 'sample task'
            board_name = Some(args[0].replace('!', ""));
            args[1..].join(" ")
        } else {
            let input: Vec<&str> = args[0].split(' ').collect();

            //t '!Tasks'
            if input.len() == 1 {
                return Err("Missing task!");
            }

            //t '!Tasks sample task'
            board_name = Some(input[0].replace('!', ""));
            input[1..].join(" ")
        }
    } else {
        //t 'sample task'
        args[0..].join(" ")
    };

    if is_note {
        insert_note(conn, &item, board_name);
    } else {
        insert_task(conn, &item, board_name);
    }
    Ok(())
}

static mut STDOUT: Option<StdoutLock> = None;

#[macro_export]
macro_rules! queue {
    ($($arg:tt)*) => {
        unsafe {
            {
                use std::io::Write;

                let stdout = $crate::STDOUT.as_mut().unwrap();
                let args = format_args!($($arg)*).to_string();
                stdout
                    .write_all(format!("{}\x1b[0m", args).as_bytes())
                    .unwrap();
            }
        };
    };
}

fn main() {
    unsafe {
        let stdout = std::io::stdout();
        let handle = stdout.lock();
        STDOUT = Some(handle);
    }

    let args: Vec<String> = std::env::args().skip(1).collect();
    let t = if cfg!(windows) {
        PathBuf::from(&env::var("APPDATA").unwrap())
    } else {
        PathBuf::from(&env::var("HOME").unwrap()).join(".config")
    }
    .join("t");

    fs::create_dir_all(&t).unwrap();
    let db = t.join("t.db");
    let conn = Connection::open(db).unwrap();
    conn.execute_batch(
        "
            CREATE TABLE IF NOT EXISTS tasks(
                content TEXT NOT NULL,
                checked BOOL NOT NULL,
                note BOOL NOT NULL,
                board TEXT NOT NULL,
                date INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS old(
                content TEXT NOT NULL
            );
        ",
    )
    .unwrap();

    if args.is_empty() {
        print(&conn);
    } else {
        match args[0].as_str() {
            "-h" | "-help" => return ui::help(),
            "-v" | "-version" => return println!("t {}", env!("CARGO_PKG_VERSION")),
            "n" | "d" if args.len() == 1 => return ui::missing_args(&args[0]),
            "o" | "old" => return print_old(&conn),
            "n" => {
                if let Err(err) = add(&args, &conn, true) {
                    return println!("{}", err);
                }
            }
            "d" => match ids(&args, &conn) {
                Ok(ids) => delete_tasks(&conn, &ids),
                Err(err) => return println!("{}", err.unwrap_or("")),
            },
            "cls" => clear_tasks(&conn),
            _ if args[0].starts_with('-') => return println!("Invalid command."),
            _ => match ids(&args, &conn) {
                Ok(ids) => check_tasks(&conn, &ids),
                //error with numbers or task?
                Err(err) => match err {
                    Some(err) => return println!("{}", err),
                    None => {
                        //check for for input errors
                        if let Err(err) = add(&args, &conn, false) {
                            return println!("{}", err);
                        }
                    }
                },
            },
        }

        print(&conn);
    }

    std::io::stdout().flush().unwrap();
}
