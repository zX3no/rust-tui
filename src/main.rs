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

fn header() {
    execute!(
        stdout(),
        Print(" "),
        SetAttribute(Attribute::Underlined),
        Print("Tasks:"),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(" [0/10]"),
        ResetColor
    )
    .ok();
}

fn task(id: i32, checked: bool, text: &str) {
    execute!(
        stdout(),
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
        Print("   "),
        Print(id)
    )
    .ok();

    if checked {
        if id < 10 {
            execute!(stdout(), Print(".    ")).ok();
        } else if id < 100 {
            execute!(stdout(), Print(".   ")).ok();
        } else {
            execute!(stdout(), Print(".  ")).ok();
        }
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("√  "),
            SetForegroundColor(Color::DarkGrey),
            Print(text),
        )
        .ok();
    } else {
        if id < 10 {
            execute!(stdout(), Print(".   ")).ok();
        } else if id < 100 {
            execute!(stdout(), Print(".  ")).ok();
        } else {
            execute!(stdout(), Print(". ")).ok();
        }
        execute!(
            stdout(),
            SetForegroundColor(Color::DarkMagenta),
            Print("[ ]"),
            ResetColor,
            Print(" "),
            Print(text)
        )
        .ok();
    }
}
fn main() -> Result<()> {
    header();
    task(10, true, "AAAAAAA");
    task(2, false, "Test");
    task(15, false, "Make this very hard project");
    task(999, false, "remember to do something");
    task(9, true, "Takssks");
    Ok(())
}
