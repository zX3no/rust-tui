#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use toml::from_str;

#[path = "./print_task.rs"]
mod print_task;

#[derive(Deserialize, Debug)]
struct Task {
    item: String,
    id: i32,
    date: String
}

pub fn create_config()
{
    let t: &str = r#"
        [[task]]
        item = 'this is a sample task'
        id = 1
        date = '7/07/2021'
        [[task]]
        item = 'among us'
        id = 2
        date = '8/07/2021'
    "#;

    let task_table: HashMap<String, Vec<Task>> = from_str(t).unwrap();
    let items: &[Task] = &task_table["task"];

    for x in 0..items.len(){
        print_task::task(items[x].id, false, &items[x].item).ok();
    }
    
    //println!("{:?}", task_table);
    //println!("{:?}", items);
}