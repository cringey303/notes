use serde::{Serialize, Deserialize}; //convert from string to object and vice-versa
use std::io;
use std::fs;
use chrono::Local; //note timestamps
use colored::*;
use inquire::Select; //interactive menu
use clap::{Parser, Subcommand}; //CLI functionality

//store notes universally
use std::path::PathBuf;
use directories::ProjectDirs;

//find/create notes folder and json file
fn get_database_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "notes") {
        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
            let _ = fs::create_dir_all(data_dir);
        }

        return data_dir.join("notes.json");
    }
    //creates locally if no home directory
    PathBuf::from("notes.json")
}

fn print_banner() {
    println!("{}", "
███╗   ██╗ ██████╗ ████████╗███████╗███████╗
████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝
██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗
██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║
██║ ╚████║╚██████╔╝   ██║   ███████╗███████║
╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝"
.truecolor(14, 184, 219).bold());
}

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    id: u32,
    body: String,
    timestamp: String,
}

//CLAP Init
#[derive(Parser)]
#[command(name = "notes")]
#[command(about = "CLI Note Application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    //if none, trigger interactive version (handled in main)
    command: Option<Commands>,
}

//define commands
#[derive(Subcommand)]
enum Commands {
    /// add note
    Add {
        note: String
    },
    /// remove note by id
    Remove {
        id: u32
    },
    /// view all notes
    List,
    Path,
}

fn add_note(notes: &mut Vec<Note>) -> io::Result<()> {
    let body: String = loop {
        println!("Enter note ('q' to cancel): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        //cancel
        if input.trim().eq_ignore_ascii_case("q") {
            println!(">Cancelled");
            return Ok(());
        }
        //empty note
        if input.trim().is_empty() {
            println!("{}", "Error: Note cannot be empty".red().bold());
            continue; //restart loop
        }

        break input.trim().to_string();
    };

    let now = Local::now();
    let now_formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
    
    let new_note = Note {
        id: notes.len() as u32 + 1,
        body: body,
        timestamp: now_formatted,
    };

    notes.push(new_note);
    println!("{}", ">Note added".green().bold());
    Ok(())
}

fn remove_note(notes: &mut Vec<Note>) -> io::Result<()> {
    loop {
        println!("Enter note ID to remove ('q' to cancel): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        //cancel
        if input.trim().eq_ignore_ascii_case("q") {
            println!(">Cancelled");
            break;
        }

        let id: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Error: Enter a valid number".red().bold());
                continue; //restart loop
            }
        };

        //check bounds
        if id == 0 || id > notes.len() as u32 {
            println!("{}", format!("Error: Note {} does not exist", id).red().bold());
            continue; //restart loop
        }

        let index = (id - 1) as usize;
        notes.remove(index);
        println!("{}", format!(">Removed Note {}", id).green().bold());

        //update indices
        for (i, note) in notes.iter_mut().enumerate() {
            note.id = (i+1) as u32;
        }
        break;
    }
    Ok(())
}

fn save_notes(notes: &Vec<Note>) {
    let db_path = get_database_path();

    if let Ok(json) = serde_json::to_string_pretty(notes) {
        let _ = fs::write(db_path, json);
    }
}

fn load_notes() -> Vec<Note> {
    let db_path = get_database_path();
    
    if let Ok(data) = fs::read_to_string(db_path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn main() -> io::Result<()> {
    // CLI arguments
    let cli = Cli::parse();

    let mut notes: Vec<Note> = load_notes();

    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Add { note } => {
                // copy add_note logic without loop
                let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let new_note = Note {
                    id: notes.len() as u32 + 1,
                    body: note,
                    timestamp: now,
                };
                notes.push(new_note);
                save_notes(&notes);
                println!("{}", "> Note added".green().bold());
            }
            Commands::Remove { id} => {
                if id == 0 || id > notes.len() as u32 {
                    eprintln!("{}", format!("Error: Note {} does not exist", id).red().bold());
                } else {
                    let index = (id - 1) as usize;
                    notes.remove(index);

                    //update indices
                    for (i, note) in notes.iter_mut().enumerate() {
                        note.id = (i+1) as u32;
                    }
                    save_notes(&notes);
                    println!("{}", format!(">Removed Note {}", id).green().bold());
                }
            }
            Commands::List => {
                if notes.is_empty() {
                    println!("{}", "No notes found".yellow().bold());
                } else {
                    println!("\n------NOTES------");
                    for note in &notes {
                        println!("{}. {} ({})", note.id, note.body, note.timestamp);
                    }
                    println!("-----------------\n");
                }
            }
            Commands::Path => {
                println!("{}", get_database_path().display());
            }
        }
        return Ok(());
    }

    // --- Interactive Loop --- //
    print_banner();

    loop {
        let options = vec![
            "Add Note",
            "Remove Note",
            "View Notes",
            "Quit",
        ];

        let choice = Select::new("", options).prompt();

        match choice {
            Ok("Add Note") => {
                if let Err(e) = add_note(&mut notes) {
                    eprint!("{} {}", "Error adding note:".red().bold(), e);
                } else {
                    save_notes(&notes);
                    println!();
                }
            },
            Ok("Remove Note") => {
                if let Err(e) = remove_note(&mut notes) {
                    eprint!("{} {}", "Error removing note:".red().bold(), e);
                } else {
                    save_notes(&notes);
                    println!();
                }
            },
            Ok("View Notes") => {
                if notes.is_empty() {
                    println!("{}", "No notes found".yellow().bold());
                    continue;
                } else {
                    println!("------NOTES------");
                    for note in &notes {
                        println!("{}. {} ({})", note.id, note.body, note.timestamp);
                    }
                    println!("-----------------\n");
                }
            },
            Ok("Quit") => {
                println!();
                println!("{}", ">Quitting...".yellow().bold());
                break;
            },
            Err(_) => {
                println!("{}", "Error: Invalid choice".red().bold());
            },
            _ => {}
        }
    }
    Ok(())
}
