use scheduler::*;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Scheduler",
        options,
        Box::new(|_cc| Box::new(SchedulerApp::default())),
    )
}

struct SchedulerApp {
    schedule: Schedule,
    new_title: String,
    new_desc: String,
    new_time: String,
    new_day_idx: usize,
    status_message: String,
}

impl Default for SchedulerApp {
    fn default() -> Self {
        let schedule;
        if let Ok(sched) = Schedule::read_tasks() {
            schedule = sched;
        } else {
            schedule = Schedule::default();
        }
        Self {
            schedule: schedule,
            new_title: String::new(),
            new_desc: String::new(),
            new_time: String::new(),
            new_day_idx: 0,
            status_message: String::new(),
        }
    }
}

impl eframe::App for SchedulerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Scheduler GUI");
                if ui.small_button("Save & Quit").clicked() {
                    if self.schedule.write_file().is_err() {
                        self.status_message = String::from("File could not be saved.");
                    } else {
                        self.status_message = String::from("File saved");
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.new_title);
            });

            ui.horizontal(|ui| {
                ui.label("Time (HH:MM):");
                ui.text_edit_singleline(&mut self.new_time);
            });

            ui.horizontal(|ui| {
                ui.label("Day:");
                egui::ComboBox::from_id_source("day_combo")
                    .selected_text(match self.new_day_idx {
                        0 => "Sun",
                        1 => "Mon",
                        2 => "Tue",
                        3 => "Wed",
                        4 => "Thu",
                        5 => "Fri",
                        6 => "Sat",
                        _ => "Mon",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.new_day_idx, 0, "Sun");
                        ui.selectable_value(&mut self.new_day_idx, 1, "Mon");
                        ui.selectable_value(&mut self.new_day_idx, 2, "Tue");
                        ui.selectable_value(&mut self.new_day_idx, 3, "Wed");
                        ui.selectable_value(&mut self.new_day_idx, 4, "Thu");
                        ui.selectable_value(&mut self.new_day_idx, 5, "Fri");
                        ui.selectable_value(&mut self.new_day_idx, 6, "Sat");
                    });
            });

            ui.label("Description:");
            ui.text_edit_multiline(&mut self.new_desc);

            if ui.button("Add Task").clicked() {
                // client-side validation of time format (H.MM) to give immediate feedback
                match Time::new(String::from(self.new_time.trim())) {
                    Ok(t) => {
                        if let Ok(day) = self.new_day_idx.try_into() {
                            let title = std::mem::take(&mut self.new_title);
                            let desc = std::mem::take(&mut self.new_desc);
                            self.schedule.add_task(day, title.clone(), t, desc);
                            self.status_message = format!("Added {} @ {} on {}", title.clone(), t.to_string(), day.to_string());
                            self.new_time.clear();
                        }
                    }
                    Err(error) => {
                        match error {
                            TimeError::InvalidTimeFormat => self.status_message = String::from("Invalid time format. Use HH:MM (e.g. 9:30)"),
                            TimeError::InvalidTime => self.status_message = String::from("Invalid time")
                        }
                    }
                }
            }

            ui.separator();
            ui.label("Tasks:");

            // fetch all tasks from the library accessor and present them grouped by day
            let tasks = self.schedule.all_tasks();
            for day in all::<DayOfWeek>() {
                let day_text = format!("{} - {}", day.to_string(), day.date());
                egui::CollapsingHeader::new(day_text).show(ui, |ui| {
                    let mut any = false;
                    for t in tasks.iter().filter(|t| t.0 == day) {
                        any = true;
                        // t is a reference to (DayOfWeek, usize, String, Time, String)
                        let id = t.1;
                        let title = &t.2;
                        let time = t.3;
                        let desc = &t.4;

                        ui.horizontal(|ui| {
                            ui.label(format!("{} @ {}", title, time.to_string()));
                            if ui.small_button("Remove").clicked() {
                                self.schedule.remove_task(day, id);
                                self.status_message = format!("Removed {} @ {}", title, time.to_string());
                            }
                            if ui.small_button("Edit").clicked() {
                                self.new_title = title.clone();
                                self.new_time = time.to_string();
                                self.new_day_idx = day.into();
                                self.new_desc = desc.clone();
                                self.status_message = format!("Editing {} @ {}. Click \"Add Task\" when done.", title, time.to_string());
                                self.schedule.remove_task(day, id);
                            }

                        });
                        ui.horizontal(|ui| {
                            ui.add_space(20.0);
                            ui.label(desc);
                        });
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