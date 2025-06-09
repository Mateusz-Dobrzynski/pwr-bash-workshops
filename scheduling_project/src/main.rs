use std::result;

use process::generate_queue;
use rand_distr::{Distribution, Normal};
use serde::Serialize;

use crate::paging::generate_normal_distribution_of_references;

mod paging;
mod process;
mod scheduling;

fn main() {
    let references = generate_normal_distribution_of_references(10, 2.0, 3.0);
    for refer in references {
        println!("{}", refer)
    }
}
