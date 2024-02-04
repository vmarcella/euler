use core::time;

const NUMBER: u64 = 600_851_475_143;

fn brute_force() {
    let mut primes = std::collections::HashSet::new();
    let mut factors = std::collections::HashSet::new();

    // The search space is constrained to the square root of the original
    // number. Let's say the prime number is `c` and the factors that make up
    // are `a` and `b`, then neither `a` or `b` cannot be greater than `sqrt(c)`
    // without resulting in `a * b > c`.
    let potential_prime_factors = f64::sqrt(NUMBER as f64) as u64;
    for num in (0..potential_prime_factors) {
        if num % 2 == 0 {
            continue;
        }

        let mut is_prime = true;
        for prime in primes.iter() {
            if *prime == 1 {
                continue;
            }

            if num % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            println!("Found prime: {}", num);
            primes.insert(num);

            if NUMBER % num == 0 {
                factors.insert(num);
            }
        }
    }

    let max = factors.iter().max().expect("No max value");
    println!("Max prime factor is {}", max,);
}

fn main() {
    let start = std::time::Instant::now();
    brute_force();
    let stop = std::time::Instant::now();
    println!("Computation took: {} seconds", (stop - start).as_secs());
}
