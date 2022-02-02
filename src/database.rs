use std::path::{Path, PathBuf};

use rusqlite::{params, Connection, Result, Statement};

lazy_static! {
    static ref DB_DIR: PathBuf = {
        let config_dir = dirs::config_dir().unwrap().join("t");

        if !Path::new(config_dir.as_path()).exists() {
            std::fs::create_dir(config_dir.as_path()).unwrap();
        }
        config_dir.join("t.db")
    };
}

pub struct Board {
    pub name: String,
    pub tasks: Vec<Task>,
    pub total: usize,
    pub checked: usize,
}

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

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open(DB_DIR.as_path()).unwrap();

        conn.pragma_update(None, "journal_mode", "WAL").unwrap();
        conn.pragma_update(None, "synchronous", "0").unwrap();
        conn.pragma_update(None, "temp_store", "0").unwrap();

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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS old(
                    content TEXT NOT NULL
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
                    "INSERT INTO old SELECT content FROM tasks WHERE rowid = {}; DELETE FROM tasks WHERE rowid = {}",
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
                    "UPDATE tasks SET checked = ((checked| 1) - (checked & 1)) WHERE rowid = ?",
                    [id],
                )
                .unwrap();
        }
    }
    pub fn clear_tasks(&self) -> Result<()> {
        self.conn
            .execute_batch("INSERT INTO old SELECT content FROM tasks WHERE checked = '1'; DELETE FROM tasks WHERE checked = '1'")?;
        Ok(())
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
            .flat_map(|id| real_ids.get(*id - 1))
            .cloned()
            .collect()
    }
    pub fn get_old(&self) -> Vec<String> {
        let mut stmt = self.stmt("SELECT * FROM old");
        stmt.query_map([], |row| {
            let content: String = row.get(0).unwrap();
            Ok(content)
        })
        .unwrap()
        .flatten()
        .collect()
    }
    fn get_board(&self, board: String) -> Board {
        let mut stmt = self.stmt(&format!(
            "SELECT *, rowid FROM tasks WHERE BOARD = '{}'",
            board
        ));

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
            name: board,
            total: tasks.len(),
            tasks,
            checked: total_checked,
        }
    }
    pub fn get_boards(&self) -> Vec<Board> {
        let mut stmt = self.stmt("SELECT DISTINCT board FROM tasks");
        let rows = stmt.query_map([], |row| row.get(0)).unwrap();
        let boards: Vec<String> = rows.into_iter().flatten().collect();

        let mut a = Vec::new();
        let mut b = Vec::new();

        for board in boards {
            if board == "Tasks" {
                a.push(self.get_board(board));
            } else {
                b.push(self.get_board(board));
            }
        }
        a.append(&mut b);
        a
    }
    pub fn total_checked(&self) -> usize {
        self.total("SELECT COUNT(*) FROM tasks WHERE checked = '1'")
    }
    pub fn total_tasks(&self) -> usize {
        self.total("SELECT COUNT(*) FROM tasks")
    }
    pub fn total_notes(&self) -> usize {
        self.total("SELECT COUNT(*) FROM tasks WHERE note = '1'")
    }
    pub fn total(&self, query: &str) -> usize {
        let mut stmt = self.stmt(query);
        let mut rows = stmt.query([]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            row.get(0).unwrap()
        } else {
            0
        }
    }
    fn stmt(&self, query: &str) -> Statement {
        self.conn.prepare(query).unwrap()
    }
}
