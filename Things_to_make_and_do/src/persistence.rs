use num_bigint::BigUint;
use num_integer::Integer;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::thread;

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

pub fn find_persistence(number: &BigUint, count: u32) -> u32 {
    if number.to_string().len() == 1 {
        return count;
    }
    let product = multiply_digits(number);
    find_persistence(&product, count + 1)
}

fn generate_number(max_digits: &u32, min_digits: &u32) -> BigUint {
    let two = "2";
    let three = "3";
    let four = "4";

    let seven = "7";
    let eight = "8";
    let nine = "9";

    let mut rng = thread_rng();

    let mut digits = max_digits - 1;
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
    while number.len() < *min_digits as usize {
        number.push_str(nine);
    }

    number.parse::<BigUint>().unwrap()
}

pub fn find_max_persistence(goal: &u32, max_digits: &u32, min_digits: &u32) -> (u32, u32, usize) {
    let mut list_of_numbers = Vec::<BigUint>::new();
    let mut max_count = 0;
    let mut n_skip = 0;
    while &max_count < goal && n_skip < 100 {
        let number: BigUint = generate_number(max_digits, min_digits);
        if list_of_numbers.binary_search(&number).is_ok() {
            n_skip += 1;
            continue;
        }
        n_skip = 0;

        let result = find_persistence(&number, 0);
        if result > max_count {
            max_count = result;
        }
        list_of_numbers.push(number);
        list_of_numbers.sort();
    }

    (max_count, n_skip, list_of_numbers.len())
}

// this is not working greate yet, it does not seem faster than the single thread version
pub fn multi_thread_find_max_persistence(
    goal: &u32,
    max_digits: &u32,
    min_digits: &u32,
) -> (u32, u32, usize, usize) {
    let mut list_of_numbers = Vec::<BigUint>::new();
    let mut n_skip = 0;
    let mut max_count_outer = 0;
    let max_count = Arc::new(Mutex::new(0));
    let mut total_threads = 0;
    while &max_count_outer < goal && n_skip < 100 {
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
            total_threads += 1;

            handles.push(thread::spawn(move || {
                let result = find_persistence(&cloned_number, 0);
                let mut max_count_value = max_count.lock().unwrap();
                if result > *max_count_value {
                    *max_count_value = result;
                }
            }));

            list_of_numbers.push(number);
            list_of_numbers.sort();
        } else {
            for handle in handles {
                if handle.is_finished() {
                    handle.join().unwrap();
                }
            }
        }
        max_count_outer = *max_count.lock().unwrap();
    }

    (
        max_count_outer,
        n_skip,
        list_of_numbers.len(),
        total_threads,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_digits() {
        {
            let number = BigUint::from(2789u32);
            let product = multiply_digits(&number);
            assert_eq!(product, BigUint::from(1008u32));
        }
        {
            let number = BigUint::from(999u32);
            let product = multiply_digits(&number);
            assert_eq!(product, BigUint::from(729u32));
        }
        {
            let number = BigUint::from(1u32);
            let product = multiply_digits(&number);
            assert_eq!(product, BigUint::from(1u32));
        }
        {
            let number = BigUint::from(278099u32);
            let product = multiply_digits(&number);
            assert_eq!(product, BigUint::from(0u32));
        }
    }

    #[test]
    fn test_persistence() {
        {
            let number = "277777788888899".parse::<BigUint>().unwrap();
            let persistence = find_persistence(&number, 0);
            assert_eq!(persistence, 11);
        }
        {
            let number = "2789".parse::<BigUint>().unwrap();
            let persistence = find_persistence(&number, 0);
            assert_eq!(persistence, 2);
        }
        {
            let number = "909".parse::<BigUint>().unwrap();
            let persistence = find_persistence(&number, 0);
            assert_eq!(persistence, 1);
        }
        {
            let number = "6".parse::<BigUint>().unwrap();
            let persistence = find_persistence(&number, 0);
            assert_eq!(persistence, 0);
        }
    }
}
