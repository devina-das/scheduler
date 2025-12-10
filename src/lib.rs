use std::cmp::Ordering;
pub use enum_iterator::{all, Sequence};
use chrono::{Local, Datelike, NaiveDate, Weekday};

use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};

// -----------------------------------------------Errors-----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum FileError {
    FileReadError, FileWriteError
}
// -----------------------------------------------Date-----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Default for Date {
    fn default() -> Self {
         Date::from(Local::now().date_naive())
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl From<NaiveDate> for Date {
    fn from(value: NaiveDate) -> Self {
        Self {
            year: value.year(),
            month: value.month(),
            day: value.day(),
        }
    }
}

impl From<Date> for Weekday {
    fn from(value: Date) -> Self {
         NaiveDate::from_ymd_opt(value.year, value.month, value.day).unwrap().weekday()
    }
}
// -----------------------------------------------Time-----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Time {
    pub hour: usize,
    pub min: usize,
    pub post: bool
}

impl ToString for Time {
    fn to_string(&self) -> String {
        let meridiem = if self.post {"PM"} else {"AM"};
        format!("{:02}:{:02} {}", self.hour, self.min, meridiem)
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {hour: 9, min: 0, post: false}
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.min == other.min
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.post && !other.post {Some(Ordering::Greater)}
        else if !self.post && other.post  {Some(Ordering::Less)}
        else {(self.hour, self.min).partial_cmp(&(other.hour, other.min))}
    }
}
// -----------------------------------------------Task-----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub date: Date,
    pub title: String,
    pub time: Time,
    pub desc: String,
}

// -----------------------------------------------Schedule-----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub next_id: usize,
    pub tasks: Vec<Task>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            next_id: 0,
            tasks: Vec::new(),
        }
    }
}

impl Schedule {
    pub fn add_task(&mut self, date: Date, title: String, time: Time, desc: String) {
        let new_task = Task {
            id: self.next_id,
            date,
            title,
            time,
            desc,
        };
        self.tasks.push(new_task);
        self.sort_tasks();
        self.next_id += 1;
    }

    pub fn remove_task(&mut self, task_id: usize) {
        self.tasks.retain(|t| t.id != task_id);
    }

    pub fn update_task(
        &mut self,
        task_id: usize,
        date: Date,
        title: String,
        time: Time,
        desc: String,
    ) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.date = date;
            task.title = title;
            task.time = time;
            task.desc = desc;
        }
        self.sort_tasks();
    }

    fn sort_tasks(&mut self) {
        self.tasks.sort_by(|a, b| {
            match a.date.cmp(&b.date) {
                Ordering::Equal => a.time.partial_cmp(&b.time).unwrap_or(Ordering::Equal),
                other => other,
            }
        });
    }

    pub fn read_tasks() -> Result<Self, FileError> {
        let path = "scheduler.json";
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            if let Ok(data) = serde_json::from_reader::<_, Schedule>(reader) {
                return Ok(data);
            }
        }
        Err(FileError::FileReadError)
    }

    pub fn write_file(&self) -> Result<(), FileError> {
        let path = "scheduler.json";
        match File::create(path) {
            Ok(file) => {
                serde_json::to_writer_pretty(file, &self).unwrap();
                Ok(())
            }
            Err(_e) => Err(FileError::FileWriteError),
        }
    }

    pub fn all_tasks(&self) -> Vec<(usize, Date, String, Time, String)> {
        self.tasks
            .iter()
            .map(|task| (task.id, task.date, task.title.clone(), task.time, task.desc.clone()))
            .collect()
    }
}