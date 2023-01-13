use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

use crate::file::with_rewind;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Self {
        Self {
            text,
            created_at: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{created_at}]", self.text)
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = with_rewind(collect_tasks, &file)?;
    tasks.push(task);

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    if task_position == 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = with_rewind(collect_tasks, &file)?;

    if task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    file.set_len(0)?; // truncate

    tasks.remove(task_position - 1);

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;

    let tasks = with_rewind(collect_tasks, &file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {task}", i + 1);
        }
    }

    Ok(())
}

fn collect_tasks(file: &File) -> Result<Vec<Task>> {
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    Ok(tasks)
}
