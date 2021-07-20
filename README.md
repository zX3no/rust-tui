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
t help 

    Usage
        t [<options> <args>] 

    Options
        none                Display a list of tasks.
        none+args           Create a task
        none+number         Check/uncheck task

        check, c            Check/uncheck task
        add, a              Create a task
        clear               Clear checked tasks and archive them
        old                 Displays previously completed tasks
        help
    
    Examples
        t                   Display tasks
        t example task      Creates a task 'example task'
        t 1 2 3             Checks task 1, 2 and 3
        t add example task  Creates a task 'example task'
        t check 1           Checks task 1
        t clear             Clears all checked task
        t old               Displays previously completed tasks
```

Config Directory:

Windows: `%appdata%/t`

### TODO

Move to better argument handler

Add support for notes