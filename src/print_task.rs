#[allow(unused_imports)]
use std::io::{stdout, Write};

#[allow(unused_imports)]
use std::string;

#[allow(unused_imports)]
use crossterm::{
    event, execute,
    style::Attribute,
    style::{Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand, Result,
};

pub fn header(completed_tasks: i32, total_tasks: i32) -> Result<()> {
    execute!(
        stdout(),
        Print(" "),
        SetAttribute(Attribute::Underlined),
        Print("Tasks:"),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(" ["),
        Print(completed_tasks),
        Print("/"),
        Print(total_tasks),
        Print("]"),
        ResetColor
    )
}

pub fn task(id: i32, checked: bool, text: &str) -> Result<()> {
    execute!(
        stdout(),
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print(id)
    )?;
    if checked {
        if id < 10 {
            execute!(stdout(), Print(".    "))?;
        } else if id < 100 {
            execute!(stdout(), Print(".   "))?;
        } else {
            execute!(stdout(), Print(".  "))?;
        }
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("√  "),
            SetForegroundColor(Color::DarkGrey),
            Print(text),
            ResetColor
        )?;
    } else {
        if id < 10 {
            execute!(stdout(), Print(".   "))?;
        } else if id < 100 {
            execute!(stdout(), Print(".  "))?;
        } else {
            execute!(stdout(), Print(". "))?;
        }
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkMagenta),
            Print("[ ]"),
            ResetColor,
            Print(" "),
            Print(text),
            ResetColor
        )?;
    }
    Ok(())
}
