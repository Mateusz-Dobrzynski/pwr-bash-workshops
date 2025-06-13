mod experiment;
mod paging;
mod process;
mod scheduling;

fn main() {
    experiment::round_robin_processing_time();
    experiment::fcfs_vs_round_robin();
    experiment::convoy_effect();
    experiment::fifo_vs_least_recently_used();
}
