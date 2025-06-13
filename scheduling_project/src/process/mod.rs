use std::{
    fs::{self, File},
    io::Write,
};

use rand::{random_bool, Rng};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct Process {
    pub arrival_time: f32,
    pub burst_time: f32,
    pub priority: Option<u16>,
    pub name: String,
    pub waiting_since: Option<f32>,
    pub total_waiting_time: Option<f32>,
}

pub fn create(name: &str, arrival_time: f32, burst_time: f32, priority: Option<u16>) -> Process {
    Process {
        arrival_time,
        priority,
        burst_time,
        name: name.to_owned(),
        waiting_since: None,
        total_waiting_time: None,
    }
}

pub fn read_processes_list_from(path: &str) -> Result<Vec<Process>, Error> {
    let path_content = fs::read_to_string(path).unwrap();
    let queue: Vec<Process> = serde_json::from_str(&path_content).unwrap();
    Ok(queue)
}

pub fn save_processes_list_to_path(queue: &Vec<Process>, path: &str) -> Result<(), Error> {
    let serialized_queue = serde_json::to_string(&queue).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(serialized_queue.as_bytes()).unwrap();
    Ok(())
}

pub fn read_pages_references_from(path: &str) -> Result<Vec<i16>, Error> {
    let path_content = fs::read_to_string(path).unwrap();
    let queue: Vec<i16> = serde_json::from_str(&path_content).unwrap();
    Ok(queue)
}

pub fn save_pages_references_to_path(references: Vec<i16>, path: &str) -> Result<(), Error> {
    let serialized_references = serde_json::to_string(&references).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(serialized_references.as_bytes()).unwrap();
    Ok(())
}

pub fn generate_queue(
    length: i16,
    mean_burst_time: f32,
    burst_time_standard_deviation: f32,
) -> Vec<Process> {
    let mut queue: Vec<Process> = vec![];
    let mut rng = rand::rng();
    let mut time: f32 = 0.0;
    let normal = Normal::new(mean_burst_time, burst_time_standard_deviation).unwrap();

    for i in 0..length {
        let mut burst_time = normal.sample(&mut rand::rng());
        if burst_time < 0.0 {
            burst_time *= -1.0;
        }
        let priority: u16 = rng.random();
        let time_progression = random_bool(0.9);
        if time_progression {
            let random_time_progression: f32 = rng.random();
            time += random_time_progression;
        }
        queue.push(Process {
            arrival_time: time,
            burst_time: burst_time,
            priority: Some(priority),
            name: get_human_readable_process_name(i),
            waiting_since: None,
            total_waiting_time: None,
        });
    }
    queue
}

fn get_human_readable_process_name(process_number: i16) -> String {
    let nato_phonetic_alphabet: Vec<&str> = vec![
        "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India",
        "Juliett", "Kilo", "Lima", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo",
        "Sierra", "Tango", "Uniform", "Victor", "Whiskey", "X-ray", "Yankee", "Zulu",
    ];
    let index: usize = process_number as usize % nato_phonetic_alphabet.len() as usize;
    let suffix = process_number / nato_phonetic_alphabet.len() as i16;
    let name = format!("{}_{}", nato_phonetic_alphabet.get(index).unwrap(), suffix);
    name.to_owned()
}
