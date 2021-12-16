use std::path::{Path, PathBuf};

use rusqlite::{params, Connection, Result};

lazy_static! {
    static ref CONFIG_DIR: PathBuf = {
        let config_dir = dirs::config_dir().unwrap().join("t");

        if !Path::new(config_dir.as_path()).exists() {
            std::fs::create_dir(config_dir.as_path()).unwrap();
        }
        config_dir
    };
    static ref DB_DIR: PathBuf = dirs::config_dir().unwrap().join("t\\t.db");
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open(DB_DIR.as_path()).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks(
                    content TEXT NOT NULL,
                    checked BOOL NOT NULL,
                    board TEXT NOT NULL,
                    date TEXT NOT NULL
                )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS old_tasks(
                    content TEXT NOT NULL,
                    date TEXT NOT NULL
                )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes(
                    content TEXT NOT NULL,
                    date TEXT NOT NULL
                )",
            [],
        )
        .unwrap();
        Self { conn }
    }
    pub fn insert_task(&self, task: &str, board: Option<&str>) {
        let board = if let Some(board) = board {
            board
        } else {
            "Tasks"
        };

        self.conn
            .execute(
                "INSERT INTO tasks (content, checked, board, date) VALUES (?1, ?2, ?3, datetime('now', 'localtime'))",
                params![task, false, board],
            )
            .unwrap();
    }
    pub fn delete_task(&self, id: usize) {
        self.conn
            .execute("DELETE FROM tasks WHERE rowid = ?", [id])
            .unwrap();
    }

    pub fn check_task(&self, id: usize) {
        self.conn
            .execute("UPDATE tasks SET checked = '1' WHERE rowid = ?", [id])
            .unwrap();
    }
    pub fn clear_tasks(&self) -> Result<()> {
        let tasks = self.get_checked()?;
        for task in tasks {
            self.conn.execute(
                "INSERT INTO old_tasks (content, date) VALUES (?1, ?2)",
                params![task.content, task.date],
            )?;

            self.conn
                .execute("DELETE FROM tasks WHERE rowid = ?", params![task.id])?;
        }
        Ok(())
    }
    pub fn get_checked(&self) -> Result<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT *, rowid FROM tasks WHERE checked = 1")
            .unwrap();

        Ok(stmt
            .query_map([], |row| {
                Ok(Task {
                    content: row.get(0)?,
                    checked: row.get(1)?,
                    board: row.get(2)?,
                    date: row.get(3)?,
                    id: row.get(4)?,
                })
            })
            .unwrap()
            .flatten()
            .collect())
    }
    pub fn get_tasks(&self) -> Vec<Task> {
        let mut stmt = self.conn.prepare("SELECT *, rowid FROM tasks").unwrap();

        stmt.query_map([], |row| {
            Ok(Task {
                content: row.get(0).unwrap(),
                checked: row.get(1).unwrap(),
                board: row.get(2).unwrap(),
                date: row.get(3).unwrap(),
                id: row.get(4).unwrap(),
            })
        })
        .unwrap()
        .flatten()
        .collect()
    }
    pub fn get_old_tasks(&self) -> Vec<(String, String)> {
        let mut stmt = self.conn.prepare("SELECT * FROM old_tasks").unwrap();

        stmt.query_map([], |row| {
            let content = row.get(0).unwrap();
            let date = row.get(1).unwrap();
            Ok((content, date))
        })
        .unwrap()
        .flatten()
        .collect()
    }
}

#[derive(Debug)]
pub struct Task {
    content: String,
    checked: bool,
    board: String,
    date: String,
    id: usize,
}
