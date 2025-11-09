use indexmap::IndexMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum DayOfWeek {
    Mon, Tue, Wed, Thu, Fri, Sat, Sun
}

// time can be repped as 23.59 for 23:59
struct Task {
    title: String,
    time: f64,
    desc: String,
}

// I was using a HashMap at first, but those are unordered.
// IndexMap is ordered
pub struct List {
    next_id: usize,
    schedule: IndexMap<DayOfWeek, IndexMap<usize, Task>>
}

impl Task {
    pub fn display(&self){
        println!("{} @ {}", self.title, self.time);
        print!("          ");
        println!("{}", self.desc);
    }
}

impl List {
    pub fn new() -> List {
        let mut week: IndexMap<DayOfWeek, IndexMap<usize, Task>> = IndexMap::new();
        week.insert(DayOfWeek::Mon, IndexMap::new());
        week.insert(DayOfWeek::Tue, IndexMap::new());
        week.insert(DayOfWeek::Wed, IndexMap::new());
        week.insert(DayOfWeek::Thu, IndexMap::new());
        week.insert(DayOfWeek::Fri, IndexMap::new());
        week.insert(DayOfWeek::Sat, IndexMap::new());
        week.insert(DayOfWeek::Sun, IndexMap::new());

        List {
            next_id : 0,
            schedule: week
        }
    }

    pub fn add_task(&mut self, day: DayOfWeek, title: String, time: f64, desc: String) {
        if !check_time(time) {
            println!("invalid time");
            return;
        }
        let new_task = Task {title, time, desc};
        self.schedule.get_mut(&day).unwrap().insert(self.next_id, new_task);
        self.order_tasks();
        self.next_id += 1;
    }

    pub fn remove_task(&mut self, target: usize) {
        for (_, tasks) in self.schedule.iter_mut() {
            if tasks.shift_remove(&target).is_some() {
                return;
            }
        }
    }

    pub fn edit_task() {
        todo!();
    }

    pub fn display(&self) {
        println!("/////////////////////////////////////////");
        for (day, tasks) in &self.schedule {
            println!("{}", to_string(*day));
            for (id, task) in tasks {
                print!("     ");
                print!("#{} - ", id);
                task.display();
            }
        }
    }

    // orders tasks by time
    fn order_tasks(&mut self) {
        for (_, tasks) in self.schedule.iter_mut() {
            for i in 0..tasks.len() {
                for j in 0..tasks.len() - i - 1 {
                    let t1 = tasks.get_index(j).unwrap().1.time;
                    let t2 = tasks.get_index(j + 1).unwrap().1.time;
                    if t1 > t2 {
                        tasks.swap_indices(j, j + 1); 
                    }
                }
            }
        }
    }
}

fn check_time(time: f64) -> bool {
    let int = time.floor();
    let dec = time - int;
    int >= 0.00 && int < 24.00 && dec >= 0.00 && dec < 0.60 
}

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