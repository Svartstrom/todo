use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

struct task {
    prio: i32,
    description: String,
}

impl task {
    fn from_terminal (raw: String) -> Self {
        return task {
            prio: 9999,
            description: String::clone(&raw.to_string())
        }
    }

    fn from_file (raw: String) -> Self {
        
    }
}

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("TODO")
        .unwrap();

    let mut full = String::new();
    for argument in env::args().skip(1) {
        full = full + &argument + " ";
    }
    let todo = task::from_terminal(full);

    if let Err(e) = writeln!(file, "={}= {}",todo.prio, todo.description) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
