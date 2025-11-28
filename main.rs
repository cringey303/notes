let mut input = String::new();
io::stdin().read_line(&mut input)?;

//to_string() allocates new memory
//inside loop?
let clean_input = input.trim().to_string(); 

let mut todos: Vec<String> = Vec::new()
loop {
    if input == 1 {

    }
}

struct Note {
    id: u32,
    body: String,
    timestamp: String,
}