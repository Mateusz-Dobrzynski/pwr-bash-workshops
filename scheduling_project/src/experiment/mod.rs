use std::{
    f32::INFINITY,
    fs::File,
    io::{Error, Write},
};

use crate::{
    paging::{fifo, generate_normal_distribution_of_references, least_recently_used},
    process::{generate_queue, save_processes_list_to_path, Process},
    scheduling::{fcfs, round_robin},
};

pub fn round_robin_processing_time() {
    let min_time = 10.0;
    let max_time = 250.0;
    let processes_count = 50;
    let burst_time_standard_deviation = 1.0;
    let mut queues: Vec<Vec<Process>> = vec![];
    let mut average_waiting_times_for_different_execution_times: Vec<f32> = vec![];
    let mut output: String = "Processing time,Mean burst time,Average waiting time\n".to_owned();
    for mean_burst_time in (10..140).step_by(30) {
        let queue = generate_queue(
            processes_count,
            mean_burst_time as f32,
            burst_time_standard_deviation,
        );
        for i in ((min_time as usize)..(max_time as usize)).step_by(20) {
            let robin = round_robin(queue.clone(), i as f32);
            average_waiting_times_for_different_execution_times.push(robin.average_waiting_time);
            output += &format!(
                "{},{},{}\n",
                i,
                mean_burst_time,
                average_waiting_times_for_different_execution_times
                    .last()
                    .unwrap(),
            );
        }
        queues.push(queue);
    }
    let serialized_input = serde_json::to_string(&queues).unwrap();
    save_string_to_file(
        serialized_input,
        "test_data/round_robin_processing_time_input.csv",
    );
    save_string_to_file(output, "test_data/round_robin_processing_time_output.csv");
}

pub fn fcfs_vs_round_robin() {
    let mut queues: Vec<Vec<Process>> = vec![];
    let mut robin_waiting_times: Vec<f32> = vec![];
    let mut fcfs_waiting_times: Vec<f32> = vec![];
    let mut output =
        "Mean burst time,Round-robin avg waiting time,FCFS avg waiting time\n".to_owned();
    for i in (10..110).step_by(10) {
        let queue = generate_queue(50, i as f32, 3.0);
        let robin = round_robin(queue.clone(), (i / 2) as f32);
        let fcfs = fcfs(queue.clone());
        robin_waiting_times.push(robin.average_waiting_time);
        fcfs_waiting_times.push(fcfs.average_waiting_time);
        output += &format!(
            "{},{},{}\n",
            i, robin.average_waiting_time, fcfs.average_waiting_time
        );
        queues.push(queue);
    }
    let serialized_input = serde_json::to_string(&queues).unwrap();
    save_string_to_file(serialized_input, "test_data/fcfs_vs_round_robin_input.json");
    save_string_to_file(output, "test_data/fcfs_vs_round_robin_output.csv");
}

pub fn convoy_effect() {
    let mut asc_queue = generate_queue(50, 10.0, 3.0);
    for i in 0..asc_queue.len() {
        asc_queue[i].arrival_time = 0.0;
    }
    asc_queue.sort_by_key(|entry| entry.burst_time as i16);
    let mut desc_queue = asc_queue.clone();
    desc_queue.reverse();
    let fcfs_asc = fcfs(asc_queue.clone());
    let fcfs_desc = fcfs(desc_queue.clone());
    save_processes_list_to_path(&asc_queue, "test_data/convoy_ascending_input.json");
    save_processes_list_to_path(&desc_queue, "test_data/convoy_descending_input.json");
    fcfs_asc.to_csv("test_data/convoy_asc_output.csv");
    fcfs_desc.to_csv("test_data/convoy_desc_output.csv");
    let summary = format!(
        "ASC,DESC\n{},{}",
        fcfs_asc.average_waiting_time, fcfs_desc.average_waiting_time
    );
    save_string_to_file(summary, "test_data/convoy_summary.csv");
}

pub fn fifo_vs_least_recently_used() {
    let addresses_count = 64;
    let physical_memory_size = 32;
    let mut fifo_worst_ratio: f32 = INFINITY;
    let mut fifo_best_ratio: f32 = 0.0;
    let mut lru_worst_ratio = INFINITY;
    let mut lru_best_ratio = 0.0;
    let mut fifo_ratios: Vec<f32> = vec![];
    let mut lru_ratios: Vec<f32> = vec![];
    let mut all_references: Vec<Vec<i16>> = vec![];

    for _ in 0..100 {
        let references = generate_normal_distribution_of_references(64, 16.0, 10.0);
        all_references.push(references.clone());
        let fifo = fifo(addresses_count, physical_memory_size, references.clone());
        let least_recently_used =
            least_recently_used(addresses_count, physical_memory_size, references);

        lru_ratios.push(least_recently_used.hit_miss_ratio);
        fifo_ratios.push(fifo.hit_miss_ratio);

        if least_recently_used.hit_miss_ratio > lru_best_ratio {
            lru_best_ratio = least_recently_used.hit_miss_ratio;
        } else if least_recently_used.hit_miss_ratio < lru_worst_ratio {
            lru_worst_ratio = least_recently_used.hit_miss_ratio;
        }
        if fifo.hit_miss_ratio > fifo_best_ratio {
            fifo_best_ratio = fifo.hit_miss_ratio;
        }
        if fifo.hit_miss_ratio < fifo_worst_ratio {
            fifo_worst_ratio = fifo.hit_miss_ratio
        }
    }

    let average_lru_ratio = lru_ratios.iter().sum::<f32>() / lru_ratios.len() as f32;
    let average_fifo_ratio = fifo_ratios.iter().sum::<f32>() / lru_ratios.len() as f32;
    let output = format!(
        r#"
        Metric,FIFO,Least recently used
        Average ratio,{},{}
        Best ratio,{},{}
        Worst ratio,{},{}
    "#,
        average_fifo_ratio,
        average_lru_ratio,
        fifo_best_ratio,
        lru_best_ratio,
        fifo_worst_ratio,
        lru_worst_ratio
    );
    let input = serde_json::to_string(&all_references).unwrap();
    save_string_to_file(input, "test_data/fifo_vs_lru_input.csv");
    save_string_to_file(output, "test_data/fifo_vs_lru_output.csv");
}

fn save_string_to_file(string: String, path: &str) -> Result<(), Error> {
    let mut file = File::create(path).unwrap();
    file.write_all(string.as_bytes()).unwrap();
    Ok(())
}
