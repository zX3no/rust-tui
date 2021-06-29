#[allow(unused_imports)]
use std::io::{stdout, Write};

#[allow(unused_imports)]
use crossterm::{
    execute, event, terminal,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, SetAttribute},
    ExecutableCommand, Result,
    style::Attribute,
};

fn main() -> Result<()> {
    execute!(
        stdout(),
        Print(" "),
        SetAttribute(Attribute::Underlined),
        Print("Tasks:"), 
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(" [0/10]"),
        ResetColor
    )?;
    execute!(
        stdout(),
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print("1. "),
        Print(" "),
        SetForegroundColor(Color::Green),
        Print("√"),
        Print(" "),
        ResetColor,
        Print(" "),
        Print("Do this?"),
    )?;
    execute!(
        stdout(),
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print("1. "),
        SetForegroundColor(Color::DarkMagenta),
        Print("[ ]"),
        ResetColor,
        Print(" "),
        Print("Do this?"),
    )?;
    Ok(())
}