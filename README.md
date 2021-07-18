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
t 

    Usage
        t [<options> ...]

    Options
        none                Display a list of tasks.
        none+args           Create a task
        check, c            Check/uncheck task
        add, a              Create a task
        clear               Clear checked tasks and archive them
    
    Examples
        t                   Display tasks
        t example task      Creates a task 'example task'
        t add example task  Creates a task 'example task'
        t check 1           Checks task 1
        t clear             Clears all checked task
```

Config Directory:

Windows: `%appdata%/t`

### TODO

Write help menu / move to better argument handler

Add support for notes