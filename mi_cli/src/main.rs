use clap::{Parser, Subcommand};
use eframe::egui;
use eframe::App;
use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser)]
#[command(name = "mi_cli")]
#[command(version = "1.0")]
#[command(author = "Tu Nombre")]
#[command(about = "Una CLI simple para gestionar tareas")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inicia la interfaz gráfica
    Gui,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gui => {
            let options = eframe::NativeOptions {
                initial_window_size: Some(egui::vec2(400.0, 300.0)),
                ..Default::default()
            };
            eframe::run_native("Gestor de Tareas", options, Box::new(|_cc| Box::<MyApp>::default()));
        }
    }
}

#[derive(Default)]
struct MyApp {
    tasks: Vec<Task>,
    new_task: String,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Gestor de Tareas");
            ui.horizontal(|ui| {
                ui.label("Nueva tarea:");
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("Agregar").clicked() {
                    self.add_task();
                }
            });

            ui.separator();
            ui.label("Lista de tareas:");
            for (index, task) in self.tasks.iter().enumerate() {
                let mut completed = task.completed;
                ui.horizontal(|ui| {
                    ui.checkbox(&mut completed, "");
                    ui.label(&task.description);
                    if completed != task.completed {
                        self.complete_task(index);
                    }
                });
            }
        });

        self.load_tasks();
    }
}

impl MyApp {
    fn add_task(&mut self) {
        if !self.new_task.is_empty() {
            self.tasks.push(Task {
                description: self.new_task.clone(),
                completed: false,
            });
            self.new_task.clear();
            self.save_tasks();
        }
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = !task.completed;
            self.save_tasks();
        }
    }

    fn load_tasks(&mut self) {
        if let Ok(tasks) = read_tasks() {
            self.tasks = tasks;
        }
    }

    fn save_tasks(&self) {
        write_tasks(&self.tasks).expect("Error al guardar las tareas");
    }
}

fn read_tasks() -> io::Result<Vec<Task>> {
    let file_path = "tasks.json";
    if Path::new(file_path).exists() {
        let data = fs::read_to_string(file_path)?;
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        Ok(tasks)
    } else {
        Ok(vec![])
    }
}

fn write_tasks(tasks: &[Task]) -> io::Result<()> {
    let file_path = "tasks.json";
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(file_path, data)?;
    Ok(())
}