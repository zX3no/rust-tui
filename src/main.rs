use crossterm::Result;

#[allow(non_snake_case)]
mod PrintTask;
use PrintTask::{header, task};

fn main() -> Result<()> {
    header();
    //idk what to do about all these 'oks'.
    task(10, true, "AAAAAAA").ok();
    task(2, false, "Test").ok();
    task(15, false, "Make this very hard project").ok();
    task(999, false, "remember to do something").ok();
    task(9, true, "Takssks").ok();
    Ok(())
}
