use std::{
    io::{StdoutLock, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

mod ui;

static mut STDOUT: Option<StdoutLock> = None;

#[derive(Debug)]
pub enum Item {
    Task(Task),
    Note(Note),
}

impl Item {
    pub fn board(&self) -> &str {
        match self {
            Item::Task(task) => &task.board,
            Item::Note(note) => &note.board,
        }
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match self {
            Item::Task(task) => task.to_string(),
            Item::Note(note) => note.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Task {
    pub board: String,
    pub text: String,
    pub checked: bool,
    pub date: u64,
}

impl ToString for Task {
    fn to_string(&self) -> String {
        let board = if self.board.is_empty() {
            String::new()
        } else {
            format!(".{}", self.board)
        };
        format!(
            "[task{}]\n{}\n{}\n{}\n",
            board, self.text, self.checked, self.date
        )
    }
}

#[derive(Debug)]
pub struct Note {
    pub text: String,
    pub board: String,
    pub date: u64,
}

impl ToString for Note {
    fn to_string(&self) -> String {
        let board = if self.board.is_empty() {
            String::new()
        } else {
            format!(".{}", self.board)
        };
        format!("[note{}]\n{}\n{}\n", board, self.text, self.date)
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn total_tasks(config: &[Item]) -> usize {
    config
        .iter()
        .filter(|item| matches!(item, Item::Task(_)))
        .count()
}

pub fn print(config: &[Item]) {
    let total = config.len();

    if total == 0 {
        return ui::empty_tasks();
    }

    let total_tasks = total_tasks(config);

    let total_notes = config
        .iter()
        .filter(|item| matches!(item, Item::Note(_)))
        .count();

    let total_checked = config
        .iter()
        .filter(|item| match item {
            Item::Task(task) => task.checked,
            Item::Note(_) => false,
        })
        .count();

    let now = now();

    let mut current_board = None;

    ui::clear();

    for (i, item) in config.iter().enumerate() {
        if current_board.is_none() || current_board != Some(item.board()) {
            if current_board.is_some() {
                queue!("\n");
            }

            current_board = Some(item.board());
            let total_tasks = config
                .iter()
                .filter(|item| Some(item.board()) == current_board)
                .filter(|item| matches!(item, Item::Task(_)))
                .count();

            let total_checked = config
                .iter()
                .filter(|item| Some(item.board()) == current_board)
                .filter(|item| match item {
                    Item::Task(task) => task.checked,
                    Item::Note(_) => false,
                })
                .count();

            ui::header(total_checked, total_tasks, item.board());
        }
        match item {
            Item::Task(task) => {
                let days = ((now - task.date) as f64 * 0.000011574).round() as u64;
                ui::task(i + 1, task.checked, &task.text, days, total);
            }
            Item::Note(note) => ui::note(i + 1, &note.text, total_notes),
        }
    }

    ui::footer(total_checked, total_tasks, total_notes);
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
        if let Ok(num) = end_str.parse() {
            end = Some(num);
        } else {
            return None;
        }
    }

    Some((start.unwrap(), end.unwrap()))
}

fn ids(config: &[Item], args: &[String]) -> Result<Vec<usize>, String> {
    let args = if args.iter().any(|arg| arg == &String::from('d')) {
        &args[1..]
    } else {
        &args[0..]
    }
    .join(" ")
    .trim()
    .to_string();

    if is_row_of_numbers(&args) {
        let mut ids: Vec<usize> = Vec::new();
        for num in args.split(' ') {
            let num = num.parse().unwrap();
            if num > config.len() || num == 0 {
                return Err(format!("Task '{}' doesn't exist!", num));
            } else {
                ids.push(num);
            }
        }
        ids.sort();
        Ok(ids)
    } else if let Some((first, last)) = get_range(&args) {
        if first > last {
            Err(format!("'{}' must be smaller than '{}'!", first, last))
        } else if last > total_tasks(config) {
            Err(format!("Task '{}' doesn't exist!", last))
        } else if first == 0 {
            Err(format!("Task '{}' doesn't exist!", first))
        } else {
            Ok((first..=last).collect())
        }
    } else {
        Ok(Vec::new())
    }
}

fn add(config: &mut Vec<Item>, args: &[String], is_note: bool) -> Result<(), &'static str> {
    let args = if is_note { &args[1..] } else { args };
    let mut board = String::new();

    let text = if args[0].contains('!') {
        if args.len() > 1 {
            //A task with a board.
            board = args[0].replace('!', "");
            args[1..].join(" ")
        } else {
            return Err("Entered a board without a task!");
        }
    } else {
        //A task without a board.
        args[0..].join(" ")
    };

    let text = text.replace('\n', "");
    let board = board.replace('\n', "");

    if is_note {
        config.push(Item::Note(Note {
            text,
            board,
            date: now(),
        }));
    } else {
        config.push(Item::Task(Task {
            text,
            board,
            checked: false,
            date: now(),
        }));
    }
    Ok(())
}

fn parse_config(config_string: &str) -> Vec<Item> {
    let mut items = Vec::new();
    for item in config_string.split('[') {
        let mut lines = item.split('\n');

        let mut task = true;

        let mut content = String::new();
        let mut board = String::new();
        let mut checked = false;
        let mut date = 0;

        //Get the type of item and board name.
        if let Some(line) = lines.next() {
            if line.contains("task") {
                task = true;
            } else if line.contains("note") {
                task = false;
            } else {
                continue;
            }

            let header: String = line.split(']').collect();
            let mut temp_board = header.split('.');
            temp_board.next();
            if let Some(temp_board) = temp_board.next() {
                board = temp_board.to_string();
            }
        }

        //The content of the item.
        if let Some(line) = lines.next() {
            content = line.to_string();
        }

        //Check if the task is checked.
        if task {
            if let Some(line) = lines.next() {
                if line == "false" {
                    checked = false;
                } else if line == "true" {
                    checked = true;
                } else {
                    continue;
                }
            }
        }

        //Get the items timestamp.
        if let Some(line) = lines.next() {
            if let Ok(num) = line.parse::<u64>() {
                date = num;
            } else {
                continue;
            }
        }

        if task {
            items.push(Item::Task(Task {
                board,
                text: content,
                checked,
                date,
            }));
        } else {
            items.push(Item::Note(Note {
                board,
                text: content,
                date,
            }));
        }
    }
    items.sort_by(|a, b| a.board().cmp(b.board()));
    items
}

fn handle_arguments(args: Vec<String>, config: &mut Vec<Item>) -> Result<(), String> {
    match args[0].as_str() {
        "-h" | "-help" => return Err(ui::HELP.to_string()),
        "-v" | "-version" => return Err(format!("t {}", env!("CARGO_PKG_VERSION"))),
        "n" | "d" if args.len() == 1 => {
            ui::missing_args(&args[0]);
            return Ok(());
        }
        "n" => add(config, &args, true)?,
        "d" => {
            let ids = ids(config, &args)?;
            if ids.is_empty() {
                return Err(format!("Task '{}' does not exist!", args[1..].join(" ")));
            }
            for id in ids.into_iter().rev() {
                if id == 0 {
                    continue;
                }
                config.remove(id - 1);
            }
        }
        "cls" => config.retain(|item| match item {
            Item::Task(task) => !task.checked,
            Item::Note(_) => true,
        }),
        _ if args[0].starts_with('-') => return Err("Invalid command.".to_string()),
        _ => {
            let ids = ids(config, &args)?;
            if ids.is_empty() {
                add(config, &args, false)?
            } else {
                for id in ids {
                    let item = config.get_mut(id - 1).unwrap();
                    match item {
                        Item::Task(task) => task.checked = !task.checked,
                        Item::Note(_) => (),
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    unsafe {
        let stdout = std::io::stdout();
        let handle = stdout.lock();
        STDOUT = Some(handle);
    }

    let t = if cfg!(windows) {
        PathBuf::from(&std::env::var("APPDATA").unwrap())
    } else {
        PathBuf::from(&std::env::var("HOME").unwrap()).join(".config")
    }
    .join("t");
    std::fs::create_dir_all(&t).unwrap();

    let config_path = t.join("t.ini");
    let config_string = match std::fs::read_to_string(&config_path) {
        Ok(config) => config,
        Err(_) => String::new(),
    };
    let mut config = parse_config(&config_string);
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        print(&config);
    } else {
        let result = handle_arguments(args, &mut config);
        config.sort_by(|a, b| a.board().cmp(b.board()));

        if let Err(err) = result {
            queue!("{}", err);
        } else {
            print(&config);
        }

        //Save config
        let mut output = String::new();
        for item in config {
            output.push_str(&item.to_string());
            output.push('\n');
        }
        output.pop();
        output.pop();
        std::fs::write(config_path, &output).unwrap();
    }

    std::io::stdout().flush().unwrap();
}
