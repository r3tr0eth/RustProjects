use clap::{Parser, Subcommand};
use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead};
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
    }
}

fn add_task(task: &str) -> io::Result<()> {
    let file_path = "tasks.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    writeln!(file, "{}", task)?;
    println!("Tarea agregada: {}", task);
    Ok(())
}

fn list_tasks() -> io::Result<()> {
    let file_path = "tasks.txt";
    if Path::new(file_path).exists() {
        let file = fs::File::open(file_path)?;
        let reader = io::BufReader::new(file);
        println!("Lista de tareas:");
        for line in reader.lines() {
            println!("- {}", line?);
        }
    } else {
        println!("No hay tareas guardadas.");
    }
    Ok(())
}