use indexmap::IndexMap;

// repps the different days of the week
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum DayOfWeek {
    Mon, Tue, Wed, Thu, Fri, Sat, Sun
}

// title is basically a quick description
// time is when it takes place. 24-hour clock and 23:59 is repped as 23.59
// desc is a more in-depth description
#[derive(Debug, Clone)]
struct Task {
    id: usize,
    title: String,
    time: f64,
    desc: String,
}

// was using HashMap at first, but did some searching online and IndexMap
// seems better because it's ordered.
pub struct List {
    next_id: usize,
    schedule: IndexMap<DayOfWeek, Vec<Task>>
}

impl Task {
    // displays the task like this:
    //      #0 - Task 1 @ 5.45
    //                This is our first task
    pub fn display(&self) {
        println!("#{} - {} @ {}", self.id, self.title, self.time);
        print!("          ");
        println!("{}", self.desc);
    }
}

impl List {

    // makes a new list
    // this is lowkey jank i know
    pub fn new() -> List {
        let mut week: IndexMap<DayOfWeek,Vec<Task>> = IndexMap::new();
        week.insert(DayOfWeek::Mon, Vec::new());
        week.insert(DayOfWeek::Tue, Vec::new());
        week.insert(DayOfWeek::Wed, Vec::new());
        week.insert(DayOfWeek::Thu, Vec::new());
        week.insert(DayOfWeek::Fri, Vec::new());
        week.insert(DayOfWeek::Sat, Vec::new());
        week.insert(DayOfWeek::Sun, Vec::new());

        List {
            next_id : 0,
            schedule: week
        }
    }

    // adds a new task to the list in order of time
    pub fn add_task(&mut self, day: DayOfWeek, title: String, time: f64, desc: String) {
        if !check_time(time) {
            println!("invalid time");
            return;
        }
        let id = self.next_id;
        let new_task = Task {id, title, time, desc};
        self.schedule.get_mut(&day).unwrap().insert(self.next_id, new_task);
        self.order_tasks();
        self.next_id += 1;
    }


    // removes a task by the specified target id
    pub fn remove_task(&mut self, target_id: usize) {
        for (_, tasks) in self.schedule.iter_mut() {
            for idx in 0..tasks.len() {
                if tasks[idx].id == target_id {
                    tasks.remove(idx);
                    return;
                }
            }
        }
    }


    // edits the specifiec task (by target_id)
    // should also re-order if time is edited
    pub fn edit_task(target_id: usize) {
        todo!();
    }


    // displays the entire list
    pub fn display(&self) {
        println!("----------------------------------------");
        for (day, tasks) in &self.schedule {
            println!("{}", to_string(*day));
            for task in tasks {
                task.display();
            }
        }
    }

    // orders tasks by time
    fn order_tasks(&mut self) {
        for (_, tasks) in self.schedule.iter_mut() {
            for swapped in 0..tasks.len() {
                for idx in 0..tasks.len() - swapped - 1 {
                    if tasks[idx].time > tasks[idx + 1].time {
                        let temp = tasks[idx].clone();
                        tasks[idx] = tasks[idx + 1].clone();
                        tasks[idx + 1] = temp;
                    }
                }
            }
        }
    }
}


// checks if the time is valid
fn check_time(time: f64) -> bool {
    let int = time.floor();
    let dec = time - int;
    int >= 0.00 && int < 24.00 && dec >= 0.00 && dec < 0.60 
}

// turns DayOfWeek into equivalent string
fn to_string(day: DayOfWeek) -> String {
    let day_string = match day {
        DayOfWeek::Mon => String::from("Monday"),
        DayOfWeek::Tue => String::from("Tuesday"),
        DayOfWeek::Wed => String::from("Wednesday"),
        DayOfWeek::Thu => String::from("Thursday"),
        DayOfWeek::Fri => String::from("Friday"),
        DayOfWeek::Sat => String::from("Saturday"),
        DayOfWeek::Sun => String::from("Sunday"),
    };
    day_string
}