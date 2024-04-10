use std::{
    fmt::{Display, Formatter, Result},
    fs::{self, File},
    io::Write,
};

use chrono::{DateTime, Local};

use crate::app::AppResult;

#[derive(Debug, Default)]
pub struct Logger {
    logs: Vec<Log>,
}

impl Logger {
    pub fn logs(&self) -> &Vec<Log> {
        &self.logs
    }
    pub fn logs_mut(&mut self) -> &mut Vec<Log> {
        &mut self.logs
    }
}

#[derive(Debug)]
pub struct Log {
    level: Level,
    message: String,
    timestamp: DateTime<Local>,
}


impl Display for Log {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "{:?} - {:?}: {}\n", self.timestamp, self.level, self.message
        )
    }
}

#[derive(Debug)]
pub enum Level {
    Info,
    Warning,
    Error,
}

impl Logger {
    pub fn new() -> Self {
        Logger { logs: Vec::new() }
    }

    pub fn log(&mut self, level: Level, message: String) {
        let now: DateTime<Local> = Local::now();
        let log = Log {
            level,
            message,
            timestamp: now,
        };
        self.logs.push(log);
        if self.logs.len() > 1000 {
            self.logs.remove(0); // Remove the oldest log
        }
    }

    pub fn write_to_file(&mut self) -> AppResult<()> {
        fs::create_dir_all(".local/share")?;
        let mut file = File::create(".local/share/dans-editor.log")?;

        for log in self.logs.iter().rev() {
            let log_line = format!("{:?} - {:?}: {}\n", log.timestamp, log.level, log.message);
            file.write_all(log_line.as_bytes()).unwrap();
        }
        // TODO: Limit the number of log entries to 1000

        Ok(())
    }
}
