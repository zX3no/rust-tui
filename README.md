# t

[<img src="https://img.shields.io/crates/v/t-cli.svg?style=flat-square" alt="crates.io link">](https://crates.io/crates/t-cli)

t is a simple task manager written in rust.

It was designed to be a faster version of [taskbook.](https://github.com/klaussinani/taskbook)

### Install :

```
cargo install t-cli
```

### Usage:

```
t h
    Usage
        t [<options> <args>] 

    Options
        none                    Display a list of tasks.
        none+args               Create a task
        none+number             Check/uncheck task

        c                       Check/uncheck task
        a                       Add a task
        n                       Add a note
        cls                     Delete checked tasks
        d                       Delete a task
        h, help                 Displays the help page

    Examples                     
        t                       Displays tasks
        t example task          Creates a task 'example task'
        t a example task        Creates a task 'example task'
        t !TODO example task    Create a task in a board called 'TODO'        
        t a !TODO example task  Create a task in a board called 'TODO'        
        t n example note        Create note 'example note'
        t n !TODO example task  Create a note in a board called 'TODO'        
        t 1 2 3                 Checks task 1, 2 and 3
        t 1-3                   Checks task 1, 2 and 3
        t c 1-3                 Checks task 1, 2 and 3
        t d 1                   Deletes task 1
        t d 1-3                 Deletes task 1, 2 and 3
```

### Config Directory:

Windows: `%appdata%/t`

Linux: `~/.config/t`
