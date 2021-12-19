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
    pub fn run() {
        Self {
            db: Database::new(),
        }
        .parse_args()
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
            return;
        }

        ui::clear();

        //TODO: old header?
        // ui::header(0, old_tasks.len(), "Tasks");
        for (i, task) in old_tasks.iter().enumerate() {
            ui::note(i + 1, task, old_tasks.len());
        }
    }
    fn ids() -> Option<Vec<usize>> {
        let re = Regex::new("^[0-9 ]*$").unwrap();

        let mut args = ARGS.join(" ");
        if let Some(char) = args.chars().next() {
            if char == 'd' {
                args.remove(0);
            }
        }
        if re.captures(&args).is_some() {
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
    fn add(&self, is_note: bool) {
        let args = if is_note { &ARGS[1..] } else { &ARGS };
        let mut board_name = None;

        if args.is_empty() {
            panic!("this should not have happend?");
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
                panic!("Missing task!");
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
    }
    pub fn parse_args(&self) {
        match ARGS.len() {
            0 => self.print_tasks(),
            _ => match ARGS[0].as_str() {
                "n" | "d" if ARGS.len() == 1 => return ui::missing_command(ARGS[0].as_str()),
                "h" | "help" => return ui::help(),
                "v" | "version" => return println!("t {}", env!("CARGO_PKG_VERSION")),
                "o" | "old" => return self.print_old(),
                "n" => self.add(true),
                "d" => {
                    if let Some(ids) = App::ids() {
                        self.db.delete_tasks(&ids);
                    }
                }
                "cls" => self.db.clear_tasks().unwrap(),
                _ => {
                    if let Some(ids) = App::ids() {
                        self.db.check_tasks(&ids);
                    } else {
                        self.add(false);
                    }
                }
            },
        }
        self.print_tasks();
    }
}
