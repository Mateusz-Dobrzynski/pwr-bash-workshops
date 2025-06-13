use rand::{rng, seq::SliceRandom, Rng};
use rand_distr::{
    num_traits::{pow, Pow},
    Distribution, Normal,
};

pub struct PagingSimulationResults {
    pub physical_memory_size: i16,
    pub references: Vec<i16>,
    pub hits: i16,
    pub misses: i16,
    pub hit_miss_ratio: f32,
    pub swap_history: Vec<Option<i16>>,
}

pub fn least_recently_used(
    addresses_count: i16,
    physical_memory_size: i16,
    references: Vec<i16>,
) -> PagingSimulationResults {
    let references_copy = references.clone();
    let mut physical_memory = vec![None; physical_memory_size as usize];
    let mut hits = 0;
    let mut misses = 0;
    let mut used_indices: Vec<i16> = vec![];
    let mut swap_history: Vec<Option<i16>> = vec![];
    for reference in references {
        if physical_memory.contains(&Some(reference)) {
            hits += 1;
            let used_index = physical_memory
                .iter()
                .position(|x| x.is_some() && x.unwrap() == reference)
                .unwrap();
            used_indices.remove(used_index);
            used_indices.push(used_index as i16);
            swap_history.push(None);
            continue;
        }
        misses += 1;
        if physical_memory.contains(&None) {
            let free_index = physical_memory
                .iter()
                .position(|index| index.is_none())
                .unwrap();
            physical_memory[free_index] = Some(reference);
            used_indices.push(free_index as i16);
        } else {
            let least_recently_used_index = used_indices[0];
            used_indices.remove(0);
            used_indices.push(least_recently_used_index);
            swap_history.push(Some(least_recently_used_index));
            physical_memory[least_recently_used_index as usize] = Some(reference);
        }
    }
    let hit_miss_ratio: f32 = hits as f32 / misses as f32;
    PagingSimulationResults {
        physical_memory_size,
        references: references_copy,
        hit_miss_ratio,
        swap_history,
        hits,
        misses,
    }
}

pub fn fifo(
    addresses_count: i16,
    physical_memory_size: i16,
    references: Vec<i16>,
) -> PagingSimulationResults {
    let references_copy = references.clone();
    let mut physical_memory = vec![None; physical_memory_size as usize];
    let mut hits = 0;
    let mut misses = 0;
    let mut used_indices: Vec<i16> = vec![];
    let mut swap_history: Vec<Option<i16>> = vec![];
    for reference in references {
        if physical_memory.contains(&Some(reference)) {
            hits += 1;
            continue;
        }
        misses += 1;
        if physical_memory.contains(&None) {
            let free_index = physical_memory
                .iter()
                .position(|index| index.is_none())
                .unwrap();
            physical_memory[free_index] = Some(reference);
            used_indices.push(free_index as i16);
        } else {
            let first_in_index = used_indices[0];
            used_indices.remove(0);
            used_indices.push(first_in_index);
            swap_history.push(Some(first_in_index));
            physical_memory[first_in_index as usize] = Some(reference);
        }
    }
    let hit_miss_ratio: f32 = hits as f32 / misses as f32;
    PagingSimulationResults {
        physical_memory_size,
        references: references_copy,
        hit_miss_ratio,
        swap_history,
        hits,
        misses,
    }
}

pub fn generate_normal_distribution_of_references(
    addresses_count: i16,
    mean_references_count: f32,
    standard_deviation: f32,
) -> Vec<i16> {
    let mut references: Vec<i16> = vec![];
    let mut references_counts: Vec<i16> = vec![];
    let mut rng = rand::rng();
    let mut total_references_count = 0;
    let normal = Normal::new(mean_references_count, standard_deviation).unwrap();

    for _ in 0..addresses_count {
        let sample = normal.sample(&mut rng);
        let count = sample.max(0.0) as i16;
        references_counts.push(count);
        total_references_count += count;
    }

    let current_mean = references_counts.iter().sum::<i16>() as f32 / addresses_count as f32;
    let current_standard_deviation = calculate_standard_deviation(&references_counts, current_mean);

    for i in 0..references_counts.len() {
        let difference = references_counts[i] as f32 - current_mean;
        let normalized_references_count =
            mean_references_count + (standard_deviation / current_standard_deviation) * difference;
        for _ in 0..normalized_references_count.round() as usize {
            references.push(i as i16)
        }
    }

    references.shuffle(&mut rng);

    references
}

fn calculate_standard_deviation(array: &Vec<i16>, mean: f32) -> f32 {
    let mut sum = 0.0;
    for element in array {
        sum += pow(*element as f32 - mean, 2);
    }
    sum / array.len() as f32
}

#[cfg(test)]
mod round_robin_tests {
    use crate::process;

    use super::*;
    #[test]
    fn test_fifo() {
        let references = vec![0, 1, 2, 3, 4, 5, 6, 7, 4, 5, 6, 7];
        let results = fifo(8, 4, references);
        assert!(results.hit_miss_ratio == 4.0 / 8.0)
    }

    #[test]
    fn test_least_recently_used() {
        let references = vec![0, 1, 2, 0, 5, 0];
        let results = fifo(8, 4, references);
        assert!(results.hit_miss_ratio == 2.0 / 4.0)
    }
}
