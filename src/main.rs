use eframe::egui;
use schedule::{DayOfWeek, List, Time, SchedulerError};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Scheduler",
        options,
        Box::new(|_cc| Box::new(SchedulerApp::default())),
    )
}

struct SchedulerApp {
    list: List,
    new_title: String,
    new_desc: String,
    new_time: String,
    new_day_idx: usize,
    remove_id: String,
    status_message: String,
}

impl Default for SchedulerApp {
    fn default() -> Self {
        Self {
            list: List::default(),
            new_title: String::new(),
            new_desc: String::new(),
            new_time: String::new(),
            new_day_idx: 0,
            remove_id: String::new(),
            status_message: String::new(),
        }
    }
}

impl eframe::App for SchedulerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Scheduler GUI");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.new_title);
            });

            ui.horizontal(|ui| {
                ui.label("Time (HH.MM):");
                ui.text_edit_singleline(&mut self.new_time);
            });

            ui.horizontal(|ui| {
                ui.label("Day:");
                egui::ComboBox::from_id_source("day_combo")
                    .selected_text(match self.new_day_idx {
                        0 => "Mon",
                        1 => "Tue",
                        2 => "Wed",
                        3 => "Thu",
                        4 => "Fri",
                        5 => "Sat",
                        6 => "Sun",
                        _ => "Mon",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.new_day_idx, 0, "Mon");
                        ui.selectable_value(&mut self.new_day_idx, 1, "Tue");
                        ui.selectable_value(&mut self.new_day_idx, 2, "Wed");
                        ui.selectable_value(&mut self.new_day_idx, 3, "Thu");
                        ui.selectable_value(&mut self.new_day_idx, 4, "Fri");
                        ui.selectable_value(&mut self.new_day_idx, 5, "Sat");
                        ui.selectable_value(&mut self.new_day_idx, 6, "Sun");
                    });
            });

            ui.label("Description:");
            ui.text_edit_multiline(&mut self.new_desc);

            if ui.button("Add Task").clicked() {
                // client-side validation of time format (H.MM) to give immediate feedback
                match Time::new(self.new_time.trim().replace(',', ".")) {
                    Ok(t) => {
                        if let Some(day) = idx_to_day(self.new_day_idx) {
                            let title = std::mem::take(&mut self.new_title);
                            let desc = std::mem::take(&mut self.new_desc);
                            self.list.add_task(day, title, t, desc);
                            self.status_message = format!("Added task at {} on {}", t.to_string(), day.to_string());
                            self.new_time.clear();
                        }
                    }
                    Err(error) => {
                        match error {
                            SchedulerError::InvalidTimeFormat => self.status_message = String::from("Invalid time format. Use HH.MM (e.g. 9.30)"),
                            SchedulerError::InvalidTime => self.status_message = String::from("Invalid time")
                        }
                    }
                }
            }

            ui.separator();
            ui.label("Tasks:");

            // fetch all tasks from the library accessor and present them grouped by day
            let tasks = self.list.all_tasks();
            for day in [
                DayOfWeek::Mon,
                DayOfWeek::Tue,
                DayOfWeek::Wed,
                DayOfWeek::Thu,
                DayOfWeek::Fri,
                DayOfWeek::Sat,
                DayOfWeek::Sun,
            ]
            .iter()
            {
                egui::CollapsingHeader::new(to_string(*day)).show(ui, |ui| {
                    let mut any = false;
                    for t in tasks.iter().filter(|t| t.0 == *day) {
                        any = true;
                        // t is a reference to (DayOfWeek, usize, String, Time, String)
                        let id = t.1;
                        let title = &t.2;
                        let time = t.3;
                        let desc = &t.4;

                        ui.horizontal(|ui| {
                            ui.label(format!("{} @ {}", title, time.to_string()));
                            if ui.small_button("Remove").clicked() {
                                self.list.remove_task(id);
                                self.status_message = format!("Removed {}", id);
                            }
                        });
                        ui.label(desc);
                    }
                    if !any {
                        ui.label("No tasks");
                    }
                });
            }

            ui.separator();
            if !self.status_message.is_empty() {
                ui.colored_label(egui::Color32::from_rgb(100, 200, 100), &self.status_message);
            }
        });
    }
}

fn idx_to_day(idx: usize) -> Option<DayOfWeek> {
    match idx {
        0 => Some(DayOfWeek::Mon),
        1 => Some(DayOfWeek::Tue),
        2 => Some(DayOfWeek::Wed),
        3 => Some(DayOfWeek::Thu),
        4 => Some(DayOfWeek::Fri),
        5 => Some(DayOfWeek::Sat),
        6 => Some(DayOfWeek::Sun),
        _ => None,
    }
}

fn to_string(day: DayOfWeek) -> String {
    match day {
        DayOfWeek::Mon => String::from("Monday"),
        DayOfWeek::Tue => String::from("Tuesday"),
        DayOfWeek::Wed => String::from("Wednesday"),
        DayOfWeek::Thu => String::from("Thursday"),
        DayOfWeek::Fri => String::from("Friday"),
        DayOfWeek::Sat => String::from("Saturday"),
        DayOfWeek::Sun => String::from("Sunday"),
    }
}

