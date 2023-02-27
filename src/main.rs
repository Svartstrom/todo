
use std::env;

use std::fs::OpenOptions;
use std::io::prelude::*;

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
    if let Err(e) = writeln!(file, "{}",full) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
