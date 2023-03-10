use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use regex::Regex;
use std::fs::File;



struct Task {
    prio: i32,
    id: i32,
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
                    id: 8888,
                    description: String::clone(&cap2.to_string())
                })
            }
            None => {
                match re2.captures(&raw) {
                    Some(caps) => {                
                        let cap2 = caps.get(1).unwrap().as_str();
                        return Some(Task {
                            prio: 9999,
                            id: 8888,
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
        let re = Regex::new(r"id=(\d\d\d\d), prio=(\d\d\d\d): (.*)").unwrap();
        match re.captures(&raw) {
            Some(caps) => {
                let cap1 = caps.get(1).unwrap().as_str();
                let id: i32 = cap1.parse().unwrap();
                let cap2 = caps.get(2).unwrap().as_str();
                let prio: i32 = cap2.parse().unwrap();
                let cap3 = caps.get(3).unwrap().as_str();
                return Some(Task {
                    prio: prio,
                    id: id,
                    description: String::clone(&cap3.to_string())
                })
            }
            None => {return None;}
        }
    }
}

fn print_todo(file: &mut File, tasklist: &Vec<Task>) { 
    file.seek(SeekFrom::Start(0)).unwrap();
    tasklist.sort_by(|d1, d2| d1.prio.cmp(&d2.prio));
    for task in tasklist {
        println!(":::id={:04}, prio={:04}: {}",task.id, task.prio, task.description);
        if let Err(e) = writeln!(file, "id={:04}, prio={:04}: {}",task.id, task.prio, task.description) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn get_tasklist(file: &mut File) -> Vec<Task> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let mut tasklist: Vec<Task> = Default::default(); 
    for line in buffer.split('\n') {
        if let Some(old) = Task::from_file(line.to_string()) {
            tasklist.push(old);
        }
    }
    tasklist.sort_by(|d1, d2| d1.id.cmp(&d2.id));
    return tasklist;
}

fn get_task_from_terminal() -> String {
    let mut full = String::new();
    for argument in env::args().skip(1) {
        full = full + &argument + " ";
    }
    return full;
}

fn main() {
    let mut tasklist: Vec<Task> = Default::default(); 
    
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("TODO")
        .unwrap();
    
    let l = env::args().count();
    let mut full = String::new();

    let terminal_task = get_task_from_terminal();
    tasklist = get_tasklist(&mut file);

    if let Some(mut todo) = Task::from_terminal(terminal_task) {
        let last_id = match tasklist.len() {
            0 => 0,
            n => tasklist[n-1].id
        };
        todo.id = last_id + 1;
        tasklist.push(todo);
    }
    

    print_todo(&mut file, &tasklist);
}
