use rusqlite::{params, Connection};
use std::{env, fs, path::PathBuf};

#[derive(Debug)]
pub struct Board {
    pub name: String,
    pub tasks: Vec<Task>,
    pub total: usize,
    pub checked: usize,
}

#[derive(Debug)]
pub struct Task {
    pub content: String,
    pub checked: bool,
    pub note: bool,
    pub board: String,
    pub date: String,
    pub id: usize,
}

pub struct Database {
    conn: Connection,
}

impl Default for Database {
    fn default() -> Self {
        let config = {
            if let Ok(home) = env::var("HOME") {
                PathBuf::from(home).join(".config")
            } else if let Ok(home) = env::var("APPDATA") {
                PathBuf::from(home)
            } else if let Ok(home) = env::var("XDG_HOME") {
                PathBuf::from(home)
            } else {
                panic!("HOME, XDG_HOME and APPDATA enviroment variables are all empty?");
            }
        }
        .join("t");
        fs::create_dir_all(&config).unwrap();
        let db = config.join("t.db");

        let conn = Connection::open(&db).unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS tasks(
                content TEXT NOT NULL,
                checked BOOL NOT NULL,
                note BOOL NOT NULL,
                board TEXT NOT NULL,
                date TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS old(
                content TEXT NOT NULL
            );
        ",
        )
        .unwrap();

        Self { conn }
    }
}

impl Database {
    pub fn insert_task(&self, task: &str, board: Option<String>) {
        let board = if let Some(board) = board {
            board
        } else {
            String::from("Tasks")
        };

        self.conn
            .execute(
                "INSERT INTO tasks (content, checked, note, board, date) VALUES (?1, ?2, ?3, ?4, datetime('now', 'utc'))",
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
                "INSERT INTO tasks (content, checked, note, board, date) VALUES (?1, ?2, ?3, ?4, datetime('now', 'utc'))",
                params![note, false, true, board],
            )
            .unwrap();
    }
    pub fn delete_tasks(&self, ids: &[usize]) {
        let ids = self.get_real_ids(ids);
        for id in ids {
            self.conn
                .execute_batch(&format!(
                    "INSERT INTO old SELECT content FROM tasks WHERE rowid = {} AND checked = 1; DELETE FROM tasks WHERE rowid = {}",
                    id, id
                ))
                .unwrap();
        }
    }
    pub fn check_tasks(&self, ids: &[usize]) {
        let ids = self.get_real_ids(ids);
        for id in ids {
            self.conn
                .execute(
                    "UPDATE tasks SET checked = ((checked| 1) - (checked & 1)) WHERE rowid = ? AND note = '0'",
                    [id],
                )
                .unwrap();
        }
    }
    pub fn clear_tasks(&self) {
        self.conn
            .execute_batch("INSERT INTO old SELECT content FROM tasks WHERE checked = '1'; DELETE FROM tasks WHERE checked = '1'").unwrap();
    }
    pub fn get_real_ids(&self, ids: &[usize]) -> Vec<usize> {
        let boards = self.get_boards();
        let mut real_ids = Vec::new();
        for board in boards {
            for task in board.tasks {
                real_ids.push(task.id);
            }
        }
        ids.iter()
            .flat_map(|id| real_ids.get(id.saturating_sub(1)))
            .cloned()
            .collect()
    }
    pub fn get_old(&self) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT * FROM old").unwrap();
        stmt.query_map([], |row| row.get(0))
            .unwrap()
            .flatten()
            .collect()
    }
    pub fn get_boards(&self) -> Vec<Board> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT board FROM tasks")
            .unwrap();

        let mut boards: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .flatten()
            .collect();

        boards.sort_by_key(|board| board != "Tasks");

        boards
            .into_iter()
            .map(|board| {
                let mut stmt = self
                    .conn
                    .prepare("SELECT *, rowid FROM tasks WHERE BOARD = ?")
                    .unwrap();

                let mut total_checked = 0;
                let tasks: Vec<Task> = stmt
                    .query_map([&board], |row| {
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
                    name: board,
                    total: tasks.len(),
                    tasks,
                    checked: total_checked,
                }
            })
            .collect()
    }
    pub fn total_checked(&self) -> usize {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM tasks WHERE checked = '1' AND note = '0'")
            .unwrap();
        stmt.query_row([], |row| row.get(0)).unwrap()
    }
    pub fn total_tasks(&self) -> usize {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM tasks WHERE note = '0'")
            .unwrap();
        stmt.query_row([], |row| row.get(0)).unwrap()
    }
    pub fn total_notes(&self) -> usize {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM tasks WHERE note = '1'")
            .unwrap();
        stmt.query_row([], |row| row.get(0)).unwrap()
    }
    pub fn total(&self) -> usize {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM tasks").unwrap();
        stmt.query_row([], |row| row.get(0)).unwrap()
    }
}
