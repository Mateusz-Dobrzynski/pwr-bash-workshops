use rand::{rng, Rng};
use rand_distr::{Distribution, Normal};

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
    for i in 0..addresses_count {
        let references_count = normal.sample(&mut rng).round() as i16;
        references_counts.push(references_count);
        total_references_count += references_count as i32;
    }
    for i in 0..total_references_count {
        let address: i16 = rng.random_range(0..addresses_count);
        references.push(address);
        references_counts[address as usize] -= 1;
    }
    references
}
