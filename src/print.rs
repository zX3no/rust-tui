#![allow(unused_must_use)]
use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub fn clear() {
    execute!(
        stdout(),
        Hide,
        DisableBlinking,
        MoveTo(0, 0),
        Clear(ClearType::All)
    )
    .unwrap();
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
    )
    .unwrap()
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

pub fn note(id: usize, text: &str, total_notes: usize) {
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print(id)
    );

    let mut spacing: &str = "";
    if total_notes < 10 {
        spacing = ". ";
    } else if total_notes < 100 {
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
    );
    execute!(stdout(), Print("\n"));
}

pub fn task(id: usize, checked: bool, text: &str, days: i64, total_tasks: usize) {
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print(id)
    );
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
        );
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
    }
    execute!(stdout(), Print("\n"));
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
    );
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
    );
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
    d                       Deletes a task
    h, help                 Displays the help page

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
    t d 1                   Deletes task number 1
    t d 1-3                 Deletes task 1, 2 and 3
    "
    );
}
