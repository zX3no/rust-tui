use std::{
    path::{Path, PathBuf},
    time::Duration,
};

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

        conn.busy_timeout(Duration::from_millis(0)).unwrap();
        conn.pragma_update(None, "journal_mode", "WAL").unwrap();
        conn.pragma_update(None, "synchronous", "0").unwrap();
        conn.pragma_update(None, "temp_store", "MEMORY").unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks(
                    content TEXT NOT NULL,
                    checked BOOL NOT NULL,
                    note BOOL NOT NULL,
                    board TEXT NOT NULL,
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
    pub fn insert_task(&self, task: &str, board: Option<String>) {
        let board = if let Some(board) = board {
            board
        } else {
            String::from("Tasks")
        };

        self.conn
            .execute(
                "INSERT INTO tasks (content, checked, note, board, date) VALUES (?1, ?2, ?3, ?4, datetime('now', 'localtime'))",
                params![task, false, false, board],
            )
            .unwrap();
    }
    pub fn insert_note(&self, note: &str, board: Option<String>) {
        let board = if let Some(board) = board {
            board
        } else {
            String::from("Tasks")
        };

        self.conn
            .execute(
                "INSERT INTO tasks (content, checked, note, board, date) VALUES (?1, ?2, ?3, ?4, datetime('now', 'localtime'))",
                params![note, false, true, board],
            )
            .unwrap();
    }
    pub fn delete_tasks(&self, ids: &[usize]) {
        let ids = self.get_real_ids(ids);
        for id in ids {
            self.conn
                .execute("DELETE FROM tasks WHERE rowid = ?", [id])
                .unwrap();
        }
    }
    pub fn check_tasks(&self, ids: &[usize]) {
        let ids = self.get_real_ids(ids);
        for id in ids {
            self.conn
                .execute(
                    "UPDATE tasks SET checked = ((checked| 1) - (checked & 1)) WHERE rowid = ?",
                    [id],
                )
                .unwrap();
        }
    }
    pub fn clear_tasks(&self) -> Result<()> {
        self.conn
            .execute("DELETE FROM tasks WHERE checked = '1'", [])?;
        Ok(())
    }
    pub fn total_checked(&self) -> usize {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM tasks WHERE checked = '1'")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            return row.get(0).unwrap();
        } else {
            0
        }
    }
    pub fn total_tasks(&self) -> usize {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM tasks").unwrap();
        let mut rows = stmt.query([]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            return row.get(0).unwrap();
        } else {
            0
        }
    }
    pub fn get_real_ids(&self, ids: &[usize]) -> Vec<usize> {
        let mut stmt = self.conn.prepare("SELECT rowid FROM tasks").unwrap();
        let real_ids: Vec<usize> = stmt
            .query_map([], |row| Ok(row.get(0).unwrap()))
            .unwrap()
            .flatten()
            .collect();
        ids.iter()
            .map(|id| real_ids.get(*id - 1).unwrap().clone())
            .collect()
    }
    pub fn get_boards(&self) -> Vec<Board> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT board FROM tasks")
            .unwrap();
        let boards: Vec<String> = stmt
            .query_map([], |row| Ok(row.get(0).unwrap()))
            .unwrap()
            .flatten()
            .collect();

        boards
            .iter()
            .map(|board| {
                let mut stmt = self
                    .conn
                    .prepare(&format!(
                        "SELECT *, rowid FROM tasks WHERE board = '{}'",
                        board
                    ))
                    .unwrap();

                let mut total_checked: usize = 0;

                let tasks: Vec<Task> = stmt
                    .query_map([], |row| {
                        let checked = row.get(1).unwrap();
                        if checked {
                            total_checked += 1;
                        }
                        Ok(Task {
                            content: row.get(0).unwrap(),
                            checked,
                            note: row.get(2).unwrap(),
                            board: row.get(3).unwrap(),
                            date: row.get(4).unwrap(),
                            id: row.get(5).unwrap(),
                        })
                    })
                    .unwrap()
                    .flatten()
                    .collect();

                Board {
                    name: board.clone(),
                    total: tasks.len(),
                    tasks,
                    checked: total_checked,
                }
            })
            .collect()
    }
}

pub struct Task {
    pub content: String,
    pub checked: bool,
    pub note: bool,
    pub board: String,
    pub date: String,
    pub id: usize,
}

pub struct Board {
    pub name: String,
    pub tasks: Vec<Task>,
    pub total: usize,
    pub checked: usize,
}
