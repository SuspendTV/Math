use std::time::Instant;

pub mod persistence;

#[allow(unused_imports)]
use persistence::find_max_persistence;
#[allow(unused_imports)]
use persistence::multi_thread_find_max_persistence;

// Orginal number is 277777788888899

fn main() {
    let mut durations = Vec::<u128>::new();

    for i in 0..10 {
        let start = Instant::now();
        //let (max_count, n_skip, number_of_numbers, number_of_threads) =
        //    multi_thread_find_max_persistence(&11, &20, &0);
        let (max_count, _, _) = find_max_persistence(&11, &20, &0);
        let duration = start.elapsed().as_millis();
        println!(
            "Time elapsed in milliseconds: {} \n Max_count: {}",
            duration, max_count
        );
        durations.push(duration);
        println!("Iteration: {}", i);
    }

    // calculate the average
    let mut sum = 0;
    for duration in &durations {
        sum += duration;
    }

    let average = sum / durations.len() as u128;
    println!("Average time elapsed in milliseconds: {}", average);
}
