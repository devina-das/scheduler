use scheduler::*;
use eframe::egui;
use chrono::{Weekday, Local};

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
    new_date: Date,
    new_time: Time,
    status_message: String,
    editing_id: Option<usize>,
}

impl Default for SchedulerApp {
    fn default() -> Self {
        let schedule = Schedule::read_tasks().unwrap_or_else(|_| Schedule::default());
        let today = Date::default();

        Self {
            schedule,
            new_title: String::new(),
            new_desc: String::new(),
            new_date: today,
            new_time: Time::default(),
            status_message: String::new(),
            editing_id: None,
        }
    }
}

impl eframe::App for SchedulerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let today = Date::default();

        // Header & Save Button
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
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
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                // LEFT PANEL - Event Creation/Editing
                columns[0].vertical(|ui| {
                    let heading = if self.editing_id.is_some() {"Edit Event"} else {"New Event"};
                    ui.heading(heading);
                    ui.separator();

                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.new_title);


                    
                    // Date input field
                    ui.label("Date:");
                    ui.label(format!(
                        "{} - {}",
                        self.new_date.to_string(),
                        Weekday::from(self.new_date).to_string()
                    ));
                    
                    // Inline date picker
                    ui.separator();
                    ui.label("Pick from calendar:");
                    
                    // Month/Year navigation
                    ui.horizontal(|ui| {
                        if ui.small_button("◀ Prev Month").clicked() {
                            if self.new_date.month == 1 {
                                self.new_date.month = 12;
                                self.new_date.year -= 1;
                            } else {
                                self.new_date.month -= 1;
                            }
                        }
                        ui.label(format!("{:02}/{}", self.new_date.month, self.new_date.year));
                        if ui.small_button("Next Month ▶").clicked() {
                            if self.new_date.month == 12 {
                                self.new_date.month = 1;
                                self.new_date.year += 1;
                            } else {
                                self.new_date.month += 1;
                            }
                        }
                    });
                    
                    // Weekday headers
                    ui.horizontal(|ui| {
                        for day_label in &["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] {
                            ui.vertical(|ui| {
                                ui.set_width(30.0);
                                ui.centered_and_justified(|ui| {
                                    ui.label(*day_label);
                                });
                            });
                        }
                    });
                    
                    // Day grid
                    if let Some(first_day) = chrono::NaiveDate::from_ymd_opt(self.new_date.year, self.new_date.month, 1) {
                        use chrono::Datelike;
                        let first_weekday = first_day.weekday().num_days_from_sunday() as usize;
                        
                        let days_in_month = if self.new_date.month == 12 {
                            31
                        } else {
                            match chrono::NaiveDate::from_ymd_opt(self.new_date.year, self.new_date.month + 1, 1) {
                                Some(next) => {
                                    let prev = next.pred_opt().unwrap();
                                    prev.day()
                                }
                                None => 31,
                            }
                        };
                        
                        let mut day = 1u32;
                        let mut offset = first_weekday;
                        
                        for _ in 0..6 {
                            if day > days_in_month {
                                break;
                            }
                            ui.horizontal(|ui| {
                                for _ in 0..7 {
                                    ui.vertical(|ui| {
                                        ui.set_width(30.0);
                                        if offset > 0 {
                                            ui.label(" ");
                                            offset -= 1;
                                        } else if day <= days_in_month {
                                            let is_selected = day == self.new_date.day;
                                            let btn_color = if is_selected {
                                                egui::Color32::from_rgb(100, 150, 255)
                                            } else {
                                                egui::Color32::from_rgb(200, 200, 200)
                                            };
                                            let btn = egui::Button::new(
                                                egui::RichText::new(day.to_string())
                                                    .color(if is_selected { egui::Color32::WHITE } else { egui::Color32::BLACK })
                                            ).fill(btn_color);
                                            
                                            if ui.add_sized([30.0, 20.0], btn).clicked() {
                                                self.new_date.day = day;
                                            }
                                            day += 1;
                                        } else {
                                            ui.label(" ");
                                        }
                                    });
                                }
                            });
                        }
                    }

                    // Time Input
                    ui.label("Time:");
                    let btn_size = [10.0, 10.0];
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if ui.add_sized(btn_size,egui::Button::new("+")).clicked() {
                                self.new_time.hour = if self.new_time.hour >= 12 {1} else {self.new_time.hour + 1}
                            }
                            ui.label(format!("{:02}",self.new_time.hour));
                            if ui.add_sized(btn_size,egui::Button::new("–")).clicked() {
                               self.new_time.hour = if self.new_time.hour <= 1 {12} else {self.new_time.hour - 1}
                            }
                        });
                        ui.label(":");
                        ui.vertical(|ui| {
                            if ui.add_sized(btn_size,egui::Button::new("+")).clicked() {
                                self.new_time.min = if self.new_time.min >= 59 {0} else {self.new_time.min + 1}
                            }
                            ui.label(format!("{:02}",self.new_time.min));
                            if ui.add_sized(btn_size,egui::Button::new("–")).clicked() {
                               self.new_time.min = if self.new_time.min <= 0 {59} else {self.new_time.min - 1}
                            }
                        });
                        ui.vertical(|ui| {
                            if ui.add_sized(btn_size,egui::Button::new("+")).clicked() {
                                self.new_time.post = !self.new_time.post;
                            }
                            ui.label(if self.new_time.post {"PM"} else {"AM"});
                            if ui.add_sized(btn_size,egui::Button::new("–")).clicked() {
                               self.new_time.post = !self.new_time.post;
                            }
                        });
                    });

                    // Description Input
                    ui.label("Description:");
                    ui.text_edit_multiline(&mut self.new_desc);


                    // Add/Update Task
                    let button_label = if self.editing_id.is_some() {"Update Task"} else {"Add Task"};
                    if ui.button(button_label).clicked() {
                        if self.new_title.is_empty() {
                            self.status_message = String::from("Title cannot be empty.");
                        } else {
                            let time = std::mem::take(&mut self.new_time);
                            let title = std::mem::take(&mut self.new_title);
                            let desc = std::mem::take(&mut self.new_desc);

                            if let Some(id) = self.editing_id {
                                self.schedule.update_task(id, self.new_date, title.clone(), time, desc);
                                self.status_message = format!("Updated {} @ {} on {}", title, time.to_string(), self.new_date.to_string());
                                self.editing_id = None;
                            } else {
                                self.schedule.add_task(self.new_date, title.clone(), time, desc);
                                self.status_message = format!("Added {} @ {} on {}", title, time.to_string(), self.new_date.to_string());
                            }

                            let today = Date::from(Local::now().date_naive());
                            self.new_date = today;
                            self.new_time = Time::default();
                        }
                    }

                    if self.editing_id.is_some() {
                        if ui.button("Cancel Edit").clicked() {
                            self.new_title.clear();
                            self.new_desc.clear();
                            self.new_date = today;
                            self.new_time = Time::default();
                            self.editing_id = None;
                            self.status_message.clear();
                        }
                    }

                    ui.separator();
                    if !self.status_message.is_empty() {
                        ui.colored_label(egui::Color32::from_rgb(100, 200, 100), &self.status_message);
                    }
                });

                // RIGHT PANEL - Task List
                columns[1].vertical(|ui| {
                    ui.heading("Tasks");
                    ui.separator();

                    let tasks = self.schedule.all_tasks();
                    if tasks.is_empty() {
                        ui.label("No tasks scheduled");
                    } else {
                        for task in tasks {
                            let id = task.0;
                            let date = task.1;
                            let title = &task.2;
                            let time = task.3;
                            let desc = &task.4;

                            ui.horizontal(|ui| {
                                let rich_text = egui::RichText::new(format!("{} - {} @ {}", date.to_string(), title, time.to_string()));
                                let stylized = if date < today {rich_text.strikethrough()}
                                                else if date == today {rich_text.color(egui::Color32::from_rgb(100, 150, 255))}
                                                else {rich_text};
                                ui.label(stylized);
                                if ui.small_button("Edit").clicked() {
                                    self.new_title = title.clone();
                                    self.new_date = date;
                                    self.new_time = time;
                                    self.new_desc = desc.clone();
                                    self.editing_id = Some(id);
                                    self.status_message = format!("Editing {}. Click \"Update Task\" when done.", title);
                                }
                                if ui.small_button("Delete").clicked() {
                                    self.schedule.remove_task(id);
                                    self.status_message = format!("Deleted {} @ {}", title, time.to_string());
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.add_space(20.0);
                                ui.label(desc);
                            });
                        }
                    }
                });
            });
        });
    }
}