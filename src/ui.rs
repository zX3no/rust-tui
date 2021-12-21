#![allow(unused_must_use)]
use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

#[allow(dead_code)]
pub fn clear() {
    execute!(
        stdout(),
        Hide,
        DisableBlinking,
        MoveTo(0, 0),
        Clear(ClearType::All)
    );
}
pub fn help_message() {
    execute!(
        stdout(),
        Print("You have no tasks!"),
        Print(" Try adding one with:"),
        SetAttribute(Attribute::Italic),
        SetForegroundColor(Color::Cyan),
        Print(" t 'this⠀is⠀a⠀task'\n"),
        ResetColor,
    );
}
pub fn header(completed_tasks: usize, total_tasks: usize, board: &str) {
    execute!(
        stdout(),
        Print(" "),
        SetAttribute(Attribute::Underlined),
        Print(board),
        Print(":"),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(" ["),
        Print(completed_tasks),
        Print("/"),
        Print(total_tasks),
        Print("]\n"),
        ResetColor
    );
}
pub fn old_header() {
    execute!(
        stdout(),
        Print(" "),
        SetAttribute(Attribute::Underlined),
        Print("Tasks:\n"),
        ResetColor
    );
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

    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print(id),
        Print(spacing),
        SetForegroundColor(Color::DarkMagenta),
        Print(" •  "),
        ResetColor,
        Print(text),
        Print("\n")
    );
}
pub fn task(id: usize, checked: bool, text: &str, days: i64, total_tasks: usize) {
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print(id)
    );
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
        execute!(
            stdout(),
            Print(spacing),
            SetForegroundColor(Color::Green),
            Print("√  "),
            SetForegroundColor(Color::DarkGrey),
            Print(text),
            ResetColor
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
        execute!(
            stdout(),
            Print(spacing),
            SetForegroundColor(Color::DarkMagenta),
            Print("[ ]"),
            ResetColor,
            Print(" "),
            Print(text),
            ResetColor
        );
        if days > 0 {
            execute!(
                stdout(),
                Print(" "),
                SetForegroundColor(Color::DarkGrey),
                Print(days),
                Print("d"),
                ResetColor,
            );
        }
    };
    execute!(stdout(), Print("\n"));
}
pub fn footer(completed_tasks: usize, total_tasks: usize, total_notes: usize) {
    let percent: usize = (completed_tasks as f32 / total_tasks as f32 * 100.0) as usize;
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("  "),
        Print(percent),
        Print("% of all tasks completed\n  "),
        SetForegroundColor(Color::Green),
        Print(completed_tasks),
        SetForegroundColor(Color::DarkGrey),
        Print(" done · "),
        SetForegroundColor(Color::Magenta),
        Print(total_tasks - completed_tasks),
        SetForegroundColor(Color::DarkGrey),
        Print(" pending · "),
        SetForegroundColor(Color::Blue),
        Print(total_notes),
        SetForegroundColor(Color::DarkGrey),
        Print(" notes\n"),
        ResetColor,
    );
}
pub fn new_line() {
    execute!(stdout(), Print("\n"));
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
    execute!(
        stdout(),
        Print("Missing arguments for command: "),
        SetForegroundColor(Color::Cyan),
        Print(format!("'{}'\n", args)),
        ResetColor
    );
}
