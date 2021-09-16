#![allow(dead_code)]
use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io::stdout;

pub fn help_message() {
    execute!(
        stdout(),
        Print("You have no tasks!"),
        Print(" Try adding one with:"),
        SetAttribute(Attribute::Italic),
        SetForegroundColor(Color::Cyan),
        Print(" t 'this⠀is⠀a⠀task'"),
        ResetColor,
    )
    .unwrap()
}

pub fn header(completed_tasks: usize, total_tasks: usize, board: &String) {
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
    )
    .unwrap();
}

pub fn note(id: usize, text: &String, total_tasks: usize) {
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print(id)
    )
    .unwrap();

    let mut spacing: &str = "";
    if total_tasks < 10 {
        spacing = ". ";
    } else if total_tasks < 100 {
        if id < 10 {
            spacing = ".  ";
        } else if id < 100 {
            spacing = ". ";
        }
    } else if id < 10 {
        spacing = ".   ";
    } else if id < 100 {
        spacing = ".  ";
    } else {
        spacing = ". ";
    }
    execute!(
        stdout(),
        Print(spacing),
        SetForegroundColor(Color::DarkMagenta),
        Print(" •  "),
        ResetColor,
        Print(text),
    )
    .unwrap();
    execute!(stdout(), Print("\n"),).unwrap();
}

pub fn task(id: usize, checked: bool, text: &String, days: i64, total_tasks: usize) {
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print(id)
    )
    .unwrap();
    let mut spacing: &str = "";
    if checked {
        if total_tasks < 10 {
            spacing = ".  ";
        } else if total_tasks < 100 {
            if id < 10 {
                spacing = ".   ";
            } else if id < 100 {
                spacing = ".  ";
            }
        } else if id < 10 {
            spacing = ".    ";
        } else if id < 100 {
            spacing = ".   ";
        } else {
            spacing = ".  ";
        }
        execute!(
            stdout(),
            Print(spacing),
            SetForegroundColor(Color::Green),
            Print("√  "),
            SetForegroundColor(Color::DarkGrey),
            Print(text),
            ResetColor
        )
        .unwrap();
    } else {
        if total_tasks < 10 {
            spacing = ". ";
        } else if total_tasks < 100 {
            if id < 10 {
                spacing = ".  ";
            } else if id < 100 {
                spacing = ". ";
            }
        } else if id < 10 {
            spacing = ".   ";
        } else if id < 100 {
            spacing = ".  ";
        } else {
            spacing = ". ";
        }
        execute!(
            stdout(),
            Print(spacing),
            SetForegroundColor(Color::DarkMagenta),
            Print("[ ]"),
            ResetColor,
            Print(" "),
            Print(text),
            ResetColor
        )
        .unwrap();
        if days > 0 {
            execute!(
                stdout(),
                Print(" "),
                SetForegroundColor(Color::DarkGrey),
                Print(days),
                Print("d"),
                ResetColor,
            )
            .unwrap();
        }
    }
    execute!(stdout(), Print("\n")).unwrap();
}

pub fn footer(completed_tasks: usize, total_tasks: usize, total_notes: usize) {
    let percent: usize = (completed_tasks as f32 / total_tasks as f32 * 100.0) as usize;
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("  "),
        Print(percent),
        Print("% of all tasks completed\n"),
        ResetColor,
    )
    .unwrap();
    execute!(
        stdout(),
        Print("  "),
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
        Print(" notes"),
        ResetColor,
    )
    .unwrap();
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

    c                       Check/uncheck task
    a                       Create a task
    n                       Create a note
    cls                     Clear checked tasks and archive them
    o                       Displays previously completed tasks
    d                       Deletes a task
    b                       Backup current tasks
    h                       Displays the help page

Examples                     
    t                       Displays tasks
    t example task          Creates a task 'example task'
    t !TODO example task    Create a task in a board called 'TODO'        
    t a example task        Creates a task 'example task'
    t a !TODO example task  Create a task in a board called 'TODO'        
    t n example note        Create note 'example note'
    t 1 2 3                 Checks task 1, 2 and 3
    t c 1                   Checks task 1
    t 1-3                   Checks task 1, 2 and 3
    t c 1-3                 Checks task 1, 2 and 3
    t cls                   Clears all checked task
    t o                     Displays previously completed tasks
    t d 1                   Deletes task number 1
    t d 1-3                 Deletes task 1, 2 and 3
    t b                     Creates the file backup.toml in the config directory
    t h                     Displays the help page"
    );
}
