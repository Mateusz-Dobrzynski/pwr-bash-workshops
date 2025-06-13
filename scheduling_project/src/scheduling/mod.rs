use std::{
    fs::File,
    io::{Error, Write},
    vec,
};

use crate::process::Process;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Simulation results for scheduling simulations
pub struct SimulationResults {
    processes: Vec<Process>,
    pub average_waiting_time: f32,
    total_execution_time: f32,
    processing_history: Vec<ProcessingRecord>,
}

/// Utility functions making it easier to read simulation results
impl SimulationResults {
    pub fn print(&self) -> () {
        println!("Processes count: {}\nAverage waiting time: {}\nTotal execution time: {}\nDetailed processing history:", self.processes.len(), self.average_waiting_time, self.total_execution_time);
        for record in &self.processing_history {
            println!(
                "{}:\n\tProcess {}\n\tProcessing time: {}",
                &record.start_time, &record.process_name, &record.duration
            )
        }
    }

    pub fn to_csv(&self, path: &str) -> Result<(), Error> {
        let mut output: String = "Time,Process_name,Processing_time".to_owned();
        for record in &self.processing_history {
            output = format!(
                "{}\n{},{},{}",
                output, record.start_time, record.process_name, record.duration
            )
        }
        let mut file = File::create(path).unwrap();
        file.write_all(output.as_bytes()).unwrap();
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
/// A bit of processing history that denotes
/// which process was handled and for how long
pub struct ProcessingRecord {
    pub start_time: f32,
    pub duration: f32,
    pub process_name: String,
}

/// FCFS scheduling algorithm implementation
pub fn fcfs(queue: Vec<Process>) -> SimulationResults {
    let mut time: f32 = 0.0;
    let mut history: Vec<ProcessingRecord> = vec![];
    let mut total_waiting_time: f32 = 0.0;
    for process in &queue {
        history.push(ProcessingRecord {
            start_time: time,
            duration: process.burst_time,
            process_name: process.name.clone(),
        });
        total_waiting_time += time - process.arrival_time;
        time += process.burst_time;
    }
    let processes_count = queue.len() as f32;
    SimulationResults {
        processes: queue,
        average_waiting_time: total_waiting_time / processes_count,
        total_execution_time: time,
        processing_history: history,
    }
}

#[cfg(test)]
/// Test incorporating convoy effect examples
/// presented on a lecture
mod fcfs_convoy_effect_tests {
    use crate::process;

    use super::*;
    #[test]
    fn convoy_effect_bad() {
        let queue: Vec<Process> = vec![
            process::create("P1", 0.0, 24.0, None),
            process::create("P2", 0.0, 3.0, None),
            process::create("P3", 0.0, 3.0, None),
        ];
        let result = fcfs(queue);
        assert!(result.average_waiting_time == 17.0);
    }

    #[test]
    fn convoy_effect_good() {
        let queue: Vec<Process> = vec![
            process::create("P1", 0.0, 3.0, None),
            process::create("P2", 0.0, 3.0, None),
            process::create("P3", 0.0, 24.0, None),
        ];
        let result = fcfs(queue);
        assert!(result.average_waiting_time == 3.0);
    }
}

/// Round-robin algorithm implementation
pub fn round_robin(mut queue: Vec<Process>, processing_time: f32) -> SimulationResults {
    let original_queue = queue.clone();
    let mut time: f32 = 0.0;
    let mut history: Vec<ProcessingRecord> = vec![];
    let mut arrived_processes: Vec<Process> = vec![];
    let mut current_process_index = 0;
    let mut all_processes_total_waiting_time: f32 = 0.0;
    let queue_size = queue.len() as f32;

    time = queue[0].arrival_time;
    simulate_processes_arrival(
        &mut queue, // a mutable reference allows to modify the original object
        &mut arrived_processes,
        &mut time,
    );

    while arrived_processes.len() > 0 || queue.len() > 0 {
        let current_process = &mut arrived_processes[current_process_index];

        // Log waiting time
        if current_process.waiting_since.is_some() && current_process.total_waiting_time.is_some() {
            let waiting_time = time - current_process.waiting_since.unwrap();
            let total_waiting_time = current_process.total_waiting_time.unwrap();
            current_process.total_waiting_time = Some(total_waiting_time + waiting_time);
        } else {
            current_process.total_waiting_time = Some(time - current_process.arrival_time);
        }

        let mut processed_for: f32 = 0.0;
        if current_process.burst_time > processing_time {
            // Process and expropriate
            processed_for = processing_time;
            time += processing_time;
            current_process.burst_time -= processing_time;
            current_process.waiting_since = Some(time);
            save_processing_record(&mut history, &time, &processed_for, current_process);
        } else {
            // Process and finish
            processed_for = current_process.burst_time;
            time += current_process.burst_time;
            current_process.burst_time = 0.0;
            all_processes_total_waiting_time += current_process.total_waiting_time.unwrap();
            save_processing_record(&mut history, &time, &processed_for, current_process);
            arrived_processes.remove(current_process_index);
            if current_process_index != 0 {
                current_process_index -= 1;
            }
        }

        if queue.len() > 0 {
            simulate_processes_arrival(&mut queue, &mut arrived_processes, &mut time);
        }

        // Determine the next process
        if current_process_index + 1 == arrived_processes.len() {
            current_process_index = 0;
        } else {
            current_process_index += 1;
        }
        if arrived_processes.len() == 0 || arrived_processes.len() == 1 {
            println!("test");
        }
    }
    let average_waiting_time = all_processes_total_waiting_time / queue_size;
    SimulationResults {
        processes: original_queue,
        average_waiting_time,
        total_execution_time: time,
        processing_history: history,
    }
}

fn simulate_processes_arrival(
    queue: &mut Vec<Process>,
    arrived_processes: &mut Vec<Process>,
    time: &mut f32,
) {
    let mut indices_to_be_removed: Vec<usize> = vec![];
    for i in 0..queue.len() {
        let next_process = &queue[i];
        if next_process.arrival_time <= *time {
            arrived_processes.push(queue[i].clone());
            indices_to_be_removed.push(i);
        }
    }
    for i in (0..indices_to_be_removed.len()).rev() {
        queue.remove(indices_to_be_removed[i]);
    }
    if arrived_processes.len() == 0 {
        let next_arrival_time = queue[0].arrival_time;
        *time = next_arrival_time;
        simulate_processes_arrival(queue, arrived_processes, time);
    }
}

fn save_processing_record(
    history: &mut Vec<ProcessingRecord>,
    time: &f32,
    processed_for: &f32,
    current_process: &mut Process,
) {
    history.push(ProcessingRecord {
        start_time: *time,
        duration: *processed_for,
        process_name: current_process.name.clone(),
    });
}

#[cfg(test)]
mod round_robin_tests {
    use crate::process;

    use super::*;
    #[test]
    /// Basic test with data from a lecture
    fn test_round_robin() {
        let queue: Vec<Process> = vec![
            process::create("P1", 0.0, 24.0, None),
            process::create("P2", 0.0, 3.0, None),
            process::create("P3", 0.0, 3.0, None),
        ];
        let result = round_robin(queue, 4.0);
        let rounded_waiting_time = (result.average_waiting_time * 100.0).round() / 100.0;
        assert!(result.total_execution_time == 30.0);
        assert!(rounded_waiting_time == 5.67); // Floating point math in action. This should be just 5.67
    }

    #[test]
    /// Testing differing arrival times
    fn test_sequential_arrival_times() {
        let queue: Vec<Process> = vec![
            process::create("P1", 1.0, 4.0, None),
            process::create("P2", 5.0, 4.0, None),
            process::create("P3", 9.0, 4.0, None),
        ];
        let result = round_robin(queue, 4.0);
        let rounded_waiting_time = (result.average_waiting_time * 100.0).round() / 100.0;
        assert!(rounded_waiting_time == 0.0);
        assert!(result.total_execution_time == 13.0);
    }

    #[test]
    /// Testing what happens when there's a gap between one process and another
    fn test_arrival_with_gaps_times() {
        let queue: Vec<Process> = vec![
            process::create("P1", 1.0, 4.0, None),
            process::create("P2", 5.0, 4.0, None),
            process::create("P3", 9.0, 4.0, None),
        ];
        let result = round_robin(queue, 4.0);
        let rounded_waiting_time = (result.average_waiting_time * 100.0).round() / 100.0;
        assert!(rounded_waiting_time == 0.0);
    }

    #[test]
    fn test_process_arrival_simulation() {
        let mut queue: Vec<Process> = vec![
            process::create("P2", 5.0, 4.0, None),
            process::create("P3", 9.0, 4.0, None),
        ];
        let mut arrived_processes: Vec<Process> = vec![];
        let mut time = 5.0;
        simulate_processes_arrival(&mut queue, &mut arrived_processes, &mut time);
        assert!(queue.len() == 1);
        assert!(arrived_processes.len() == 1);
    }
}
