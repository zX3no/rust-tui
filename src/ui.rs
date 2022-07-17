#![allow(dead_code)]
use crate::queue;
use std::process::Command;

pub const RESET: &str = "\x1b[0m";
pub const WHITE: &str = "\x1b[37m";
pub const GREY: &str = "\x1b[90m";
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const BLUE: &str = "\x1b[34m";
pub const YELLOW: &str = "\x1b[33m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const DIM: &str = "\x1b[2m";
pub const BOLD: &str = "\x1b[1m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINED: &str = "\x1b[4m";
pub const BUNDERLINE: &str = "\x1B[1;4m";
pub const STRICKEN: &str = "\x1B[9m";
pub const UNDERSCORE: &str = "\x1b[4m";
pub const BLINK: &str = "\x1b[5m";
pub const REVERSE: &str = "\x1b[7m";
pub const HIDDEN: &str = "\x1b[8m";
pub const HELP: &str = "
Usage
    t [<options> <args>] 

Options
    none                    Display a list of tasks.
    none+args               Create a task
    none+number             Check/uncheck task

    n                       Add a note
    d                       Delete a task
    cls                     Delete all checked tasks
    o, old                  Displays deleted tasks 
    -h, -help               Displays the help page
    -v, -version            Displays version

Examples                     
    t                       Displays tasks
    t example task          Creates a task 'example task'
    t !TODO example task    Create a task in a board called 'TODO'        
    t 1 2 3                 Checks task 1, 2 and 3
    t 1-3                   Checks task 1, 2 and 3
    t n example note        Create note 'example note'
    t n !TODO example task  Create a note in a board called 'TODO'        
    t d 1                   Deletes task 1
    t d 1-3                 Deletes task 1, 2 and 3";

#[macro_export]
macro_rules! queue {
    ($($arg:tt)*) => {
        unsafe {
            use std::io::Write;
            let stdout = $crate::STDOUT.as_mut().unwrap();
            let args = format_args!($($arg)*).to_string();
            stdout
                .write_all(format!("{}\x1b[0m", args).as_bytes())
                .unwrap();
        };
    };
}

#[cfg(windows)]
pub fn clear() {
    Command::new("cmd").args(["/C", "cls"]).status().unwrap();
}

#[cfg(unix)]
pub fn clear() {
    Command::new("/bin/sh")
        .args(["-c", "clear"])
        .status()
        .unwrap();
}

pub fn empty_tasks() {
    queue!("You have no tasks!\n");
    queue!("Try adding one with: ");
    queue!("{}{}{}", CYAN, ITALIC, "t 'this is a task'\n");
}

pub fn header(total_checked: usize, total_tasks: usize, board: &str) {
    let board = if board.is_empty() { "Tasks" } else { board };
    queue!(" {}{}{}:", UNDERLINED, board, RESET);
    queue!("{} [{}/{}]\n", GREY, total_checked, total_tasks);
}

pub fn note(id: usize, text: &str, total_notes: usize) {
    let spacing = spacing(id, total_notes);
    queue!("{}   {}{}", GREY, id, spacing);
    queue!("{} •  ", MAGENTA);
    queue!("{}\n", text);
}

pub fn task(id: usize, checked: bool, text: &str, days: u64, total_tasks: usize) {
    let spacing = spacing(id, total_tasks);
    let days = if days > 0 && !checked {
        format!(" {}d", days)
    } else {
        String::new()
    };

    let checked = if checked {
        format!("{} √ {}", GREEN, RESET)
    } else {
        format!("{}[ ]{}", MAGENTA, RESET)
    };

    queue!("{}   {}{}", GREY, id, spacing);
    queue!("{} {}{}{}\n", checked, text, GREY, days);
}

pub fn spacing(id: usize, total: usize) -> &'static str {
    if total < 10 {
        ". "
    } else if total < 100 {
        if id < 10 {
            ".  "
        } else if id < 100 {
            ". "
        } else {
            unreachable!();
        }
    } else if id < 10 {
        ".   "
    } else if id < 100 {
        ".  "
    } else {
        ". "
    }
}

pub fn footer(completed_tasks: usize, total_tasks: usize, total_notes: usize) {
    let percent: usize = (completed_tasks as f32 / total_tasks as f32 * 100.0) as usize;
    let note = if total_notes == 1 { "note" } else { "notes" };

    queue!("\n");
    queue!("{}  {}% of all tasks completed\n  ", GREY, percent);
    queue!("{}{}", GREEN, completed_tasks);
    queue!("{} done · ", GREY);
    queue!("{}{}", MAGENTA, total_tasks - completed_tasks);
    queue!("{} pending · ", GREY);
    queue!("{}{}", BLUE, total_notes);
    queue!("{} {}\n", GREY, note);
}

pub fn missing_args(args: &str) {
    queue!("Missing arguments for command: {}'{}'\n", CYAN, args);
}
