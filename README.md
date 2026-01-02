# NotesCLI

<img width="343" height="186" alt="Screenshot 2026-01-01 at 15 12 51" src="https://github.com/user-attachments/assets/bf54fd04-303a-4223-9a80-da11f89388a1" />

## Installation

### Prerequisites
You need to have **Rust** and **Cargo** installed on your machine. You can install them here: [rustup.rs](https://rustup.rs/).

### Installing with **Cargo**
You can install this tool directly from GitHub using Cargo. 

```bash
cargo install --git https://github.com/cringey303/notes
```
After installing, run
```bash
notes
```

## Features:
- Uses serde to convert note objects into saveable strings
- Saves to `notes.json` for continuity
- Uses Inquire for a selectable menu
- Terminal colors

Next steps:
- Add note edit/search functionality
- Add CLAP (command line argument parser) functionality for quick commands

## OR Running from source

### Installation & Usage
1. **Clone the repository** (or download the files):
   ```bash
   git clone https://github.com/cringey303/notes
   cd notes
2. **Run the program**
   ```bash
   cargo run
Now you are ready to create notes!
