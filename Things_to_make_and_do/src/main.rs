use rand::{thread_rng, Rng};
use num_bigint::BigUint;
use num_integer::Integer;
use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex};

fn multiply_digits_v1(number: &BigUint) -> BigUint {
    let mut product = BigUint::from(1u32);
    let mut temp = number.clone();
    while temp > BigUint::from(0u32) {
        let digit = temp.clone() % BigUint::from(10u32);
        temp = temp / BigUint::from(10u32);
        product *= digit;
    }
    product
}

fn multiply_digits(number: &BigUint) -> BigUint {
    let mut product = BigUint::from(1u32);
    let mut temp = number.clone();
    while temp > BigUint::from(0u32) {
        let (quotient, remainder) = temp.div_mod_floor(&BigUint::from(10u32));
        product *= remainder;
        temp = quotient;
    }
    product
}

fn persistence(number: &BigUint, count: u32 ) -> u32 {
    //println!("Number: {}", number);
    if number.to_string().len() == 1 {
        return count;
    }
    let product = multiply_digits(number);
    persistence(&product, count + 1)
}

fn generate_number(max_digits: u32, min_digits: u32) -> BigUint {
    
    let two = "2";
    let three = "3";
    let four = "4";

    let seven = "7";
    let eight = "8";
    let nine = "9";

    let mut rng = thread_rng();

    // Exclusive range
    let mut digits = max_digits-1;
    let number_of_sevens: u32 = rng.gen_range(0..digits);
    digits -= number_of_sevens;
    let number_of_eights: u32 = rng.gen_range(0..digits);
    digits -= number_of_eights;
    let number_of_nines: u32 = rng.gen_range(0..digits);
    
    let mut number = String::new();
    
    let temp = rng.gen_range(0..3);
    match temp {
        0 => number.push_str(two),
        1 => number.push_str(three),
        2 => number.push_str(four),
        _ => panic!("Invalid number"),
    }


    for _ in 0..number_of_sevens {
        number.push_str(seven);
    }
    for _ in 0..number_of_eights {
        number.push_str(eight);
    }
    for _ in 0..number_of_nines {
        number.push_str(nine);
    }
    while number.len() < min_digits as usize {
        number.push_str(nine);
    }
    
    number.parse::<BigUint>().unwrap()
}

fn find_max_persistence(goal: u32, max_digits: u32, min_digits: u32) {
    let mut list_of_numbers = Vec::<BigUint>::new();
    let mut max_count = 0;
    let mut n_skip = 0;
    while max_count < goal && n_skip < 100{
        let number: BigUint = generate_number(max_digits, min_digits);
        if list_of_numbers.binary_search(&number).is_ok() {
            n_skip += 1;
            continue;
        }
        n_skip = 0;
        
        //println!("{:?}", list_of_numbers);
        
        let result = persistence(&number, 0);
        if result > max_count {
            //println!("Number: {}, Persistence: {}", number, result);
            max_count = result;
        }
        list_of_numbers.push(number);
        list_of_numbers.sort();
        //println!("list_of_numbers length: {}", list_of_numbers.len());
    }

    //println!("Max count: {}", max_count);
    //println!("Number of skips: {}", n_skip);
    //println!("List of numbers length: {}", list_of_numbers.len());
}

fn multi_thread_find_max_persistence(goal: u32, max_digits: u32, min_digits: u32) {
    let mut list_of_numbers = Vec::<BigUint>::new();
    let mut n_skip = 0;
    let mut max_count_outer = 0;
    let max_count = Arc::new(Mutex::new(0));
    let total_threads = Arc::new(Mutex::new(0));
    while max_count_outer < goal && n_skip < 100{
        
        let mut handles = Vec::new();
        if handles.len() < 64 {
            let number: BigUint = generate_number(max_digits, min_digits);
            if list_of_numbers.binary_search(&number).is_ok() {
                n_skip += 1;
                continue;
            }
            n_skip = 0;
            
            let cloned_number = number.clone();

            let max_count = Arc::clone(&max_count);
            let total_threads = Arc::clone(&total_threads);

            handles.push(thread::spawn(move || {
                let mut total_threads = total_threads.lock().unwrap();
                *total_threads += 1;

                let result = persistence(&cloned_number, 0);
                let mut max_count_value = max_count.lock().unwrap();
                if result > *max_count_value {
                    //println!("Number: {}, Persistence: {}", number, result);
                    *max_count_value = result;
                }
            }));

            list_of_numbers.push(number);
            list_of_numbers.sort();
            //println!("list_of_numbers length: {}", list_of_numbers.len());
        }
        else {
            for handle in handles{
                if handle.is_finished() {
                    handle.join().unwrap();
                }
            }
        }
        max_count_outer = *max_count.lock().unwrap();
    }

    //println!("Max count: {}", max_count_outer);
    //println!("Number of skips: {}", n_skip);
    //println!("List of numbers length: {}", list_of_numbers.len());
    //println!("Total threads: {}", *total_threads.lock().unwrap());
}

fn main() {
    
    //multi_thread_find_max_persistence(11, 20, 0);


    //let number = BigUint::from(277777788888899u128);
    //let result = persistence(&number, 0);
    //println!("Number: {}, Persistence: {}", number, result);
    let mut durations = Vec::<u128>::new();
    for i in 0..1000{
        let start = Instant::now();
        //multi_thread_find_max_persistence(11, 20, 0);
        find_max_persistence(11, 20, 0);
        let duration = start.elapsed().as_millis();
        println!("Time elapsed in milliseconds: {}", duration);
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

    //for _ in 0..100{
    //    let n = generate_number(10,5);
    //    println!("Number: {}", n);
    //}
    //let product = multiply_digits(&n);
    //println!("Product: {}", product);
    //let result = persistence(&n, 0);
    //println!("Persistence: {}", result);

}
