fn main() {
    let mut sum = naive();
    println!("Sum: {}", sum);

    sum = complex_for_no_reason();
    println!("Sum: {}", sum);
}

/// Simple solution that just iterates through all numbers up to 1000 and checks
/// if they are divisible by 3 or 5.
fn naive() -> u32 {
    let mut sum = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    return sum;
}

/// This solution starts at 5, steps by 5, and then calculates the multiples of 3 below and above
/// the current number. It then adds the current number, the multiple below, and the multiple above
/// if the current number is also not a multiple of 3.
fn complex_for_no_reason() -> u32 {
    let mut sum = 0;
    for i in (5..=1000).step_by(5) {
        // Exclude multiples of 3 when i is both a multiple of 3 & 5
        if i % 3 == 0 {
            sum += i
        } else {
            // compute the multiples of 3 "below" and "above" i
            let multiple_below = i - (i % 3);
            let multiple_above = i + (3 - (i % 3));
            sum += i + multiple_below + multiple_above;
        }
    }

    // The sum is off by 2002 because we use the inclusive operator in the for
    // loop which ends up adding 1000 (i) & 1002 (multiple_above) to the sum.
    return sum - 2002;
}
