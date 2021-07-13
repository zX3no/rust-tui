use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    Result,
};
use std::io::stdout;

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
        Print("]\n"),
        ResetColor
    )?;

    Ok(())
}

#[allow(dead_code)]
pub fn task(id: usize, checked: bool, text: &str) -> Result<()> {
    //I could keep tasks as far left as possible until there are
    //bigger more than 10 and more than 100
    //TODO parse total_tasks as function parameter
    execute!(
        stdout(),
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
    execute!(stdout(), Print("\n"),)?;
    Ok(())
}

#[allow(dead_code)]
pub fn footer(completed_tasks: usize, total_tasks: usize) -> Result<()> {
    let percent: usize = (completed_tasks as f32 / total_tasks as f32 * 100.0) as usize;
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print("\n"),
        Print("  "),
        Print(percent),
        Print("% of all tasks completed\n"),
        ResetColor,
    )?;
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
        Print(" pending"),
        ResetColor,
    )?;
    Ok(())
}
