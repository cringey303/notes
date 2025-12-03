use serde::{Serialize, Deserialize};
use std::io;
use chrono::Local;

/*TODO:
-color
-add_note(): valid input loop (same as remove)
*/
fn print_banner() {
    println!("
        ███╗   ██╗ ██████╗ ████████╗███████╗███████╗
        ████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝
        ██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗
        ██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║
        ██║ ╚████║╚██████╔╝   ██║   ███████╗███████║
        ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝");
}

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    id: u32,
    body: String,
    timestamp: String,
}

fn print_menu() {
    println!("-----------------");
    println!("1. Add Note");
    println!("2. Remove Note");
    println!("3. View Notes");
    println!("4. Quit");
    println!("Enter choice: ");
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
            println!("Error: Note cannot be empty");
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
    println!(">Note added");
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
                println!("Error: Enter a valid number");
                continue; //restart loop
            }
        };

        //check bounds
        if id == 0 || id > notes.len() as u32 {
            println!("Error: Note {} does not exist", id);
            continue; //restart loop
        }

        let index = (id - 1) as usize;
        notes.remove(index);
        println!(">Removed Note {}", id);

        //update indices
        for (i, note) in notes.iter_mut().enumerate() {
            note.id = (i+1) as u32;
        }
        break;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    print_banner();
    let mut notes: Vec<Note> = Vec::new();

    loop {
        print_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let clean_input: i32 = input.trim().parse().unwrap_or(0);

        println!();
        //add note
        if clean_input == 1 {
            if let Err(e) = add_note(&mut notes) {
                eprint!("Error adding note: {}", e);
            }

        //remove note
        } else if clean_input == 2 {
            if let Err(e) = remove_note(&mut notes) {
                eprint!("Error removing note: {}", e);
            }

        //view notes
        } else if clean_input == 3 {
            if notes.is_empty() {
                println!("No notes found");
                continue;
            } else { 
                println!("------NOTES------");
                for note in &notes {
                    println!("{}. {} ({})", note.id, note.body, note.timestamp);
                }
                println!("-----------------\n");
            }

        //quit
        } else if clean_input == 4 {
            println!(">Quitting...");
            break; //quit
        } else {
            println!("Error: Invalid choice");
        }
    }
    Ok(())
}


