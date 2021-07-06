#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use toml::from_str;

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
    /*
    for i in items.iter() {
        println!("{}", i);
    }
    */

    println!("{}", items[1].item);

    //println!("{:?}", task_table);
    //println!("{:?}", items);
}