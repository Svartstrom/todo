use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use regex::Regex;

struct Task {
    prio: i32,
    description: String,
}

impl Task {
    fn from_terminal (raw: String) -> Option<Self> {
        let re = Regex::new(r"^ ?(\d+) (.*)").unwrap();
        let re2 = Regex::new(r"(.+)").unwrap();

        match re.captures(&raw) {
            Some(caps) => {
                let cap1 = caps.get(1).unwrap().as_str();
                let num: i32 = cap1.parse().unwrap();
                let cap2 = caps.get(2).unwrap().as_str();
                return Some(Task {
                    prio: num,
                    description: String::clone(&cap2.to_string())
                })
            }
            None => {
                match re2.captures(&raw) {
                    Some(caps) => {                
                        let cap2 = caps.get(1).unwrap().as_str();
                        return Some(Task {
                            prio: 9999,
                            description: String::clone(&cap2.to_string())
                        })
                    }
                    None => {return None;}
                }
            }
        }
    }

    fn from_file (raw: String) -> Option<Self> {
        if raw.len() == 0 {
            return None;
        }
        let re = Regex::new(r"=(\d*)= (.*)").unwrap();
        match re.captures(&raw) {
            Some(caps) => {
                let cap1 = caps.get(1).unwrap().as_str();
                let num: i32 = cap1.parse().unwrap();
                let cap2 = caps.get(2).unwrap().as_str();
                return Some(Task {
                    prio: num,
                    description: String::clone(&cap2.to_string())
                })
            }
            None => {return None;}
        }
    }
}

/*
fn print_todo(file: OpenOptions, todo: Vec<Task>) { 

}*/

fn main() {
    let mut tasklist: Vec<Task> = Default::default(); 
    
    let mut buffer = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("TODO")
        .unwrap();
    file.read_to_string(&mut buffer);

    let mut full = String::new();
    for argument in env::args().skip(1) {
        full = full + &argument + " ";
    }
   

    for line in buffer.split('\n') {
        if let Some(old) = Task::from_file(line.to_string()) {
            tasklist.push(old);
        }
    }
    if let Some(todo) = Task::from_terminal(full) {
        tasklist.push(todo);
    }
    tasklist.sort_by(|d1, d2| d1.prio.cmp(&d2.prio));

    file.seek(SeekFrom::Start(0));
    for task in tasklist {
        println!("::: ={}= {}",task.prio, task.description);
        if let Err(e) = writeln!(file, "={}= {}",task.prio, task.description) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
