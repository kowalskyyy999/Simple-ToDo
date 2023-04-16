use std::env::args;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

struct ToDo {
    // id, name task, 
    map: HashMap<String, Status>
}

#[derive(Debug)]
struct Status {
    active: bool,
    status: String,
}

impl ToDo {
    fn new() -> Result<ToDo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open("todo.txt")?;

        let mut content = String::new();

        f.read_to_string(&mut content)?;

        // Version 1
        let map: HashMap<String, Status> = content
            .lines()
            .map(|line| line.splitn(3, "\t").collect::<Vec<&str>>())
            .map(|v| (v[0], v[1], v[2]))
            .map(|(k, a, s)| (String::from(k), 
                        Status { 
                            active: FromStr::from_str(a).unwrap(), 
                            status: s.to_string() }))
            .collect();

        Ok(ToDo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, Status { active: true, status: "On progress".to_string()});
    }

    // How to save the map to disk
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();

        for (k, v) in self.map {
            let record = format!("{}\t{}\t{}\n", k, v.active, v.status);
            content.push_str(&record);
        }
        std::fs::write("todo.txt", content)
    }

    fn complete(&mut self, key: &String) -> Option<()> {

        let status_complete = Status { active: false, status: "Completed".to_string()};

        match self.map.get_mut(key) {
            Some(v) => Some( *v = status_complete),
            None => None
        }
    }

    fn pending(&mut self, key: &str) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(v.status = "Pending".to_string()),
            None => None
        }
    }

}

fn main() {

    if args().len() != 3 {
        eprintln!("Usage: 'action' 'task' ");
        return;
    }

    let action = std::env::args().nth(1).expect("Please specify an action");
    let task = std::env::args().nth(2).expect("Please specify the task");

    // let mut todo = ToDo {
    //     map: HashMap::new(),
    // };

    let mut todo = ToDo::new().expect("Failed initialize object");

    if action == "add" {
        todo.insert(task);
        match todo.save() {
            Ok(_) => println!("todo saved!"),
            Err(msg) => println!("An Error occured : {}", msg)
        }
    } else if action == "complete" {
        match todo.complete(&task) {
            None => println!("{} is not present in the list", task),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved!"),
                Err(msg) => println!("An Error occured : {}", msg)
            }
        }
    } else if action == "pending" {
        match todo.pending(&task) {
            None => println!("{} is not present in the list", task),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved!"),
                Err(msg) => println!("An Error occured : {}", msg)
            }
        }
    }

}
