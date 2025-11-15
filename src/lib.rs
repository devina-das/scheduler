use indexmap::IndexMap;
use std::{cmp::Ordering};
use enum_iterator::{all, Sequence};

#[derive(Debug, Clone, Copy)]
pub enum SchedulerError {
    InvalidTimeFormat, InvalidTime
}

pub enum TaskAttribute {
    Title, Day, Time, Desc 
}

// -----------------------------------------------DayOfWeek-----------------------------------------------

#[derive(Sequence, Hash, PartialEq, Eq, Debug, Clone, Copy)]
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

fn string_to_day(day: String) -> Option<DayOfWeek> {
    match day.as_str() {
        "Monday" => Some(DayOfWeek::Mon),
        "Tuesday" => Some(DayOfWeek::Tue),
        "Wednesday" => Some(DayOfWeek::Wed),
        "Thursday" => Some(DayOfWeek::Thu),
        "Friday" => Some(DayOfWeek::Fri),
        "Saturday" => Some(DayOfWeek::Sat),
        "Sunday" => Some(DayOfWeek::Sun),
        _ => None,
    }
}

// -----------------------------------------------Time-----------------------------------------------

// TIME FORMAT
#[derive(Debug, Clone, Copy)]
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
    pub fn new(time: String) -> Result<Self, SchedulerError> {
        let parts: Vec<&str> = time.split(".").collect();
        if parts.len() != 2 {
            return Err(SchedulerError::InvalidTimeFormat);
        }

        let hour = parts[0].parse::<usize>();
        let min = parts[1].parse::<usize>();

        if hour.is_err() || min.is_err() {
            return Err(SchedulerError::InvalidTime);
        }

        let h_checked = hour.unwrap();
        let m_checked = min.unwrap();

        if h_checked > 23 || m_checked > 59 {
            return Err(SchedulerError::InvalidTime);
        }
        Ok(Self {
            hour: h_checked,
            mins: m_checked
        })

    }
}

// -----------------------------------------------Task-----------------------------------------------

// CHANGED: time (f64) -> time (Time)
#[derive(Debug, Clone)]
struct Task {
    id: usize,
    day: DayOfWeek,
    title: String,
    time: Time,
    desc: String,
}

impl Task {
    // DISPLAY TASK
    //      #0 - Task 1 @ 5.45
    //                This is our first task
    pub fn display(&self) {
        println!("#{} - {} @ {}", self.id, self.title, self.time.to_string());
        print!("          ");
        println!("{}", self.desc);
    }
}

// -----------------------------------------------List-----------------------------------------------
pub struct List {
    next_id: usize,
    schedule: IndexMap<DayOfWeek, Vec<Task>>
}

impl Default for List {
    // CHANGED: simplified the function
    fn default() -> Self {
        Self {
            next_id : 0,
            schedule: all::<DayOfWeek>().map(|day| (day, Vec::<Task>::new())).collect()
        }
    }
}

impl List {
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
            Some((day, idx)) => self.schedule.get_mut(&day).unwrap().remove(idx),
            None => return
        };
    }

    // EDIT TASK
    pub fn edit_task(&mut self, id: usize, atb: TaskAttribute, new: String) {
        let task = self.get_task(id);
        match task {
            Some((day, idx)) => {
                match atb {
                    TaskAttribute::Title => {
                        self.schedule.get_mut(&day).unwrap()[idx].title = new;
                    }
                    TaskAttribute::Desc => {
                        self.schedule.get_mut(&day).unwrap()[idx].desc = new;
                    }
                    TaskAttribute::Time => {
                        let new_time = Time::new(new);
                        match new_time {
                            Ok(time) => {
                                self.schedule.get_mut(&day).unwrap()[idx].time = time;
                            },
                            Err(_error) => return
                        }
                    }
                    TaskAttribute::Day => {
                        let old = self.schedule.get_mut(&day).unwrap().remove(idx);
                        let new_day =  string_to_day(new).unwrap();
                        self.add_task(new_day, old.title, old.time, old.desc);
                    }
                }
            },
            None => return
        }
    }

    // DISPLAY TASKS
    pub fn display(&self) {
        println!("----------------------------------------");
        for (day, tasks) in &self.schedule {
            println!("{}", day.to_string());
            for task in tasks {
                task.display();
            }
        }
    }

    // fetches task at specific id if it exists
    pub fn get_task(&self, target_id: usize) -> Option<(DayOfWeek, usize)> {
        for (day, tasks) in self.schedule.iter() {
            for idx in 0..tasks.len() {
                if tasks[idx].id == target_id {
                    return Some((*day, idx));
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