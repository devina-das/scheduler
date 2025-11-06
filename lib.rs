use std::collections::HashMap;

enum Importance {
    Low, 
    Medium, 
    High,
}

enum DayOfWeek {
    Mon, 
    Tue, 
    Wed, 
    Thu, 
    Fri, 
    Sat, 
    Sun,
}

// I'm thinking time can be represented as 5.45 for 5:45
// We could bound it from 0.00 -> 23.59
// We would have to make sure the decimal part is like 0 -> 59
struct Task {
    title: String,
    time: f64,
    importance: Importance,
    desc: String,
    status: bool,
}

// Tasks[0] -> (Mon) for example
pub struct List {
    next_id: usize,
    tasks: Vec<HashMap<usize, Task>>,
}

impl Task {
    pub fn edit_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn edit_time(&mut self, time: f64) {
        if !check_time(time) {
            println!("Invalid time! Must be 0.00 -> 23.59");
            return;
        }
        self.time = time;
    }

    pub fn edit_importance(&mut self, importance: Importance) {
        self.importance = importance;
    }

    pub fn edit_desc(&mut self, desc: String) {
        self.desc = desc;
    }

    pub fn mark_done(&mut self) {
        self.status = true;
    }
}

impl List {
    pub fn new() -> List {
        let mut days = Vec::new();
        for _ in 0..7 {
            days.push(HashMap::new());
        }
        List {
            next_id: 0,
            tasks: days,
        }
    }

    pub fn add_task(&mut self, day: DayOfWeek, title: String, time: f64, importance: Importance, desc: String) {
        if !check_time(time) {
            println!("Invalid time! Must be 0.00 -> 23.59");
            return;
        }

        let new_task = Task {
            title,
            time,
            importance,
            desc,
            status: false,
        };

        let idx = match day {
            DayOfWeek::Mon => 0,
            DayOfWeek::Tue => 1,
            DayOfWeek::Wed => 2,
            DayOfWeek::Thu => 3,
            DayOfWeek::Fri => 4,
            DayOfWeek::Sat => 5,
            DayOfWeek::Sun => 6,
        };

        self.tasks[idx].insert(self.next_id, new_task);
        self.next_id += 1;
    }

    pub fn remove_task(&mut self, day: DayOfWeek, id: usize) {
        let idx = match day {
            DayOfWeek::Mon => 0,
            DayOfWeek::Tue => 1,
            DayOfWeek::Wed => 2,
            DayOfWeek::Thu => 3,
            DayOfWeek::Fri => 4,
            DayOfWeek::Sat => 5,
            DayOfWeek::Sun => 6,
        };
        self.tasks[idx].remove(id);
    }

    pub fn edit_task()
}


// Kinda the logic for checking time, but I don't know where/how to implement.
fn check_time(time: f64) -> bool {
    let int = time.floor();
    let dec = time - int;
    int >= 0.00 && int < 24.00 && dec >= 0.0 && dec < 0.60 
}