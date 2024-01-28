use std::time::Instant;

pub mod calculate_pi;
pub mod persistence;

#[allow(unused_imports)]
use calculate_pi::calculate_pi;
#[allow(unused_imports)]
use persistence::find_max_persistence;
#[allow(unused_imports)]
use persistence::multi_thread_find_max_persistence;

// Orginal number is 277777788888899
#[allow(dead_code)]
fn persistence() {
    let mut durations = Vec::<u128>::new();

    let total_start = Instant::now();
    for i in 0..1000 {
        let start = Instant::now();
        //let (max_count, n_skip, number_of_numbers, number_of_threads) =
        //    multi_thread_find_max_persistence(&11, &20, &0);
        let (_, _, _) = find_max_persistence(&11, &20, &0);
        let duration = start.elapsed().as_millis();
        durations.push(duration);
        println!(
            "Iteration: {}\t Time elapsed in milliseconds: {}",
            i, duration
        );
    }

    // calculate the average
    let mut sum = 0;
    for duration in &durations {
        sum += duration;
    }

    let average = sum / durations.len() as u128;

    let total_duration = total_start.elapsed().as_millis();

    println!("Average time elapsed in milliseconds: {}", average);
    println!("Total time elapsed in milliseconds: {}", total_duration);
}

fn pi() {
    let iterations = 10e6.into();
    println!("iterations = {}", iterations);

    let start = Instant::now();
    let pi = calculate_pi(iterations);
    let duration = start.elapsed().as_millis();
    println!("pi = {}", pi);
    println!("Time elapsed in milliseconds: {}", duration);
}

fn main() {
    //persistence();
    pi();
}
