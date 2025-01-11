use clap::{Parser, Subcommand};
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
    /// Agrega una nueva tarea
    Add { task: String },
    /// Lista todas las tareas
    List,
    /// Edita una tarea existente
    Edit { index: usize, task: String },
    /// Elimina una tarea
    Remove { index: usize },
    /// Marca una tarea como completada
    Complete { index: usize },
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => {
            add_task(&task).expect("Error al agregar la tarea");
        }
        Commands::List => {
            list_tasks().expect("Error al listar las tareas");
        }
        Commands::Edit { index, task } => {
            edit_task(index, &task).expect("Error al editar la tarea");
        }
        Commands::Remove { index } => {
            remove_task(index).expect("Error al eliminar la tarea");
        }
        Commands::Complete { index } => {
            complete_task(index).expect("Error al completar la tarea");
        }
    }
}

fn add_task(task: &str) -> io::Result<()> {
    let mut tasks = read_tasks()?;
    tasks.push(Task {
        description: task.to_string(),
        completed: false,
    });
    write_tasks(&tasks)?;
    println!("Tarea agregada: {}", task);
    Ok(())
}

fn list_tasks() -> io::Result<()> {
    let tasks = read_tasks()?;
    if tasks.is_empty() {
        println!("No hay tareas guardadas.");
    } else {
        println!("Lista de tareas:");
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("{}: {} {}", index + 1, status, task.description);
        }
    }
    Ok(())
}

fn edit_task(index: usize, new_task: &str) -> io::Result<()> {
    let mut tasks = read_tasks()?;
    if index > 0 && index <= tasks.len() {
        tasks[index - 1].description = new_task.to_string();
        write_tasks(&tasks)?;
        println!("Tarea editada: {}", new_task);
    } else {
        println!("Índice de tarea no válido.");
    }
    Ok(())
}

fn remove_task(index: usize) -> io::Result<()> {
    let mut tasks = read_tasks()?;
    if index > 0 && index <= tasks.len() {
        tasks.remove(index - 1);
        write_tasks(&tasks)?;
        println!("Tarea eliminada.");
    } else {
        println!("Índice de tarea no válido.");
    }
    Ok(())
}

fn complete_task(index: usize) -> io::Result<()> {
    let mut tasks = read_tasks()?;
    if index > 0 && index <= tasks.len() {
        tasks[index - 1].completed = true;
        write_tasks(&tasks)?;
        println!("Tarea marcada como completada.");
    } else {
        println!("Índice de tarea no válido.");
    }
    Ok(())
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