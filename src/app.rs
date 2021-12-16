use crate::database::Database;
use crate::print;
use regex::Regex;

lazy_static! {
    static ref ARGS: Vec<String> = std::env::args().skip(1).collect();
}
pub struct App {
    db: Database,
}

impl App {
    pub fn new() -> Self {
        Self {
            db: Database::new(),
        }
    }
    pub fn print_tasks(&self) {
        let tasks = self.db.get_tasks();
        let checked = self.db.get_checked().unwrap().len();
        let len = tasks.len();

        if tasks.is_empty() {
            print::help_message();
            return;
        }

        let mut prev_board = String::new();
        //TODO: date and notes
        for task in tasks {
            if prev_board.is_empty() {
                prev_board = task.board;
                print::header(checked, len, &prev_board)
            } else if prev_board != task.board {
                prev_board = task.board;
                print::header(checked, len, &prev_board)
            }
            print::task(task.id, task.checked, &task.content, 0, len);
        }

        print::footer(checked, len, 0);
    }
    fn ids() -> Vec<usize> {
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
    pub fn add_task(&self, is_note: bool) {
        let args: Vec<String> = std::env::args().skip(1).collect();

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
                    if let "a" | "d" | "n" | "c" = ARGS[0].as_str() {
                        println!("Missing arguments for '{}'", &ARGS[0]);
                        return;
                    }
                }
                match ARGS[0].as_str() {
                    "h" | "--help" | "help" => {
                        print::help();
                        return;
                    }
                    "a" => self.add_task(false),
                    "d" => self.db.delete_tasks(&App::ids()),
                    "c" => self.db.check_tasks(&App::ids()),
                    "cls" => self.db.clear_tasks().unwrap(),
                    "n" => self.add_task(true),
                    _ => {
                        if numbers {
                            self.db.check_tasks(&App::ids());
                        } else {
                            self.add_task(false);
                        }
                    }
                };

                self.print_tasks();
            }
        }
    }
}
