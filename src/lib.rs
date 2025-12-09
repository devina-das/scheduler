use std::cmp::Ordering;
pub use enum_iterator::{all, Sequence};
use chrono::{Datelike, NaiveDate, Weekday};

use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};

// -----------------------------------------------Errors-----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TimeError {
    InvalidTimeFormat, InvalidTime
}

pub enum FileError {
    FileReadError, FileWriteError
}
// -----------------------------------------------DayOfWeek-----------------------------------------------

#[derive(Serialize, Deserialize, Sequence, Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum DayOfWeek {
    Sun, Mon, Tue, Wed, Thu, Fri, Sat
}

impl ToString for DayOfWeek {
    fn to_string(&self) -> String { 
        match self {
            DayOfWeek::Sun => String::from("Sunday"),
            DayOfWeek::Mon => String::from("Monday"),
            DayOfWeek::Tue => String::from("Tuesday"),
            DayOfWeek::Wed => String::from("Wednesday"),
            DayOfWeek::Thu => String::from("Thursday"),
            DayOfWeek::Fri => String::from("Friday"),
            DayOfWeek::Sat => String::from("Saturday")
        }
    }
}

impl TryFrom<usize> for DayOfWeek {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DayOfWeek::Sun),
            1 => Ok(DayOfWeek::Mon),
            2 => Ok(DayOfWeek::Tue),
            3 => Ok(DayOfWeek::Wed),
            4 => Ok(DayOfWeek::Thu),
            5 => Ok(DayOfWeek::Fri),
            6 => Ok(DayOfWeek::Sat),
            _ => Err(())
        }
    }
}

impl From<DayOfWeek> for usize {
    fn from(value: DayOfWeek) -> Self {
        match value {
            DayOfWeek::Sun => 0,
            DayOfWeek::Mon => 1,
            DayOfWeek::Tue => 2,
            DayOfWeek::Wed => 3,
            DayOfWeek::Thu => 4,
            DayOfWeek::Fri => 5,
            DayOfWeek::Sat => 6
        }
    }
}

impl From<Weekday> for DayOfWeek {
    fn from(value: Weekday) -> Self {
        match value {
            Weekday::Sun => DayOfWeek::Sun,
            Weekday::Mon => DayOfWeek::Mon,
            Weekday::Tue => DayOfWeek::Tue,
            Weekday::Wed => DayOfWeek::Wed,
            Weekday::Thu => DayOfWeek::Thu,
            Weekday::Fri => DayOfWeek::Fri,
            Weekday::Sat => DayOfWeek::Sat
        }
    }
}

// -----------------------------------------------Date-----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Date {
    pub fn from_naive(date: NaiveDate) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }

    pub fn to_naive(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year, self.month, self.day).unwrap()
    }

    pub fn day_of_week(&self) -> DayOfWeek {
        self.to_naive().weekday().into()
    }
}

// -----------------------------------------------Time-----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Time {
    pub hour: usize,
    pub mins: usize
}

impl ToString for Time {
    fn to_string(&self) -> String {
        let mut hour_disp = self.hour.to_string();
        if self.hour < 10 {
            hour_disp = String::from("0") + hour_disp.as_str();
        }
        let mut min_disp = self.mins.to_string();
        if self.mins < 10 {
            min_disp = String::from("0") + min_disp.as_str();
        }
        hour_disp + ":" + min_disp.as_str()
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.mins == other.mins
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.hour, self.mins).partial_cmp(&(other.hour, other.mins))
    }
}

impl Time {
    // NEW TIME -> from String input
    pub fn new(time: String) -> Result<Self, TimeError> {
        let parts: Vec<&str> = time.split(":").collect();
        if parts.len() != 2 {
            return Err(TimeError::InvalidTimeFormat);
        }

        let hour = parts[0].parse::<usize>();
        let min = parts[1].parse::<usize>();

        if hour.is_err() || min.is_err() {
            return Err(TimeError::InvalidTime);
        }

        let h_checked = hour.unwrap();
        let m_checked = min.unwrap();

        if h_checked > 23 || m_checked > 59 {
            return Err(TimeError::InvalidTime);
        }
        Ok(Self {
            hour: h_checked,
            mins: m_checked
        })

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

    pub fn tasks_by_day(&self, day: DayOfWeek) -> Vec<(usize, Date, String, Time, String)> {
        self.tasks
            .iter()
            .filter(|task| task.date.day_of_week() == day)
            .map(|task| (task.id, task.date, task.title.clone(), task.time, task.desc.clone()))
            .collect()
    }
}