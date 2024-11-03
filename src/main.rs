use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }

    fn toggle_complete(&mut self) {
        self.completed = !self.completed;
    }
}

struct ToDoApp {
    tasks: Vec<Task>,
    new_task_description: String,
}

impl ToDoApp {
    fn load_tasks() -> Vec<Task> {
        let path = "tasks.json";
        if Path::new(path).exists() {
            let data = fs::read_to_string(path).expect("Konnte Datei nicht laden!");
            serde_json::from_str(&data).expect("Konnte JSON nicht lesen!")
        } else {
            Vec::new()
        }
    }

    fn save_tasks(&self) {
        let data = serde_json::to_string(&self.tasks).expect("Konnte Aufgabe nicht serialisieren!");
        fs::write("tasks.json", data).expect("Fehler beim schreiben!");
    }
}

impl eframe::App for ToDoApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To-Do-Liste");

            let task_list = self.tasks.clone();

            for (i, task) in task_list.iter().enumerate() {
                ui.horizontal(|ui| {
                    let mut completed = task.completed;
                    if ui.checkbox(&mut completed, "").clicked() {
                        self.tasks[i].toggle_complete();
                        self.save_tasks();
                    }
                    ui.label(&task.description);
                });
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task_description);
                if ui.button("Neue Aufgabe").clicked() {
                    if !self.new_task_description.is_empty() {
                        self.tasks.push(Task::new(self.new_task_description.clone()));
                        self.new_task_description.clear();
                        self.save_tasks();
                    }
                }
            });

            if ui.button("Aufgabe speichern!").clicked() {
                self.save_tasks();
            }
        });
    }
}

fn main() {
    let app = ToDoApp {
        tasks: ToDoApp::load_tasks(),
        new_task_description: String::new(),
    };

    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "To-Do Liste",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}