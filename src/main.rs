use schedule::{DayOfWeek, List};

fn main() {
    let mut new_list = List::new();

    // task 1
    let day1 = DayOfWeek::Mon;
    let title1 = String::from("Task 1");
    let time1 = 05.45;
    let desc1 = String::from("This is our first task");
    new_list.add_task(day1, title1, time1, desc1);
    new_list.display();


    // task 2
    let day2 = DayOfWeek::Mon;
    let title2 = String::from("Task 2");
    let time2 = 02.45;
    let desc2 = String::from("This is our second task");
    new_list.add_task(day2, title2, time2, desc2);
    new_list.display();

    // remove task
    new_list.remove_task(1);
    new_list.display();





}
