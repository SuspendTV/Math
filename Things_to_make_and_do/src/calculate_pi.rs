use num_bigfloat::BigFloat;

pub fn calculate_pi_copilot(iterations: BigFloat) -> BigFloat {
    let mut pi = BigFloat::from(0.0);
    let four = BigFloat::from(4.0);
    let one = BigFloat::from(1.0);
    let mut i = BigFloat::from(0.0);
    let two = BigFloat::from(2.0);

    while i < iterations {
        let temp = &four / (&two * &i + &one);
        if i % 2.0 == 0.0 {
            pi = &pi + &temp;
        } else {
            pi = &pi - &temp;
        }
        i = &i + &one;
    }
    pi
}

pub fn calculate_pi(iterations: BigFloat) -> BigFloat {
    //pi = 0
    //for i in range(1000000):
    //    pi += 4 * (1 - (i % 2) * 2) / (2 * i + 1)

    let mut pi = BigFloat::new();
    let one = BigFloat::from(1u8);
    let two = BigFloat::from(2u8);
    let four = BigFloat::from(4u8);
    let mut i = BigFloat::new();
    while i < iterations {
        pi += four * (one - (i.rem(&two)) * two) / (two * i + one);
        if i.rem(&BigFloat::from(100000u32)) == BigFloat::from(0u8) {
            print!("\ri = {}", i);
            print!("\tpi = {}", pi);
        }
        i += one;
    }
    println!("");

    pi
}
