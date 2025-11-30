use serde::{Serialize, Deserialize};
use std::io;
use chrono::Local;

/*TODO:
-color
-remove_note(): valid input loop
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
    println!("1. Add Note");
    println!("2. Remove Note");
    println!("3. View Notes");
    println!("4. Quit");
    println!("Enter choice: ");
}

fn add_note(notes: &mut Vec<Note>) -> io::Result<()> {
    println!("Enter note: ");
    let mut body = String::new();
    io::stdin().read_line(&mut body)?;

    let now = Local::now();
    let now_formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
    
    let new_note = Note {
        id: notes.len() as u32 + 1,
        body: body.trim().to_string(),
        timestamp: now_formatted,
    };

    notes.push(new_note);
    println!(">Note added");
    Ok(())
}

//TODO: loop to keep taking integer until it is < len notes
fn remove_note(notes: &mut Vec<Note>) -> io::Result<()> {
    println!("Enter note ID: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Failed to read line.");
    let id: u32 = input.trim().parse().expect("Error: ID must be an integer.");
    let index: u32 = id - 1;
    notes.remove(index.try_into().unwrap());
    println!(">Removed note {}", id);

    //update indices
    let mut i: u32 = 1;
    for note in notes {
        note.id = i;
        i+=1;
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
            println!();

        //remove note
        } else if clean_input == 2 {
            if let Err(e) = remove_note(&mut notes) {
                eprint!("Error removing note: {}", e);
            }
            println!();

        //view notes
        } else if clean_input == 3 {
            println!("------NOTES------");
            for note in &notes {
                println!("{}. {} ({})", note.id, note.body, note.timestamp);
            }
            println!("-----------------\n");

        //quit
        } else if clean_input == 4 {
            println!(">Quitting...");
            break; //quit
        } else {
            println!("Error: Invalid choice");
            println!();
        }
    }
    Ok(())
}


