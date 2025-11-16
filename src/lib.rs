use indexmap::IndexMap;
use std::{cmp::Ordering};
pub use enum_iterator::{all, Sequence};

use std::fs::File;
use std::io::{BufReader};
use serde::{Serialize, Deserialize};

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
    Mon, Tue, Wed, Thu, Fri, Sat, Sun
}

impl ToString for DayOfWeek {
    fn to_string(&self) -> String { 
        match self {
            DayOfWeek::Mon => String::from("Monday"),
            DayOfWeek::Tue => String::from("Tuesday"),
            DayOfWeek::Wed => String::from("Wednesday"),
            DayOfWeek::Thu => String::from("Thursday"),
            DayOfWeek::Fri => String::from("Friday"),
            DayOfWeek::Sat => String::from("Saturday"),
            DayOfWeek::Sun => String::from("Sunday")
        }
    }
}

impl TryFrom<usize> for DayOfWeek {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DayOfWeek::Mon),
            1 => Ok(DayOfWeek::Tue),
            2 => Ok(DayOfWeek::Wed),
            3 => Ok(DayOfWeek::Thu),
            4 => Ok(DayOfWeek::Fri),
            5 => Ok(DayOfWeek::Sat),
            6 => Ok(DayOfWeek::Sun),
            _ => Err(())
        }
    }
}

impl DayOfWeek {
    pub fn to_idx(&self) -> usize {
        match self {
            DayOfWeek::Mon => 0,
            DayOfWeek::Tue => 1,
            DayOfWeek::Wed => 2,
            DayOfWeek::Thu => 3,
            DayOfWeek::Fri => 4,
            DayOfWeek::Sat => 5,
            DayOfWeek::Sun => 6,
        }
    }
}
// -----------------------------------------------Time-----------------------------------------------

// TIME FORMAT
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Time {
    hour: usize,
    mins: usize
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

// CHANGED: time (f64) -> time (Time)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    id: usize,
    day: DayOfWeek,
    title: String,
    time: Time,
    desc: String,
}

// -----------------------------------------------List-----------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    next_id: usize,
    schedule: IndexMap<DayOfWeek, Vec<Task>>
}

// OLD NAME -> List
impl Default for Schedule {
    // CHANGED: simplified the function
    fn default() -> Self {
        Self {
            next_id : 0,
            schedule: all::<DayOfWeek>().map(|day| (day, Vec::<Task>::new())).collect()
        }
    }
}

impl Schedule {
    // ADD TASK
    // CHANGED: accepts Time insted of f64
    pub fn add_task(&mut self, day: DayOfWeek, title: String, time: Time, desc: String) {
        let new_task = Task {id: self.next_id, day: day, title: title, time: time, desc: desc};
        let target_day = self.schedule.get_mut(&day).unwrap();
        target_day.push(new_task);
        target_day.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self.next_id += 1;
    }


    // REMOVE TASK
    pub fn remove_task(&mut self, target_id: usize) {
        let task = self.get_task(target_id);
        match task {
            Some((tsk, idx)) => self.schedule.get_mut(&tsk.day).unwrap().remove(idx),
            None => return
        };
    }

    // READ FILE DATA
    pub fn read_tasks() -> Result<Self, FileError>{
        let path = "scheduler.json";
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            if let Ok(data) = serde_json::from_reader::<_, Schedule>(reader) {
                return Ok(Self {next_id: data.next_id, schedule: data.schedule});
            }
        }
        return Err(FileError::FileReadError);
    }

    // WRITE FILE DATA
    pub fn write_file(&self) -> Result<(), FileError> {
        let path = "scheduler.json";
        match File::create(path) {
            Ok(file) => {
                serde_json::to_writer_pretty(file, &self).unwrap();
                Ok(())
            },
            Err(_e) => Err(FileError::FileWriteError)
        }
    }

    // fetches task at specific id if it exists
    pub fn get_task(&self, target_id: usize) -> Option<(Task, usize)> {
        for (_, tasks) in self.schedule.iter() {
            for idx in 0..tasks.len() {
                if tasks[idx].id == target_id {
                    return Some((tasks[idx].clone(), idx));
                }
            }
        }
        None
    }

    // public accessor to return all tasks with their day and fields as owned data
    // Returns a Vec of tuples: (DayOfWeek, id, title, time, desc)
    pub fn all_tasks(&self) -> Vec<(DayOfWeek, usize, String, Time, String)> {
        let mut out: Vec<(DayOfWeek, usize, String, Time, String)> = Vec::new();
        for (_, tasks) in &self.schedule {
            for task in tasks {
                out.push((task.day, task.id, task.title.clone(), task.time, task.desc.clone()));
            }
        }
        out
    }
}