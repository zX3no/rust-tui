use crate::database::Database;
use crate::ui;
use chrono::{TimeZone, Utc};
use regex::Regex;

lazy_static! {
    static ref ARGS: Vec<String> = std::env::args().skip(1).collect();
}
pub struct App {
    pub db: Database,
}

impl App {
    pub fn new() -> Self {
        Self {
            db: Database::new(),
        }
    }
    pub fn print_tasks(&self) {
        let total_tasks = self.db.total_tasks();
        let total_checked = self.db.total_checked();
        let total_notes = self.db.total_notes();

        if total_tasks == 0 {
            ui::help_message();
            return;
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
    fn ids() -> Option<Vec<usize>> {
        let re = Regex::new("^[0-9 ]*$").unwrap();

        let mut args = ARGS.join(" ");
        if let Some(char) = args.chars().next() {
            if char == 'd' {
                args.remove(0);
            }
        }
        if let Some(_) = re.captures(&args) {
            return Some(ARGS.iter().flat_map(|arg| arg.parse::<usize>()).collect());
        }

        let re = Regex::new(r"(?x)(?P<first>\d+)-(?P<last>\d+)").unwrap();

        if let Some(caps) = re.captures(&ARGS.join(" ")) {
            let first = caps["first"].parse::<usize>().unwrap();
            let last = caps["last"].parse::<usize>().unwrap();

            if first > last {
                println!("Invalid range! First number must be smaller than last.");
                return None;
            }

            return Some((first..last + 1).collect());
        }
        None
    }
    fn add(&self, is_note: bool, skip_command: bool) {
        let args = if skip_command { &ARGS[1..] } else { &ARGS };
        let mut board_name = None;
        let item = if args.len() >= 2 {
            if args[0].contains('!') {
                //t !board 'task ...'
                board_name = Some(args[0].replace('!', ""));
                args[1..].join(" ")
            } else {
                //t 'long task ...'
                args[0..].join(" ")
            }
        } else if !args.is_empty() {
            if args[0].contains('!') {
                //t !board
                panic!("Missing task!");
            } else {
                //t 'task'
                args[0..].join(" ")
            }
        } else {
            panic!("This should not have happend!");
        };

        if is_note {
            self.db.insert_note(&item, board_name);
        } else {
            self.db.insert_task(&item, board_name);
        }
    }
    fn add_task_short(&self) {
        self.add(false, false)
    }
    fn add_note(&self) {
        self.add(true, true);
    }
    pub fn parse_args(&self) {
        match ARGS.len() {
            0 => {
                self.print_tasks();
            }
            _ => {
                let mut numbers = false;
                ARGS.iter().for_each(|arg| {
                    arg.chars().for_each(|char| {
                        if char.is_numeric() {
                            numbers = true;
                        } else if char != '-' {
                            numbers = false;
                        }
                    })
                });
                if ARGS.len() == 1 {
                    if let "d" | "n" | "c" = ARGS[0].as_str() {
                        println!("Missing arguments for '{}'", &ARGS[0]);
                        return;
                    }
                }
                match ARGS[0].as_str() {
                    "h" | "help" | "--help" => {
                        ui::help();
                        return;
                    }
                    "v" | "version" | "--version" => {
                        println!("t {}", env!("CARGO_PKG_VERSION"));
                        return;
                    }
                    "d" => {
                        if let Some(ids) = App::ids() {
                            self.db.delete_tasks(&ids);
                        }
                    }
                    "cls" => self.db.clear_tasks().unwrap(),
                    "n" => self.add_note(),
                    _ => {
                        if let Some(ids) = App::ids() {
                            self.db.check_tasks(&ids);
                        } else {
                            self.add_task_short();
                        }
                    }
                };

                self.print_tasks();
            }
        }
    }
}
