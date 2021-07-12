use std::io::stdout;
use crossterm::{
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    execute, Result,
};
#[allow(dead_code)]
pub fn header(completed_tasks: usize, total_tasks: usize) -> Result<()> {
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
    )?;

    Ok(())
}

#[allow(dead_code)]
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
