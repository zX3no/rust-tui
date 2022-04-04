use crate::database::Database;
use crate::ui;
use chrono::{TimeZone, Utc};
use regex::Regex;
use static_init::dynamic;

#[dynamic]
static ARGS: Vec<String> = std::env::args().skip(1).collect();

pub struct App {
    pub db: Database,
}

impl App {
    pub fn run() {
        Self {
            db: Database::new(),
        }
        .parse_args()
    }
    pub fn parse_args(&self) {
        match ARGS.len() {
            0 => self.print_tasks(),
            _ => {
                match ARGS[0].as_str() {
                    "n" | "d" if ARGS.len() == 1 => return ui::missing_args(ARGS[0].as_str()),
                    "h" | "help" => return ui::help(),
                    "v" | "version" => return println!("t {}", env!("CARGO_PKG_VERSION")),
                    "o" | "old" => return self.print_old(),
                    "n" => {
                        if let Err(err) = self.add(true) {
                            return println!("{err}");
                        }
                    }
                    "d" => match self.ids() {
                        Ok(ids) => self.db.delete_tasks(&ids),
                        Err(err) => return println!("{err}"),
                    },
                    "cls" => return self.clear_tasks(),
                    _ => match self.ids() {
                        Ok(ids) => self.db.check_tasks(&ids),
                        Err(err) => {
                            if !err.is_empty() {
                                return println!("{err}");
                            } else if let Err(err) = self.add(false) {
                                return println!("{err}");
                            }
                        }
                    },
                }
                self.print_tasks();
            }
        }
    }
    pub fn print_tasks(&self) {
        let total_tasks = self.db.total_tasks();
        let total_checked = self.db.total_checked();
        let total_notes = self.db.total_notes();

        if total_tasks == 0 {
            return ui::help_message();
        }

        let boards = self.db.get_boards();

        ui::clear();

        let mut i = 1;

        for board in boards {
            ui::header(board.checked, board.total, &board.name);
            for task in board.tasks {
                if task.note {
                    ui::note(i, &task.content, total_tasks);
                } else {
                    let date = Utc
                        .datetime_from_str(&task.date, "%Y-%m-%d %H:%M:%S")
                        .unwrap();
                    let days = (Utc::now() - date).num_days();
                    ui::task(i, task.checked, &task.content, days, total_tasks);
                }
                i += 1;
            }
            ui::new_line();
        }

        ui::footer(total_checked, total_tasks, total_notes);
    }
    fn print_old(&self) {
        let old_tasks = self.db.get_old();
        if old_tasks.is_empty() {
            return println!("No old tasks.");
        }

        ui::clear();

        ui::old_header();

        for (i, task) in old_tasks.iter().enumerate() {
            ui::note(i + 1, task, old_tasks.len());
        }
    }
    fn ids(&self) -> Result<Vec<usize>, &str> {
        let args = if ARGS.iter().any(|arg| arg == &String::from('d')) {
            ARGS[1..].to_owned()
        } else {
            ARGS[0..].to_owned()
        };

        let join = args.join(" ");
        let input = join.trim();

        let single_number = Regex::new("^[0-9 ]*$").unwrap();
        let number_range = Regex::new(r"^(?x)(?P<first>\d+)-(?P<last>\d+)$").unwrap();

        let len = self.db.total_tasks();

        if let Some(caps) = single_number.captures(input) {
            caps.get(0)
                .unwrap()
                .as_str()
                .split(' ')
                .map(|str| {
                    if let Ok(num) = str.parse::<usize>() {
                        if num <= len {
                            Ok(num)
                        } else {
                            Err("Task does not exist")
                        }
                    } else {
                        Err("Invalid number.")
                    }
                })
                .collect()
        } else if let Some(caps) = number_range.captures(input) {
            let first = caps["first"].parse::<usize>().unwrap();
            let last = caps["last"].parse::<usize>().unwrap();

            if first > last {
                return Err("Invalid range! First number must be smaller than last.");
            }

            Ok((first..last + 1).collect())
        } else {
            //weird hack, when err.is_empty() i can trigger something else
            Err("")
        }
    }
    fn add(&self, is_note: bool) -> Result<(), &str> {
        let args = if is_note { &ARGS[1..] } else { &ARGS };
        let mut board_name = None;

        if args.is_empty() {
            return Err("No arguments provided.");
        }

        let item = if args.len() >= 2 {
            if args[0].contains('!') {
                //t !board 'task ...'
                board_name = Some(args[0].replace('!', ""));
                args[1..].join(" ")
            } else {
                //t 'long task ...'
                args[0..].join(" ")
            }
        } else if args[0].contains('!') {
            let strs: Vec<&str> = args[0].split(' ').collect();

            //t '!board'
            if strs.len() == 1 {
                return Err("Missing task!");
            } else {
                //t '!board task ...'
                board_name = Some(strs[0].replace('!', ""));
                strs[1..].join(" ")
            }
        } else {
            //t 'task'
            args[0..].join(" ")
        };

        if is_note {
            self.db.insert_note(&item, board_name);
        } else {
            self.db.insert_task(&item, board_name);
        }
        Ok(())
    }
    fn clear_tasks(&self) {
        if self.db.total_checked() == 0 {
            println!("Not tasks to clear!");
        } else {
            self.db.clear_tasks().unwrap();
            if self.db.total_tasks() != 0 {
                self.print_tasks();
            }
        }
    }
}
