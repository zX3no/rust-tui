#![allow(unused_must_use)]
use crossterm::{
    queue,
    style::{Print, Stylize},
};
use std::{io::stdout, process::Command};

#[allow(dead_code)]
pub fn clear() {
    if cfg!(windows) {
        Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .expect("failed to queue process");
    } else if cfg!(unix) {
        Command::new("/bin/sh")
            .args(["-c", "clear"])
            .status()
            .expect("failed to queue process");
    }
}
pub fn help_message() {
    queue!(
        stdout(),
        Print("You have no tasks!\n Try adding one with: "),
        Print(" t 'this⠀is⠀a⠀task'\n".cyan().italic()),
    );
}
//TODO: headers do not align with each other
pub fn header(completed_tasks: usize, total_tasks: usize, board: &str) {
    queue!(
        stdout(),
        Print(" "),
        Print(format!("{}:", board.underlined())),
        Print(format!(" [{}/{}]\n", completed_tasks, total_tasks).dark_grey())
    );
}
pub fn old_header() {
    queue!(stdout(), Print(" "), Print("Tasks:\n".underlined()));
}
pub fn note(id: usize, text: &str, total_notes: usize) {
    let spacing = if total_notes < 10 {
        ". "
    } else if total_notes < 100 {
        if id < 10 {
            ".  "
        } else if id < 100 {
            ". "
        } else {
            panic!("id > 100");
        }
    } else if id < 10 {
        ".   "
    } else if id < 100 {
        ".  "
    } else {
        ". "
    };

    queue!(
        stdout(),
        Print(format!("   {}{}", id, spacing).dark_grey()),
        Print(" •  ".dark_magenta()),
        Print(format!("{}\n", text)),
    );
}
pub fn task(id: usize, checked: bool, text: &str, days: i64, total_tasks: usize) {
    queue!(stdout(), Print(format!("   {}", id).dark_grey()));
    if checked {
        let spacing = if total_tasks < 10 {
            ".  "
        } else if total_tasks < 100 {
            if id < 10 {
                ".   "
            } else if id < 100 {
                ".  "
            } else {
                panic!("id > 100");
            }
        } else if id < 10 {
            ".    "
        } else if id < 100 {
            ".   "
        } else {
            ".  "
        };
        queue!(
            stdout(),
            Print(format!(
                "{}{}{}",
                spacing.dark_grey(),
                "√  ".green(),
                text.dark_grey()
            ))
        );
    } else {
        let spacing = if total_tasks < 10 {
            ". "
        } else if total_tasks < 100 {
            if id < 10 {
                ".  "
            } else if id < 100 {
                ". "
            } else {
                panic!("id > 100");
            }
        } else if id < 10 {
            ".   "
        } else if id < 100 {
            ".  "
        } else {
            ". "
        };
        queue!(
            stdout(),
            Print(format!(
                "{}{} {}",
                spacing.dark_grey(),
                "[ ]".dark_magenta(),
                text
            ))
        );
        if days > 0 {
            queue!(stdout(), Print(format!(" {}d", days).dark_grey()));
        }
    };
    queue!(stdout(), Print("\n"));
}
pub fn footer(completed_tasks: usize, total_tasks: usize, total_notes: usize) {
    let percent: usize = (completed_tasks as f32 / total_tasks as f32 * 100.0) as usize;
    queue!(
        stdout(),
        Print(format!("  {}% of all tasks complted\n  ", percent).dark_grey()),
        Print(completed_tasks.to_string().green()),
        Print(" done · ".dark_grey()),
        Print(format!("{}", total_tasks - completed_tasks).magenta()),
        Print(" pending · ".dark_grey()),
        Print(total_notes.to_string().blue()),
        Print(" notes\n".dark_grey()),
    );
}
pub fn new_line() {
    queue!(stdout(), Print("\n"));
}
pub fn help() {
    println!(
        "
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
    h, help                 Displays the help page
    v, version              Displays version

Examples                     
    t                       Displays tasks
    t example task          Creates a task 'example task'
    t !TODO example task    Create a task in a board called 'TODO'        
    t 1 2 3                 Checks task 1, 2 and 3
    t 1-3                   Checks task 1, 2 and 3
    t n example note        Create note 'example note'
    t n !TODO example task  Create a note in a board called 'TODO'        
    t d 1                   Deletes task 1
    t d 1-3                 Deletes task 1, 2 and 3
    "
    );
}

pub fn missing_args(args: &str) {
    queue!(
        stdout(),
        Print("Missing arguments for command: "),
        Print(format!("'{}'\n", args).cyan()),
    );
}
