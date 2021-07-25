## t

t is a simple task manager written in rust.

It was designed to be a faster version of [taskbook.](https://github.com/klaussinani/taskbook)

### Install :

```
git clone https://github.com/zX3no/t
cd t
cargo install --path .
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
        a                       Create a task
        cls                     Clear checked tasks and archive them
        o                       Displays previously completed tasks
        d                       Deletes a task
        h                       Displays the help page

    Examples                     
        t                       Displays tasks
        t example task          Creates a task 'example task'
        t !TODO example task    Create a task in a board called 'TODO'        
        t a example task        Creates a task 'example task'
        t a !TODO example task  Create a task in a board called 'TODO'        
        t 1 2 3                 Checks task 1, 2 and 3
        t c 1                   Checks task 1
        t cls                   Clears all checked task
        t o                     Displays previously completed tasks
        t d 1                   Deletes task number 1
        t h                     Displays the help page
```

Config Directory:

Windows: `%appdata%/t`

### TODO

Show some examples of what tasks, boards and notes look like

Move to better argument handler

Add support for notes